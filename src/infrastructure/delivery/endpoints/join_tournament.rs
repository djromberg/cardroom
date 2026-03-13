use super::build_response;
use super::create_auth_info;

use crate::application::AuthRole;
use crate::application::JoinTournamentRequest;
use crate::application::JoinTournamentError;
use crate::application::JoinTournament;
use crate::application::JoinTournamentResponse;

use axum::http::StatusCode;
use axum::{extract, Json, response};
use axum_keycloak_auth::decode::KeycloakToken;
use tokio::sync::Mutex;
use uuid::Uuid;

use std::sync::Arc;


pub async fn handle_request(
    extract::State(service): extract::State<Arc<Mutex<impl JoinTournament>>>,
    extract::Path(tournament_id): extract::Path<Uuid>,
    extract::Extension(token): extract::Extension<KeycloakToken<AuthRole>>,
    extract::Json(request): extract::Json<JoinTournamentRequest>,
) -> Result<Json<JoinTournamentResponse>, JoinTournamentError> {
    let auth_info = create_auth_info(token)?;
    let request = JoinTournamentRequest { tournament_id, nickname: request.nickname };
    let mut service = service.lock().await;
    let response = service.join_tournament(request, &auth_info)?;
    Ok(Json(response))
}


impl response::IntoResponse for JoinTournamentError {
    fn into_response(self) -> response::Response {
        match self {
            JoinTournamentError::AuthError(error) => error.into_response(),
            JoinTournamentError::NicknameError(error) => build_response(StatusCode::BAD_REQUEST, error.to_string()),
            JoinTournamentError::LoadTournamentError(error) => error.into_response(),
            JoinTournamentError::TournamentError(error) => error.into_response(),
            JoinTournamentError::SaveTournamentError(error) => error.into_response(),
        }
    }
}
