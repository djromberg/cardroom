use crate::application::AuthInfo;
use crate::application::FindTournamentsRequest;
use crate::application::FindTournamentsError;
use crate::application::FindTournaments;

use axum::{extract, Json, response};
use serde::Serialize;
use tokio::sync::Mutex;
use uuid::Uuid;

use std::sync::Arc;


#[derive(Debug, Serialize)]
pub struct ResponseBody {
    // TODO: use serialized application's TournamentInfo
    pub tournament_count: usize
}


pub async fn handle_request(
    extract::State(service): extract::State<Arc<Mutex<impl FindTournaments>>>,
) -> Result<Json<ResponseBody>, FindTournamentsError> {
    let request = FindTournamentsRequest { };

    // let auth_info = AuthInfo::Unauthenticated;
    let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: crate::application::AuthRole::Member };

    let service = service.lock().await;
    let response = service.find_tournaments(request, &auth_info)?;
    Ok(Json(ResponseBody { tournament_count: response.infos.len() }))
}


impl response::IntoResponse for FindTournamentsError {
    fn into_response(self) -> response::Response {
        match self {
            FindTournamentsError::QueryTournamentsError(error) => error.into_response(),
            FindTournamentsError::AuthError(error) => error.into_response(),
        }
    }
}
