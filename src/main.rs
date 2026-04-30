mod application;
mod domain;
mod infrastructure;

use crate::application::ServiceProvider;

use crate::infrastructure::InMemoryTableRepository;
use crate::infrastructure::InMemoryTournamentRepository;
use crate::infrastructure::AxumServer;

use std::io::Error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let table_repository = InMemoryTableRepository::new();
    let tournament_repository = InMemoryTournamentRepository::new();
    let service_provider = ServiceProvider::new(tournament_repository, table_repository);
    let server = AxumServer::new(service_provider, 3020);
    server.serve().await?;
    Ok(())
}
