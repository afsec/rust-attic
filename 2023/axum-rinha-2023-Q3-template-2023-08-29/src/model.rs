use crate::result::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: Maybe could be an Extractor pattern
pub trait SizeValidator {
    const MAX_LENGTH: usize;
    fn validate_max_length(&self) -> AppResult<()>;
}

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub apelido: Apelido,
    pub nome: Nome,
}

/// Obrigatório, string de até 32 caracteres
#[derive(Serialize, Deserialize)]
pub struct Apelido(String);

impl SizeValidator for Apelido {
    const MAX_LENGTH: usize = 7;

    fn validate_max_length(&self) -> AppResult<()> {
        if self.0.chars().count() > Self::MAX_LENGTH {
            return Err(AppError::InvalidSizeForApelido);
        }
        Ok(())
    }
}

/// Obrigatório, string de até 100 caracteres
#[derive(Serialize, Deserialize)]
pub struct Nome(String);

impl SizeValidator for Nome {
    const MAX_LENGTH: usize = 12;

    fn validate_max_length(&self) -> AppResult<()> {
        if self.0.chars().collect::<Vec<_>>().len() > Self::MAX_LENGTH {
            return Err(AppError::InvalidSizeForNome);
        }
        Ok(())
    }
}
