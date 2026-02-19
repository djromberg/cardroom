mod create_tournament;
mod join_tournament;
mod observe_table;

use crate::application::AuthError;
use crate::domain::LoadTournamentError;
use crate::domain::TournamentError;

use axum::response::IntoResponse;
use axum::response::Response;
use axum::http::StatusCode;


pub use create_tournament::handle_request as create_tournament;
pub use join_tournament::handle_request as join_tournament;
pub use observe_table::handle_request as observe_table;


fn build_response(status_code: axum::http::StatusCode, message: String) -> Response {
    Response::builder().status(status_code).body(message.into()).unwrap()
}


impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::AuthenticationRequired => build_response(StatusCode::UNAUTHORIZED, self.to_string()),
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


impl IntoResponse for TournamentError {
    fn into_response(self) -> Response {
        match self {
            TournamentError::NotSuchTable => build_response(StatusCode::NOT_FOUND, self.to_string()),
            _ => build_response(StatusCode::BAD_REQUEST, self.to_string()),
        }
    }
}
