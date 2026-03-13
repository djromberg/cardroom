use super::build_response;
use super::create_auth_info;

use crate::application::AuthRole;
use crate::application::CreateTournamentRequest;
use crate::application::CreateTournamentError;
use crate::application::CreateTournament;
use crate::application::TournamentSummary;

use axum::http::StatusCode;
use axum::{extract, Json, response};
use axum_keycloak_auth::decode::KeycloakToken;
use tokio::sync::Mutex;

use std::sync::Arc;


pub async fn handle_request(
    extract::State(service): extract::State<Arc<Mutex<impl CreateTournament>>>,
    extract::Extension(token): extract::Extension<KeycloakToken<AuthRole>>,
    extract::Json(request): extract::Json<CreateTournamentRequest>,
) -> Result<Json<TournamentSummary>, CreateTournamentError> {
    let auth_info = create_auth_info(token)?;
    let mut service = service.lock().await;
    let response = service.create_tournament(request, &auth_info)?;
    Ok(Json(response))
}


impl response::IntoResponse for CreateTournamentError {
    fn into_response(self) -> response::Response {
        match self {
            CreateTournamentError::TournamentSpecificationError(error) => build_response(StatusCode::BAD_REQUEST, error.to_string()),
            CreateTournamentError::SaveTournamentError(error) => error.into_response(),
            CreateTournamentError::AuthError(error) => error.into_response(),
        }
    }
}
