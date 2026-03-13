mod create_tournament;
mod find_tournaments;
mod join_tournament;
mod observe_table;

use crate::application::AuthError;
use crate::application::AuthInfo;
use crate::application::AuthRole;
use crate::domain::LoadTournamentError;
use crate::domain::QueryTournamentsError;
use crate::domain::SaveTournamentError;
use crate::domain::TournamentError;

use axum::response::IntoResponse;
use axum::response::Response;
use axum::http::StatusCode;

use axum_keycloak_auth::decode::KeycloakToken;
use uuid::Uuid;

pub use create_tournament::handle_request as create_tournament;
pub use find_tournaments::handle_request as find_tournaments;
pub use join_tournament::handle_request as join_tournament;
pub use observe_table::handle_request as observe_table;


fn build_response(status_code: axum::http::StatusCode, message: String) -> Response {
    Response::builder().status(status_code).body(message.into()).unwrap()
}


fn create_auth_info(token: KeycloakToken<AuthRole>) -> Result<AuthInfo, AuthError> {
    let account_id = Uuid::parse_str(&token.subject).map_err(|_| AuthError::InvalidAccountId)?;
    let roles = token.roles.iter().map(|kcr| kcr.role().clone()).collect();
    Ok(AuthInfo::new(account_id, roles))
}


impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::InvalidAccountId => build_response(StatusCode::UNAUTHORIZED, self.to_string()),
            AuthError::PermissionDenied { .. } => build_response(StatusCode::FORBIDDEN, self.to_string()),
        }
    }
}


impl IntoResponse for LoadTournamentError {
    fn into_response(self) -> Response {
        match self {
            LoadTournamentError::TournamentNotFound => build_response(StatusCode::NOT_FOUND, self.to_string()),
            _ => build_response(StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        }
    }
}


impl IntoResponse for SaveTournamentError {
    fn into_response(self) -> Response {
        match self {
            SaveTournamentError::TournamentOutdated => build_response(StatusCode::CONFLICT, self.to_string()),
            _ => build_response(StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        }
    }
}


impl IntoResponse for QueryTournamentsError {
    fn into_response(self) -> Response {
        match self {
            QueryTournamentsError::DatabaseQueryError => build_response(StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        }
    }
}


impl IntoResponse for TournamentError {
    fn into_response(self) -> Response {
        match self {
            TournamentError::NotSuchTable => build_response(StatusCode::NOT_FOUND, self.to_string()),
            _ => build_response(StatusCode::BAD_REQUEST, self.to_string()),
        }
    }
}
