/*!
 * A rust library for interacting with the Airtable API.
 *
 * For more information, the Airtable API is documented at [airtable.com/api](https://airtable.com/api).
 *
 * Example:
 *
 * ```ignore
 * use airtable_api::{Airtable, Record};
 * use serde::{Deserialize, Serialize};
 *
 * async fn get_records() {
 *     // Initialize the Airtable client.
 *     let airtable = Airtable::new_from_env();
 *
 *     // Get the current records from a table.
 *     let mut records: Vec<Record<SomeFormat>> = airtable
 *         .list_records(
 *             "Table Name",
 *             "Grid view",
 *             vec!["the", "fields", "you", "want", "to", "return"],
 *         )
 *         .await
 *         .unwrap();
 *
 *     // Iterate over the records.
 *     for (i, record) in records.clone().iter().enumerate() {
 *         println!("{} - {:?}", i, record);
 *     }
 * }
 *
 * #[derive(Debug, Clone, Serialize, Deserialize)]
 * pub struct SomeFormat {
 *     pub x: bool,
 * }
 * ```
 */
#![allow(clippy::field_reassign_with_default)]
use std::{fmt, fmt::Debug};

use anyhow::Result;
use chrono::{offset::Utc, DateTime};
use schemars::JsonSchema;
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APICall<T> {
    /// If there are more records, the response will contain an
    /// offset. To fetch the next page of records, include offset
    /// in the next request's parameters.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub offset: String,
    /// The current page number of returned records.
    pub records: Vec<Record<T>>,
    /// The Airtable API will perform best-effort automatic data conversion
    /// from string values if the typecast parameter is passed in. Automatic
    /// conversion is disabled by default to ensure data integrity, but it may
    /// be helpful for integrating with 3rd party data sources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typecast: Option<bool>,
}

/// An Airtable record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record<T> {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    pub fields: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<DateTime<Utc>>,
}

/// An airtable user.
// #[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[derive(Debug, Default, Clone, Serialize, JsonSchema, Deserialize)]
pub struct User {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub email: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
}

enum UserField {
    Id,
    Email,
    Name,
}

const USERFIELDS: &[&str] = &["id", "email", "name"];

impl<'de> Deserialize<'de> for UserField {
    fn deserialize<D>(deserializer: D) -> Result<UserField, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UserFieldVisitor;

        impl<'de> Visitor<'de> for UserFieldVisitor {
            type Value = UserField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`id` `email` or `name`")
            }

            fn visit_str<E>(self, value: &str) -> Result<UserField, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "id" => Ok(UserField::Id),
                    "email" => Ok(UserField::Email),
                    "name" => Ok(UserField::Name),
                    _ => Err(serde::de::Error::unknown_field(value, USERFIELDS)),
                }
            }
        }

        deserializer.deserialize_identifier(UserFieldVisitor)
    }
}

struct UserVisitor;

impl<'de> Visitor<'de> for UserVisitor {
    type Value = User;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct User")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<User, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let id = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let email = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        let name = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
        Ok(User { id, email, name })
    }

    fn visit_map<V>(self, mut map: V) -> Result<User, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut id = None;
        let mut email = None;
        let mut name = None;
        while let Some(key) = map.next_key()? {
            match key {
                UserField::Id => {
                    if id.is_some() {
                        return Err(serde::de::Error::duplicate_field("id"));
                    }
                    id = Some(map.next_value()?);
                }
                UserField::Email => {
                    if email.is_some() {
                        return Err(serde::de::Error::duplicate_field("email"));
                    }
                    email = Some(map.next_value()?);
                }
                UserField::Name => {
                    if name.is_some() {
                        return Err(serde::de::Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value()?);
                }
            }
        }
        let id = id.unwrap_or_default();
        let email = email.ok_or_else(|| serde::de::Error::missing_field("email"))?;
        let name = name.unwrap_or_default();
        Ok(User { id, email, name })
    }
}

struct UsersVisitor;

impl<'de> Visitor<'de> for UsersVisitor {
    type Value = Vec<User>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a very special vector")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
        let mut users: Vec<User> = Default::default();

