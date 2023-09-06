use axum::http::StatusCode;
use hyper::Error as HyperError;
use std::fmt::Display;
use std::net::AddrParseError;
use thiserror::Error;
pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    InvalidSizeForNome,
    InvalidSizeForApelido,
    AddrParseError(#[from] AddrParseError),
    HyperError(#[from] HyperError),
    // Custom(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<AppError> for StatusCode {
    fn from(error: AppError) -> Self {
        match error {
            AppError::InvalidSizeForApelido | AppError::InvalidSizeForNome => {
                Self::UNPROCESSABLE_ENTITY
            }
            AppError::AddrParseError(_) => Self::BAD_REQUEST,
            AppError::HyperError(_) => Self::BAD_REQUEST,
        }
    }
}
