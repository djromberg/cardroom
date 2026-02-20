use crate::application::AuthInfo;
use crate::application::ObserveTableError;
use crate::application::ObserveTable;
use crate::application::ObserveTableRequest;
use crate::application::ObserveTableResponse;

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
    extract::State(service): extract::State<Arc<Mutex<impl ObserveTable>>>,
) -> Result<Response, ObserveTableError> {
    log::info!("WEBSOCKET REQUEST");

    // TODO: check auth
    let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: crate::application::AuthRole::Member };

    let request = ObserveTableRequest {
        tournament_id,
        table_number,
    };

    let mut service = service.lock().await;
    let response = service.observe_table(request, &auth_info)?;
    Ok(wsu.on_upgrade(|socket| observe_table(socket, response)))
}


pub async fn observe_table(mut socket: WebSocket, response: ObserveTableResponse) {
    log::info!("{:?} observes table", socket);
    let _table_state = response.table_state;
    // TODO: serialize table state and send it as first message
    if socket.send(extract::ws::Message::Text("table_state".into())).await.is_ok() {
        let mut receiver = response.receiver;
        while let Ok(message) = receiver.recv().await {
            // TODO: error handling
            log::info!("sending message {:?} to {:?}", message, socket);
            if socket.send(extract::ws::Message::Text("table_message".into())).await.is_err() {
                break;
            }
        }
    }
    log::info!("{:?} finished observing table", socket);
}


impl response::IntoResponse for ObserveTableError {
    fn into_response(self) -> response::Response {
        match self {
            ObserveTableError::AuthError(error) => error.into_response(),
            ObserveTableError::LoadTournamentError(error) => error.into_response(),
            ObserveTableError::TournamentError(error) => error.into_response(),
        }
    }
}
