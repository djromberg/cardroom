mod application;
mod domain;
mod infrastructure;

use std::io::Error;

use application::ServiceProvider;
use infrastructure::InMemoryTournamentRepository;
use infrastructure::AxumServer;

use crate::domain::TableEventBroadcast;


#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let repository = InMemoryTournamentRepository::new();
    let broadcast = TableEventBroadcast::new();
    let provider = ServiceProvider::new(repository, broadcast);
    let server = AxumServer::new(3020);
    server.serve(provider).await
}
