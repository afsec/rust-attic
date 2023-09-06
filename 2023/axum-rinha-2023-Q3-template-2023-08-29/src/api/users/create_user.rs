use crate::{
    model::User,
    repository::{RepoUsers, UserToCreate},
};
use axum::{http::StatusCode, Json};

use super::ApiUsers;

impl ApiUsers {
    pub async fn create(
        Json(user_to_create): Json<UserToCreate>,
    ) -> (StatusCode, Json<Option<User>>) {
        let saved_user = match RepoUsers::create(user_to_create).await {
            Ok(user) => user,
            Err(error) => {
                tracing::error!("{error}");
                return (error.into(), Json(None));
            }
        };
    
        (StatusCode::CREATED, Json(Some(saved_user)))
    }
}

