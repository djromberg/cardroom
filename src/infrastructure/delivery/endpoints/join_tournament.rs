use super::build_response;

use crate::application::AuthInfo;
use crate::application::JoinTournamentRequest;
use crate::application::JoinTournamentError;
use crate::application::JoinTournament;
use crate::domain::LoadTournamentError;

use axum::http::StatusCode;
use axum::{extract, Json, response};
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
    table_id: Uuid,
}


pub async fn handle_request(
    extract::State(service): extract::State<Arc<Mutex<impl JoinTournament>>>,
    extract::Path(tournament_id): extract::Path<Uuid>,
    extract::Json(request): extract::Json<RequestBody>,
) -> Result<Json<ResponseBody>, JoinTournamentError> {
    let request = JoinTournamentRequest { tournament_id, nickname: request.nickname };

    // let auth_info = AuthInfo::Unauthenticated;
    let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: crate::application::AuthRole::Member };

    let mut service = service.lock().await;
    let response = service.join_tournament(request, &auth_info)?;
    Ok(Json(ResponseBody { table_id: response.table_id }))
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
