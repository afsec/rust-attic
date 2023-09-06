use super::{user_alias::UserAlias, User};
#[cfg(test)]
#[derive(Debug)]
pub struct UserTest {
    user_alias: Option<UserAlias>,
}

#[cfg(test)]
impl Default for UserTest {
    fn default() -> Self {
        Default::default()
    }
}
#[cfg(test)]
impl From<User> for UserTest {
    fn from(value: User) -> Self {
        let User { user_alias } = value;
        Self { user_alias: Some(user_alias) }
    }
}

#[test]
fn check_hostname() {
    let user_alias = UserAlias::default();
    dbg!(user_alias);
}
