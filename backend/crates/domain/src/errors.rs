//! Domain errors

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

impl From<validator::ValidationErrors> for DomainError {
    fn from(err: validator::ValidationErrors) -> Self {
        DomainError::Validation(err.to_string())
    }
}
