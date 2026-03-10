use super::create_auth_info;

use crate::application::AuthRole;
use crate::application::FindTournamentsRequest;
use crate::application::FindTournamentsResponse;
use crate::application::FindTournamentsError;
use crate::application::FindTournaments;

use axum::{extract, Json, response};
use axum_keycloak_auth::decode::KeycloakToken;
use tokio::sync::Mutex;

use std::sync::Arc;


pub async fn handle_request(
    extract::State(service): extract::State<Arc<Mutex<impl FindTournaments>>>,
    extract::Extension(token): extract::Extension<KeycloakToken<AuthRole>>,
    extract::Query(request): extract::Query<FindTournamentsRequest>
) -> Result<Json<FindTournamentsResponse>, FindTournamentsError> {
    let auth_info = create_auth_info(token)?;
    let service = service.lock().await;
    let response = service.find_tournaments(request, &auth_info)?;
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
