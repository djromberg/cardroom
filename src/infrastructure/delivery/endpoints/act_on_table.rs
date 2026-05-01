use crate::application::ApplicationError;
use crate::application::AuthInfo;
use crate::application::AuthRole;
use crate::application::ActOnTable;
use crate::application::TableAction;

use axum::extract;
use serde::Deserialize;
use uuid::Uuid;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActOnTableRequest {
    pub action: TableAction,
}


pub async fn act_on_table<Service: ActOnTable>(
    extract::State(service): extract::State<Service>,
    extract::Path(table_id): extract::Path<Uuid>,
    extract::Json(request): extract::Json<ActOnTableRequest>,
) -> Result<(), ApplicationError> {
    let auth_info = AuthInfo::new(Uuid::new_v4(), vec![AuthRole::Participant]);
    service.act_on_table(table_id, request.action)
}
