use crate::user::UserAlias;



pub mod traits_for_test;
pub mod user;

fn main() {
    let user_alias = UserAlias::default();
    dbg!(user_alias);
}
