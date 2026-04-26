// use super::auth::create_auth_layer;

use axum::extract;
use axum::Json;
use axum::Router;
use axum::routing;
use log::info;
use serde::Deserialize;
use tokio::net::TcpListener;

use std::io::Error;
use std::sync::Arc;

use crate::application::ApplicationError;
use crate::application::ServiceProvider;


#[derive(Debug)]
pub struct AxumServer {
    port: u16,
}

impl AxumServer {
    pub fn new( port: u16) -> Self {
        Self { port }
    }

    pub async fn serve<Provider: ServiceProvider + Send + Sync + 'static>(&self, provider: Provider) -> Result<(), Error> {
        let address = "0.0.0.0:".to_owned() + &self.port.to_string();
        let listener = TcpListener::bind(address).await?;

        info!("listening on {}", listener.local_addr()?);

        let router = Router::new()
            .route(
                "/tournaments",
                routing::post(create_tournament)
            )
            .with_state(Arc::new(provider)
        );
        info!("serving cardroom application ...");

        axum::serve(listener, router).await
    }
}


async fn create_tournament(
    extract::State(app): extract::State<Arc<impl ServiceProvider>>,
    extract::Json(request): extract::Json<CreateTournamentRequest>,
) {
    app.print_my_name();
    app.create_tournament(request.table_count, request.table_seat_count);
    // app.create_tournament(request.table_count, request.table_seat_count)
    // (StatusCode::OK, ())
}


#[derive(Deserialize)]
struct CreateTournamentRequest {
    table_count: u8,
    table_seat_count: u8,
}
