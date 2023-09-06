use std::fmt::Debug;

#[derive(Debug)]
pub(crate) struct Users(Vec<User>);

impl Users {
    pub(crate) fn first(&self) -> &User {
        &self.0[0]
    }
}

impl From<Vec<User>> for Users {
    fn from(vec: Vec<User>) -> Self {
        Self(vec)
    }
}
impl Default for Users {
    fn default() -> Self {
        vec![
            User {
                oid: "1".to_string().into(),
                internals: Default::default(),
            },
            User {
                oid: "2".to_string().into(),
                internals: Default::default(),
            },
            User {
                oid: "3".to_string().into(),
                internals: Default::default(),
            },
        ]
        .into()
    }
}

pub(crate) struct User {
    oid: UserId,
    internals: Internals,
}

impl Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User").field("oid", &self.oid).finish()
    }
}

#[derive(Debug)]
pub(crate) struct UserId(String);

impl From<String> for UserId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Default)]
pub(crate) struct Internals;
