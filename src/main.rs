mod application;
mod domain;
mod infrastructure;

use crate::application::ProvideTournamentEvents;
use crate::application::TableServiceProvider;
use crate::application::TournamentServiceProvider;
use crate::application::TournamentEventHandler;

use crate::infrastructure::InMemoryTableRepository;
use crate::infrastructure::InMemoryTournamentRepository;
use crate::infrastructure::AxumServer;

use std::io::Error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let tournament_service_provider = TournamentServiceProvider::new(
        InMemoryTournamentRepository::new()
    );
    let table_service_provider = TableServiceProvider::new(
        InMemoryTableRepository::new()
    );

    let mut tournament_event_handler = TournamentEventHandler::new(
        table_service_provider.clone(),
        tournament_service_provider.subscribe_events(),
    );

    let server = AxumServer::new(tournament_service_provider, 3020);

    tokio::spawn(async move {
        tournament_event_handler.handle_tournament_events().await;
    });

    server.serve().await?;

    Ok(())
}
