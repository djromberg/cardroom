use axum::Router;
use axum::routing;
use log::info;
use tokio::net::TcpListener;

use std::io::Error;

use crate::application::ProvideTableServices;
use crate::application::ProvideTournamentServices;

use super::endpoints;


#[derive(Debug)]
pub struct AxumServer<TournamentServices, TableServices> {
    tournament_services: TournamentServices,
    table_services: TableServices,
    port: u16,
}


impl<TournamentServices: ProvideTournamentServices, TableServices: ProvideTableServices> AxumServer<TournamentServices, TableServices> {
    pub fn new(tournament_services: TournamentServices, table_services: TableServices, port: u16) -> Self {
        Self { tournament_services, table_services, port }
    }

    pub async fn serve(&self) -> Result<(), Error> {
        let address = "0.0.0.0:".to_owned() + &self.port.to_string();
        let listener = TcpListener::bind(address).await?;

        info!("listening on {}", listener.local_addr()?);

        let router = Router::new()
            .route(
                "/tournaments",
                routing::post(endpoints::create_tournament::<TournamentServices::CreateTournamentServiceType>)
            )
            .with_state(self.tournament_services.create_tournament_service())
            .route(
                "/tables/{table_id}/action",
                routing::post(endpoints::act_on_table::<TableServices::ActOnTableServiceType>)
            )
            .with_state(self.table_services.act_on_table_service());
        info!("serving cardroom application ...");

        axum::serve(listener, router).await
    }
}
