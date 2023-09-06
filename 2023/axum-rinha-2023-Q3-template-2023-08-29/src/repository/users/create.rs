use serde::Deserialize;
use uuid::Uuid;

use crate::{
    model::{Apelido, Nome, SizeValidator, User},
    result::{AppError, AppResult},
};

use super::RepoUsers;

impl RepoUsers {
    pub async fn create(user_to_create: UserToCreate) -> AppResult<User> {
        let user = User::try_from(user_to_create)?;
        // bail!("TODO")
        Ok(user)
    }
}

#[derive(Deserialize)]
pub struct UserToCreate {
    apelido: Apelido,
    nome: Nome,
}

impl TryFrom<UserToCreate> for User {
    type Error = AppError;

    fn try_from(value: UserToCreate) -> Result<Self, Self::Error> {
        let UserToCreate { apelido, nome } = value;
        apelido.validate_max_length()?;
        nome.validate_max_length()?;
        let user = User {
            id: Uuid::new_v4(),
            apelido,
            nome,
        };
        Ok(user)
    }
}