        // While there are entries remaining in the input, add them
        // into our vector.
        while let Some(user) = access.next_element::<User>()? {
            users.push(user);
        }

        Ok(users)
    }
}

/// The response returned from listing users.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct UsersResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<User>,
}

/// The response returned from deleting a user.
/// FROM: https://airtable.com/api/enterprise#enterpriseAccountUserDeleteUserByEmail
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteUserResponse {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        rename = "deletedUsers"
    )]
    pub deleted_users: Vec<User>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorResponse>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub email: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AttachmentShort {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Attachment {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub url: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub filename: String,
    #[serde(default)]
    pub size: i64,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub type_: String,
    #[serde(default)]
    pub thumbnails: Thumbnails,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Thumbnails {
    #[serde(default)]
    pub small: Full,
    #[serde(default)]
    pub large: Full,
    #[serde(default)]
    pub full: Full,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Full {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub url: String,
    #[serde(default)]
    pub width: i64,
    #[serde(default)]
    pub height: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NewCollaborator {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub collaborators: Vec<Collaborator>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Collaborator {
    #[serde(default)]
    pub user: User,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize",
        rename = "permissionLevel"
    )]
    pub permission_level: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EnterpriseUsersResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<EnterpriseUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseUser {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub state: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub email: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub name: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "lastActivityTime"
    )]
    pub last_activity_time: Option<DateTime<Utc>>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize",
        rename = "invitedToAirtableByUserId"
    )]
    pub invited_to_airtable_by_user_id: String,
    #[serde(rename = "createdTime")]
    pub created_time: DateTime<Utc>,
    #[serde(default)]
    pub collaborations: Collaborations,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Collaborations {
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        rename = "workspaceCollaborations"
    )]
    pub workspace_collaborations: Vec<Collaboration>,
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        rename = "baseCollaborations"
    )]
    pub base_collaborations: Vec<Collaboration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collaboration {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize",
        rename = "baseId"
    )]
    pub base_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize",
        rename = "permissionLevel"
    )]
    pub permission_level: String,
    #[serde(rename = "createdTime")]
    pub created_time: DateTime<Utc>,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize",
        rename = "grantedByUserId"
    )]
    pub granted_by_user_id: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize",
        rename = "workspaceId"
    )]
    pub workspace_id: String,
}

struct AttachmentsVisitor;

impl<'de> Visitor<'de> for AttachmentsVisitor {
    type Value = Vec<Attachment>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a very special vector")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
        let mut attachments: Vec<Attachment> = Default::default();

        // While there are entries remaining in the input, add them
        // into our vector.
        while let Some(attachment) = access.next_element::<Attachment>()? {
            attachments.push(attachment);
        }

        Ok(attachments)
    }
}

pub mod user_format_as_array_of_strings {
    use serde::{self, ser::SerializeSeq, Deserializer, Serializer};

    use super::{User, UsersVisitor};

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(array: &[String], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Make our array of Airtable user objects.
        let mut seq = serializer.serialize_seq(Some(array.len())).unwrap();
        for e in array {
            seq.serialize_element(&User {
                id: Default::default(),
                email: e.to_string(),
                name: Default::default(),
            })
            .unwrap();
        }
        seq.end()
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let airtable_users = deserializer.deserialize_seq(UsersVisitor {}).unwrap();

        let mut users: Vec<String> = Default::default();
        for a in airtable_users {
            users.push(a.email.to_string());
        }

        Ok(users)
    }
}

pub mod user_format_as_string {
    use serde::{self, ser::SerializeStruct, Deserializer, Serializer};

    use super::{UserVisitor, USERFIELDS};

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(email: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("User", 1)?;
        state.serialize_field("email", &email)?;
        state.end()
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let user = deserializer
            .deserialize_struct("User", USERFIELDS, UserVisitor)
            .unwrap();
        Ok(user.email)
    }
}

pub mod attachment_format_as_array_of_strings {
    use serde::{self, ser::SerializeSeq, Deserializer, Serializer};

