use super::build_response;

use crate::application::AuthInfo;
use crate::application::CreateTournamentRequest;
use crate::application::CreateTournamentError;
use crate::application::CreateTournament;
use crate::application::TournamentSummary;

use axum::http::StatusCode;
use axum::{extract, Json, response};
use serde::Deserialize;
use tokio::sync::Mutex;
use uuid::Uuid;

use std::sync::Arc;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    table_count: u32,
    table_seat_count: u8
}


pub type ResponseBody = TournamentSummary;


pub async fn handle_request(
    extract::State(service): extract::State<Arc<Mutex<impl CreateTournament>>>,
    extract::Json(request): extract::Json<RequestBody>,
) -> Result<Json<ResponseBody>, CreateTournamentError> {
    let request = CreateTournamentRequest { table_count: request.table_count as u8, table_seat_count: request.table_seat_count };

    // let auth_info = AuthInfo::Unauthenticated;
    let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: crate::application::AuthRole::Member };

    let mut service = service.lock().await;
    let response = service.create_tournament(request, &auth_info)?;
    Ok(Json(response))
}


impl response::IntoResponse for CreateTournamentError {
    fn into_response(self) -> response::Response {
        match self {
            CreateTournamentError::TournamentSpecificationError(error) => build_response(StatusCode::BAD_REQUEST, error.to_string()),
            CreateTournamentError::SaveTournamentError(error) => build_response(StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            CreateTournamentError::AuthError(error) => error.into_response(),
        }
    }
}
