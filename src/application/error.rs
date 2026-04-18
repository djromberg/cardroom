use super::traits::RepositoryError;
use crate::domain::DomainError;

use thiserror::Error;


#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    DomainError(#[from] DomainError),
    #[error(transparent)]
    RepositoryError(#[from] RepositoryError),
}
