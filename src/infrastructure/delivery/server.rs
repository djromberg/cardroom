use super::endpoints;

use crate::application::ProvideServices;

use axum::Router;
use axum::routing;
use log::info;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use std::io::Error;
use std::sync::Arc;


#[derive(Debug)]
pub struct AxumServer {
    port: u16,
}

impl AxumServer {
    pub fn new( port: u16) -> Self {
        Self { port }
    }

    pub async fn serve<Provider: ProvideServices + Send + 'static>(&self, provider: Provider) -> Result<(), Error> {
        let address = "0.0.0.0:".to_owned() + &self.port.to_string();
        let listener = TcpListener::bind(address).await?;

        info!("listening on {}", listener.local_addr()?);

        let router = Router::new()
            .route(
                "/tournaments",
                routing::post(endpoints::create_tournament)
            )
            .with_state(Arc::new(Mutex::new(provider)));

        info!("serving cardroom application ...");

        axum::serve(listener, router).await
    }
}