    use super::{AttachmentShort, AttachmentsVisitor};

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(array: &[String], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Make our array of Airtable attachment objects.
        let mut seq = serializer.serialize_seq(Some(array.len())).unwrap();
        for e in array {
            let mut attachment: AttachmentShort = Default::default();
            attachment.url = e.to_string();
            seq.serialize_element(&attachment).unwrap();
        }
        seq.end()
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let airtable_attachments = deserializer.deserialize_seq(AttachmentsVisitor {}).unwrap();

        let mut attachments: Vec<String> = Default::default();
        for a in airtable_attachments {
            attachments.push(a.url.to_string());
        }

        Ok(attachments)
    }
}

pub mod attachment_format_as_string {
    use serde::{self, ser::SerializeSeq, Deserializer, Serializer};

    use super::{Attachment, AttachmentsVisitor};

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(url: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Make our array of Airtable attachment objects.
        let mut seq = serializer.serialize_seq(Some(1)).unwrap();
        let mut attachment: Attachment = Default::default();
        attachment.url = url.to_string();
        seq.serialize_element(&attachment).unwrap();
        seq.end()
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let airtable_attachments = deserializer.deserialize_seq(AttachmentsVisitor {}).unwrap();
        let mut url = String::new();
        if !airtable_attachments.is_empty() {
            url = airtable_attachments[0].url.to_string();
        }
        Ok(url)
    }
}

/// An airtable barcode.
// #[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[derive(Debug, Default, Clone, Serialize, JsonSchema, Deserialize)]
pub struct Barcode {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub text: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "type")]
    pub type_: String,
}

struct BarcodeVisitor;

impl<'de> Visitor<'de> for BarcodeVisitor {
    type Value = Barcode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Barcode")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Barcode, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let text = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let type_ = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        Ok(Barcode { text, type_ })
    }

    fn visit_map<V>(self, mut map: V) -> Result<Barcode, V::Error>
    where
        V: MapAccess<'de>,
    {
        let mut text = None;
        let mut type_ = None;
        while let Some(key) = map.next_key()? {
            match key {
                BarcodeField::Text => {
                    if text.is_some() {
                        return Err(serde::de::Error::duplicate_field("text"));
                    }
                    text = Some(map.next_value()?);
                }
                BarcodeField::Type => {
                    if type_.is_some() {
                        return Err(serde::de::Error::duplicate_field("type"));
                    }
                    type_ = Some(map.next_value()?);
                }
            }
        }
        let text = text.ok_or_else(|| serde::de::Error::missing_field("text"))?;
        let type_ = type_.ok_or_else(|| serde::de::Error::missing_field("type"))?;
        Ok(Barcode { text, type_ })
    }
}

enum BarcodeField {
    Text,
    Type,
}

const BARCODEFIELDS: &[&str] = &["text", "type"];

impl<'de> Deserialize<'de> for BarcodeField {
    fn deserialize<D>(deserializer: D) -> Result<BarcodeField, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BarcodeFieldVisitor;

        impl<'de> Visitor<'de> for BarcodeFieldVisitor {
            type Value = BarcodeField;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("`text` or `type`")
            }

            fn visit_str<E>(self, value: &str) -> Result<BarcodeField, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "text" => Ok(BarcodeField::Text),
                    "type" => Ok(BarcodeField::Type),
                    _ => Err(serde::de::Error::unknown_field(value, BARCODEFIELDS)),
                }
            }
        }

        deserializer.deserialize_identifier(BarcodeFieldVisitor)
    }
}

pub mod barcode_format_as_string {
    use serde::{self, ser::SerializeStruct, Deserializer, Serializer};

    use super::{BarcodeVisitor, BARCODEFIELDS};

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(text: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Barcode", 1)?;
        state.serialize_field("text", &text)?;
        // This needs to be code39 or upce.
        state.serialize_field("type", "code39")?;
        state.end()
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let barcode = deserializer
            .deserialize_struct("Barcode", BARCODEFIELDS, BarcodeVisitor)
            .unwrap();
        Ok(barcode.text)
    }
}

pub mod deserialize_null_string {
    use serde::{self, Deserialize, Deserializer};

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer).unwrap_or_default();

        Ok(s)
    }
}
