mod root;
mod users;

use self::{root::root, users::ApiUsers};
// use self::users::create_user;
use crate::AppResult;
use axum::{
    routing::{get, post},
    Router,
};

pub struct Api;

impl Api {
    pub async fn build() -> AppResult<Router> {
        let router = Router::new()
            .route("/", get(root))
            .route("/users", post(ApiUsers::create));
        Ok(router)
    }
}
