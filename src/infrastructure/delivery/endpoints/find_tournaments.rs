use super::create_auth_info;

use crate::application::AuthRole;
use crate::application::FindTournamentsRequest;
use crate::application::FindTournamentsResponse;
use crate::application::FindTournamentsError;
use crate::application::FindTournaments;

use axum::{extract, Json, response};
use axum_keycloak_auth::decode::KeycloakToken;
use serde::Deserialize;
use tokio::sync::Mutex;

use std::sync::Arc;


// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct QueryParameters {
//     min_table_count: Option<u16>,
//     max_table_count: Option<u16>
// }


// pub type ResponseBody = Vec<TournamentSummary>;


pub async fn handle_request(
    extract::State(service): extract::State<Arc<Mutex<impl FindTournaments>>>,
    extract::Extension(token): extract::Extension<KeycloakToken<AuthRole>>,
    extract::Query(request): extract::Query<FindTournamentsRequest>
) -> Result<Json<FindTournamentsResponse>, FindTournamentsError> {
    let auth_info = create_auth_info(token)?;
    // let request = FindTournamentsRequest { };
    let service = service.lock().await;
    let response = service.find_tournaments(request, &auth_info)?;
    // let summaries = response.infos.iter().map(create_summary_from_info).collect();
    Ok(Json(response))
}


impl response::IntoResponse for FindTournamentsError {
    fn into_response(self) -> response::Response {
        match self {
            FindTournamentsError::QueryTournamentsError(error) => error.into_response(),
            FindTournamentsError::AuthError(error) => error.into_response(),
        }
    }
}
