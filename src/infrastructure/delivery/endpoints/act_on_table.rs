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
    pub action: u32,
}


pub async fn act_on_table<Service: ActOnTable>(
    extract::State(service): extract::State<Service>,
    extract::Path(table_id): extract::Path<Uuid>,
    extract::Json(request): extract::Json<ActOnTableRequest>,
) {
    let auth_info = AuthInfo::new(Uuid::new_v4(), vec![AuthRole::Participant]);
    let action = TableAction::Check;
    service.act_on_table(table_id, action);
}
