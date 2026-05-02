use crate::application::ApplicationError;
use crate::application::AuthInfo;
use crate::application::AuthRole;
use crate::application::ActOnTable;

use axum::extract;
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
    extract::Path(table_id): extract::Path<Uuid>,
    extract::Json(request): extract::Json<ActOnTableRequest>,
) -> Result<(), ApplicationError> {
    let auth_info = AuthInfo::new(Uuid::new_v4(), vec![AuthRole::Participant]);
    match request {
        ActOnTableRequest::Bet(amount) => service.bet(table_id, amount, &auth_info),
        ActOnTableRequest::Check => service.check(table_id, &auth_info),
        ActOnTableRequest::Fold => service.fold(table_id, &auth_info),
    }
}
