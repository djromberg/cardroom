mod act_on_table;
mod create_tournament;

use crate::application::{ApplicationError, AuthError, RepositoryError};

use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub use act_on_table::*;
pub use create_tournament::*;


impl IntoResponse for ApplicationError {
    fn into_response(self) -> Response {
        match self {
            ApplicationError::DomainError(error) => build_response(StatusCode::BAD_REQUEST, error.to_string()),
            ApplicationError::RepositoryError(error) => {
                match error {
                    RepositoryError::ResourceNotFound => build_response(StatusCode::NOT_FOUND, error.to_string()),
                    RepositoryError::InternalError => build_response(StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
                }
            },
            ApplicationError::AuthError(error) => {
                match error {
                    AuthError::InvalidAccountId => build_response(StatusCode::BAD_REQUEST, error.to_string()),
                    AuthError::PermissionDenied { .. } => build_response(StatusCode::FORBIDDEN, error.to_string()),
                }
            }
        }
    }
}


fn build_response(status_code: axum::http::StatusCode, message: String) -> Response {
    Response::builder().status(status_code).body(message.into()).unwrap()
}
