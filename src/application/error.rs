use crate::application::AuthError;
use crate::domain::DomainError;

use thiserror::Error;


#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Resource not found")]
    ResourceNotFound,
    #[error("Internal error")]
    InternalError,
}


#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    DomainError(#[from] DomainError),
    #[error(transparent)]
    RepositoryError(#[from] RepositoryError),
}
