use super::build_response;
use super::create_auth_info;

use crate::application::AuthRole;
use crate::application::JoinTournamentRequest;
use crate::application::JoinTournamentError;
use crate::application::JoinTournament;
use crate::domain::LoadTournamentError;

use axum::http::StatusCode;
use axum::{extract, Json, response};
use axum_keycloak_auth::decode::KeycloakToken;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

use std::sync::Arc;


#[derive(Debug, Deserialize)]
pub struct RequestBody {
    nickname: String,
}


#[derive(Debug, Serialize)]
pub struct ResponseBody {
    table_number: usize,
}


pub async fn handle_request(
    extract::State(service): extract::State<Arc<Mutex<impl JoinTournament>>>,
    extract::Path(tournament_id): extract::Path<Uuid>,
    extract::Extension(token): extract::Extension<KeycloakToken<AuthRole>>,
    extract::Json(request): extract::Json<RequestBody>,
) -> Result<Json<ResponseBody>, JoinTournamentError> {
    let auth_info = create_auth_info(token)?;
    let request = JoinTournamentRequest { tournament_id, nickname: request.nickname };
    let mut service = service.lock().await;
    let response = service.join_tournament(request, &auth_info)?;
    Ok(Json(ResponseBody { table_number: response.table_number }))
}


impl response::IntoResponse for JoinTournamentError {
    fn into_response(self) -> response::Response {
        match self {
            JoinTournamentError::LoadTournamentError(error) => {
                match error {
                    LoadTournamentError::TournamentNotFound => build_response(StatusCode::NOT_FOUND, error.to_string()),
                    _ => build_response(StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
                }
            },
            JoinTournamentError::SaveTournamentError(error) => build_response(StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            JoinTournamentError::AuthError(error) => error.into_response(),
            JoinTournamentError::NicknameError(error) => build_response(StatusCode::BAD_REQUEST, error.to_string()),
            JoinTournamentError::TournamentError(error) => build_response(StatusCode::BAD_REQUEST, error.to_string()),
        }
    }
}
