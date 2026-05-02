use super::create_auth_info;

use crate::application::ApplicationError;
use crate::application::AuthRole;
use crate::application::CreateTournament;

use axum::extract;
use axum_keycloak_auth::decode::KeycloakToken;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTournamentRequest {
    pub table_count: u8,
    pub table_seat_count: u8,
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTournamentResponse {
    pub tournament_id: Uuid,
}


pub async fn create_tournament<Service: CreateTournament>(
    extract::State(service): extract::State<Service>,
    extract::Extension(token): extract::Extension<KeycloakToken<AuthRole>>,
    extract::Json(request): extract::Json<CreateTournamentRequest>,
) -> Result<extract::Json<CreateTournamentResponse>, ApplicationError> {
    let auth_info = create_auth_info(token)?;
    let tournament_id = service.create_tournament(request.table_count, request.table_seat_count, &auth_info)?;
    let response = CreateTournamentResponse { tournament_id };
    Ok(extract::Json(response))
}
