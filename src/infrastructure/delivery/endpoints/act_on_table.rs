use crate::application::ApplicationError;
use crate::application::AuthRole;
use crate::application::ActOnTable;
use crate::infrastructure::delivery::endpoints::create_auth_info;

use axum::extract;
use axum_keycloak_auth::decode::KeycloakToken;
use serde::Deserialize;
use uuid::Uuid;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActOnTableRequest {
    Bet(u32),
    Check,
    Fold,
}


pub async fn act_on_table<Service: ActOnTable>(
    extract::State(service): extract::State<Service>,
    extract::Extension(token): extract::Extension<KeycloakToken<AuthRole>>,
    extract::Path(table_id): extract::Path<Uuid>,
    extract::Json(request): extract::Json<ActOnTableRequest>,
) -> Result<(), ApplicationError> {
    let auth_info = create_auth_info(token)?;
    match request {
        ActOnTableRequest::Bet(amount) => service.bet(table_id, amount, &auth_info),
        ActOnTableRequest::Check => service.check(table_id, &auth_info),
        ActOnTableRequest::Fold => service.fold(table_id, &auth_info),
    }
}
