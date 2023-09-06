use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TrickError {
    NoArgs,
    InvalidCommand,
}

impl From<TrickError> for i32 {
    fn from(err: TrickError) -> Self {
        match err {
            TrickError::NoArgs => 1,
            TrickError::InvalidCommand => 2,
        }
    }
}

impl fmt::Display for TrickError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for TrickError {}
