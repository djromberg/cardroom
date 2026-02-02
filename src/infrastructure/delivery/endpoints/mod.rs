mod create_tournament;
mod join_tournament;
mod observe_table;

use crate::application::AuthError;

use axum::response::IntoResponse;
use axum::response::Response;
use axum::http::StatusCode;


pub use create_tournament::handle_request as create_tournament;
pub use join_tournament::handle_request as join_tournament;


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
