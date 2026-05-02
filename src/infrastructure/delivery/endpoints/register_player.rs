use super::create_auth_info;

use crate::application::ApplicationError;
use crate::application::AuthRole;
use crate::application::RegisterPlayer;

use axum::extract;
use axum_keycloak_auth::decode::KeycloakToken;

use uuid::Uuid;


pub async fn register_player<Service: RegisterPlayer>(
    extract::State(service): extract::State<Service>,
    extract::Extension(token): extract::Extension<KeycloakToken<AuthRole>>,
    extract::Path(tournament_id): extract::Path<Uuid>,
) -> Result<(), ApplicationError> {
    let auth_info = create_auth_info(token)?;
    service.register_player(tournament_id, &auth_info)?;
    Ok(())
}
