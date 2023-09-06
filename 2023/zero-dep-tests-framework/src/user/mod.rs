pub use self::user_alias::UserAlias;

#[cfg(test)]
mod tests;
mod user_alias;


pub struct User {
    user_alias: UserAlias,
}


