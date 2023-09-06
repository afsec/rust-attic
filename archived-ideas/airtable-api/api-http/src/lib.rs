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

use airtable_api_types::{
    APICall, Collaborator, DeleteUserResponse, EnterpriseUser, EnterpriseUsersResponse,
    NewCollaborator, Record, User, UsersResponse,
};
use anyhow::{bail, Result};
use reqwest::{header, Method, Request, StatusCode, Url};
use serde::{de::DeserializeOwned, Serialize};
use std::env;

/// Endpoint for the Airtable API.
const ENDPOINT: &str = "https://api.airtable.com/v0/";

/// Entrypoint for interacting with the Airtable API.
pub struct Airtable {
    key: String,
    base_id: String,
    enterprise_account_id: String,
    client: reqwest_middleware::ClientWithMiddleware,
}

/// Get the API key from the AIRTABLE_API_KEY env variable.
pub fn api_key_from_env() -> String {
    env::var("AIRTABLE_API_KEY").unwrap_or_default()
}

impl Airtable {
    /// Create a new Airtable client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API Key and Base ID your requests will work.
    /// You can leave the Enterprise Account ID empty if you are not using the
    /// Enterprise API features.
    pub fn new<K, B, E>(key: K, base_id: B, enterprise_account_id: E) -> Self
    where
        K: ToString,
        B: ToString,
        E: ToString,
    {
        let http = reqwest::Client::builder().build();
        match http {
            Ok(c) => {
                let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder()
                    .build_with_max_retries(3);
                let client = reqwest_middleware::ClientBuilder::new(c)
                    // Trace HTTP requests. See the tracing crate to make use of these traces.
                    .with(reqwest_tracing::TracingMiddleware)
                    // Retry failed requests.
                    .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(
                        retry_policy,
                    ))
                    .build();

                Self {
                    key: key.to_string(),
                    base_id: base_id.to_string(),
                    enterprise_account_id: enterprise_account_id.to_string(),

                    client,
                }
            }
            Err(e) => panic!("creating client failed: {:?}", e),
        }
        // Self {
        //     key: key.to_string(),
        //     base_id: base_id.to_string(),
        //     enterprise_account_id: enterprise_account_id.to_string(),
        // }
    }

    /// Create a new Airtable client struct from environment variables. It
    /// takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API Key and Base ID your requests will work.
    pub fn new_from_env() -> Self {
        let base_id = env::var("AIRTABLE_BASE_ID").unwrap_or_default();
        let enterprise_account_id = env::var("AIRTABLE_ENTERPRISE_ACCOUNT_ID").unwrap_or_default();

        Airtable::new(api_key_from_env(), base_id, enterprise_account_id)
    }

    /// Get the currently set API key.
    pub fn get_key(&self) -> &str {
        &self.key
    }

    fn request<B>(
        &self,
        method: Method,
        path: String,
        body: B,
        query: Option<Vec<(&str, String)>>,
    ) -> Result<Request>
    where
        B: Serialize,
    {
        let base = Url::parse(ENDPOINT)?;
        let url = base.join(&(self.base_id.to_string() + "/" + &path))?;

        let bt = format!("Bearer {}", self.key);
        let bearer = header::HeaderValue::from_str(&bt)?;

        // Set the default headers.
        let mut headers = header::HeaderMap::new();
        headers.append(header::AUTHORIZATION, bearer);
        headers.append(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let mut rb = self.client.request(method.clone(), url).headers(headers);

        match query {
            None => (),
            Some(val) => {
                rb = rb.query(&val);
            }
        }

        // Add the body, this is to ensure our GET and DELETE calls succeed.
        if method != Method::GET && method != Method::DELETE {
            rb = rb.json(&body);
        }

        // Build the request.
        Ok(rb.build()?)
    }

    /// List records in a table for a particular view.
    pub async fn list_records<T: DeserializeOwned>(
        &self,
        table: &str,
        view: &str,
        fields: Vec<&str>,
    ) -> Result<Vec<Record<T>>> {
        let mut params = vec![("pageSize", "100".to_string()), ("view", view.to_string())];
        for field in fields {
            params.push(("fields[]", field.to_string()));
        }

        // Build the request.
        let mut request = self.request(Method::GET, table.to_string(), (), Some(params))?;

        let mut resp = self.client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                bail!("status code: {}, body: {}", s, resp.text().await?);
            }
        };

        // Try to deserialize the response.
        let mut r: APICall<T> = resp.json().await?;

        let mut records = r.records;

        let mut offset = r.offset;

        // Paginate if we should.
        // TODO: make this more DRY
        while !offset.is_empty() {
            request = self.request(
                Method::GET,
                table.to_string(),
                (),
                Some(vec![
                    ("pageSize", "100".to_string()),
                    ("view", view.to_string()),
                    ("offset", offset),
                ]),
            )?;

            resp = self.client.execute(request).await?;
            match resp.status() {
                StatusCode::OK => (),
                s => {
                    bail!("status code: {}, body: {}", s, resp.text().await?);
                }
            };

            // Try to deserialize the response.
            r = resp.json().await?;

            records.append(&mut r.records);

            offset = r.offset;
        }

        Ok(records)
    }

    /// Get record from a table.
    pub async fn get_record<T: DeserializeOwned>(
        &self,
        table: &str,
        record_id: &str,
    ) -> Result<Record<T>> {
        // Build the request.
        let request = self.request(Method::GET, format!("{}/{}", table, record_id), (), None)?;

        let resp = self.client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                bail!("status code: {}, body: {}", s, resp.text().await?);
            }
        };

        // Try to deserialize the response.
        let record: Record<T> = resp.json().await?;

        Ok(record)
    }

    /// Delete record from a table.
    pub async fn delete_record(&self, table: &str, record_id: &str) -> Result<()> {
        // Build the request.
        let request = self.request(
            Method::DELETE,
            table.to_string(),
            (),
            Some(vec![("records[]", record_id.to_string())]),
        )?;

        let resp = self.client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                bail!("status code: {}, body: {}", s, resp.text().await?);
            }
        };

        Ok(())
    }

    /// Bulk create records in a table.
    ///
    /// Due to limitations on the Airtable API, you can only bulk create 10
    /// records at a time.
    pub async fn create_records<T: Serialize + DeserializeOwned>(
        &self,
        table: &str,
        records: Vec<Record<T>>,
    ) -> Result<Vec<Record<T>>> {
        // Build the request.
        let request = self.request(
            Method::POST,
            table.to_string(),
            APICall {
                records,
                offset: "".to_string(),
                typecast: Some(true),
            },
            None,
        )?;

        let resp = self.client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                bail!("status code: {}, body: {}", s, resp.text().await?);
            }
        };

        // Try to deserialize the response.
        let r: APICall<T> = resp.json().await?;

        Ok(r.records)
    }

    /// Bulk update records in a table.
    ///
    /// Due to limitations on the Airtable API, you can only bulk update 10
    /// records at a time.
    pub async fn update_records<T: Serialize + DeserializeOwned>(
        &self,
        table: &str,
        records: Vec<Record<T>>,
    ) -> Result<Vec<Record<T>>> {
        // Build the request.
        let request = self.request(
            Method::PATCH,
            table.to_string(),
            APICall {
                records,
                offset: "".to_string(),
                typecast: Some(true),
            },
            None,
        )?;

        let resp = self.client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                bail!("status code: {}, body: {}", s, resp.text().await?);
            }
        };

        // Try to deserialize the response.
        match resp.json::<APICall<T>>().await {
            Ok(v) => Ok(v.records),
            Err(_) => {
                // This might fail. On a faiture just return an empty vector.
                Ok(vec![])
            }
        }
    }

    /// List users.
    /// This is for an enterprise admin to do only.
    /// FROM: https://airtable.com/api/enterprise
    pub async fn list_users(&self) -> Result<Vec<User>> {
        if self.enterprise_account_id.is_empty() {
            // Return an error early.
            bail!("An enterprise account id is required.");
        }

        // Build the request.
        let request = self.request(
            Method::GET,
            format!(
                "v0/meta/enterpriseAccounts/{}/users",
                self.enterprise_account_id
            ),
            (),
            Some(vec![("state", "provisioned".to_string())]),
        )?;

        let resp = self.client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                bail!("status code: {}, body: {}", s, resp.text().await?);
            }
        };

        // Try to deserialize the response.
        let result: UsersResponse = resp.json().await?;

        Ok(result.users)
    }

    /// Get an enterprise user.
    /// This is for an enterprise admin to do only.
    /// FROM: https://airtable.com/api/enterprise#enterpriseAccountUserGetInformationByEmail
    /// Permission level can be: owner | create | edit | comment | read
    pub async fn get_enterprise_user(&self, email: &str) -> Result<EnterpriseUser> {
        if self.enterprise_account_id.is_empty() {
            // Return an error early.
            bail!("An enterprise account id is required.");
        }

        // Build the request.
        let request = self.request(
            Method::GET,
            format!(
                "v0/meta/enterpriseAccounts/{}/users",
                self.enterprise_account_id
            ),
            (),
            Some(vec![
                ("email", email.to_string()),
                ("include", "collaborations".to_string()),
            ]),
        )?;

        let resp = self.client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                bail!("status code: {}, body: {}", s, resp.text().await?);
            }
        };

        let r: EnterpriseUsersResponse = resp.json().await?;

        if r.users.is_empty() {
            bail!("no user was returned");
        }

        Ok(r.users.get(0).unwrap().clone())
    }

    /// Add a collaborator to a workspace.
    /// This is for an enterprise admin to do only.
    /// FROM: https://airtable.com/api/enterprise#enterpriseWorkspaceAddCollaborator
    /// Permission level can be: owner | create | edit | comment | read
    pub async fn add_collaborator_to_workspace(
        &self,
        workspace_id: &str,
        user_id: &str,
        permission_level: &str,
    ) -> Result<()> {
        if self.enterprise_account_id.is_empty() {
            // Return an error early.
            bail!("An enterprise account id is required.");
        }

        // Build the request.
        let request = self.request(
            Method::POST,
            format!("v0/meta/workspaces/{}/collaborators", workspace_id),
            NewCollaborator {
                collaborators: vec![Collaborator {
                    user: User {
                        id: user_id.to_string(),
                        email: Default::default(),
                        name: Default::default(),
                    },
                    permission_level: permission_level.to_string(),
                }],
            },
            None,
        )?;

        let resp = self.client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                bail!("status code: {}, body: {}", s, resp.text().await?);
            }
        };

        Ok(())
    }

    /// Delete internal user by email.
    /// This is for an enterprise admin to do only.
    /// The user must be an internal user, meaning they have an email with the company domain.
    /// FROM: https://airtable.com/api/enterprise#enterpriseAccountUserDeleteUserByEmail
    pub async fn delete_internal_user_by_email(&self, email: &str) -> Result<()> {
        if self.enterprise_account_id.is_empty() {
            // Return an error early.
            bail!("An enterprise account id is required.");
        }

        // Build the request.
        let request = self.request(
            Method::DELETE,
            format!(
                "v0/meta/enterpriseAccounts/{}/users",
                self.enterprise_account_id
            ),
            (),
            Some(vec![("email", email.to_string())]),
        )?;

        let resp = self.client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                bail!("status code: {}, body: {}", s, resp.text().await?);
            }
        };

        // Try to deserialize the response.
        let result: DeleteUserResponse = resp.json().await?;
        if !result.errors.is_empty() {
            bail!("body: {:?}", result);
        }

        Ok(())
    }
}
