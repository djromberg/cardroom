mod act_on_table;
mod create_tournament;

mod do_stuff;

mod register_player;

use crate::application::{ApplicationError, AuthError, AuthInfo, AuthRole, RepositoryError};

use axum::{http::StatusCode, response::{IntoResponse, Response}};

pub use act_on_table::*;
pub use create_tournament::*;

pub use do_stuff::*;

pub use register_player::*;

use axum_keycloak_auth::decode::KeycloakToken;

use uuid::Uuid;


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


fn create_auth_info(token: KeycloakToken<AuthRole>) -> Result<AuthInfo, AuthError> {
    let account_id = Uuid::parse_str(&token.subject).map_err(|_| AuthError::InvalidAccountId)?;
    let roles = token.roles.iter().map(|kcr| kcr.role().clone()).collect();
    let given_name = token.extra.profile.given_name.unwrap_or("Anonymous".to_string());
    Ok(AuthInfo::new(account_id, given_name, roles))
}
