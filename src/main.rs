mod application;
mod domain;
mod infrastructure;

use std::io::Error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    // env_logger::init();
    // let repository = InMemoryTournamentRepository::new();
    // let broadcast = TableMessageBroadcast::new();
    // let provider = ServiceProvider::new(repository, broadcast);
    // let server = AxumServer::new(3020);
    // server.serve(provider).await
    Ok(())
}
