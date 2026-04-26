mod application;
mod domain;
mod infrastructure;

use std::io::Error;

use crate::{application::Application, infrastructure::{AxumServer, InMemoryTableRepository, InMemoryTournamentRepository}};


#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let table_repository = InMemoryTableRepository::new();
    let tournament_repository = InMemoryTournamentRepository::new();
    let application = Application::new(tournament_repository, table_repository);

    // let repository = InMemoryTournamentRepository::new();
    // let broadcast = TableMessageBroadcast::new();
    // let provider = ServiceProvider::new(repository, broadcast);
    let server = AxumServer::new(3020);
    server.serve(application).await?;
    Ok(())
}
