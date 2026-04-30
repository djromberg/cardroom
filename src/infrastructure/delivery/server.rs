use axum::extract;
use axum::Router;
use axum::routing;
use log::info;
use tokio::net::TcpListener;

use std::io::Error;

use crate::application::CreateTournament;
use crate::application::CreateTournamentRequest;
use crate::application::ProvideServices;


#[derive(Debug)]
pub struct AxumServer<Provider> {
    provider: Provider,
    port: u16,
}


impl<Provider: ProvideServices> AxumServer<Provider> {
    pub fn new(provider: Provider, port: u16) -> Self {
        Self { provider, port }
    }

    pub async fn serve(&self) -> Result<(), Error> {
        let address = "0.0.0.0:".to_owned() + &self.port.to_string();
        let listener = TcpListener::bind(address).await?;

        info!("listening on {}", listener.local_addr()?);

        let router = Router::new()
            .route(
                "/tournaments",
                routing::post(create_tournament::<Provider::CreateTournamentServiceType>)
            )
            .with_state(self.provider.create_tournament_service());
        info!("serving cardroom application ...");

        axum::serve(listener, router).await
    }
}


async fn create_tournament<Service: CreateTournament>(
    extract::State(service): extract::State<Service>,
    extract::Json(request): extract::Json<CreateTournamentRequest>,
) {
}
