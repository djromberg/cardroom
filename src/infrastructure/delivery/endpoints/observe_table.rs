use super::build_response;

use crate::application::AuthInfo;
use crate::application::ObserveTableRequest;
use crate::application::ObserveTableError;
use crate::application::ObserveTable;

use axum::extract::WebSocketUpgrade;
use axum::extract::ws::WebSocket;
use axum::http::StatusCode;
use axum::{extract, Json, response};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;

use std::sync::Arc;


#[derive(Debug, Deserialize)]
pub struct RequestBody {
}


#[derive(Debug, Serialize)]
pub struct ResponseBody {
}


pub async fn handle_request(
    wsu: WebSocketUpgrade,
    extract::State(service): extract::State<Arc<Mutex<impl ObserveTable + Send + 'static>>>,
    extract::Path(tournament_id): extract::Path<Uuid>,
    extract::Path(table_number): extract::Path<usize>,
    extract::Json(request): extract::Json<RequestBody>,
) -> Result<Json<ResponseBody>, ObserveTableError> {
    
    // TODO: check auth
    let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: crate::application::AuthRole::Member };

    wsu.on_upgrade(move |socket| observe_table(socket, service));

    Ok(Json(ResponseBody { }))
}


pub async fn observe_table(
    socket: WebSocket,
    service: Arc<Mutex<impl ObserveTable>>,
) {
    let mut service = service.lock().await;
    // TODO: create websocket table observer using socket
    //       call service trait method with observer

    // service.observe_table(request, auth_info)
}


impl response::IntoResponse for ObserveTableError {
    fn into_response(self) -> response::Response {
        match self {
            ObserveTableError::AuthError(error) => error.into_response(),
        }
    }
}
