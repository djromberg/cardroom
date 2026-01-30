mod application;
mod domain;
mod infrastructure;

use std::io::Error;

use application::ServiceProvider;
use infrastructure::InMemoryTournamentRepository;
use infrastructure::AxumServer;
use infrastructure::LoggingBroadcast;


#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let repository = InMemoryTournamentRepository::new();
    let broadcast = LoggingBroadcast::new();
    let provider = ServiceProvider::new(repository, broadcast);
    let server = AxumServer::new(3000);
    server.serve(provider).await
}
