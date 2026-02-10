use crate::application::AuthInfo;
use crate::application::ObserveTableError;
use crate::application::ObserveTable;
use crate::application::ObserveTableRequest;
use crate::domain::TableEventReceiver;

use axum::body::Bytes;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::WebSocket;
use axum::response::Response;
use axum::{extract, response};
use tokio::sync::Mutex;
use uuid::Uuid;

use std::sync::Arc;


pub async fn handle_request(
    wsu: WebSocketUpgrade,
    extract::Path((tournament_id, table_number)): extract::Path<(Uuid, usize)>,
    extract::State(service): extract::State<Arc<Mutex<impl ObserveTable + Send + 'static>>>,
) -> Result<Response, ObserveTableError> {
    log::info!("WEBSOCKET REQUEST");

    // TODO: check auth
    let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: crate::application::AuthRole::Member };

    let request = ObserveTableRequest {
        tournament_id,
        table_number,
    };

    let service = service.lock().await;
    let response = service.observe_table(request, &auth_info)?;
    Ok(wsu.on_upgrade(|socket| observe_table(socket, response.receiver)))
}


pub async fn observe_table(mut socket: WebSocket, mut receiver: TableEventReceiver) {
    log::info!("receiving events ... ");
    while let Ok(event) = receiver.recv().await {
        // TODO: convert event to message
        // TODO: error handling
        log::info!("sending event {:?}", event);
        _ = socket.send(extract::ws::Message::Ping(Bytes::new())).await;
    }
}


impl response::IntoResponse for ObserveTableError {
    fn into_response(self) -> response::Response {
        match self {
            ObserveTableError::AuthError(error) => error.into_response(),
        }
    }
}
