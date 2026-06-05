use tokio::sync::broadcast::Receiver;

use crate::application::OpenTables;
use crate::application::ProvidePrivateTableServices;

use crate::domain::TournamentEvent;
use crate::domain::TournamentEventType;


#[derive(Debug)]
pub struct TournamentEventHandler<ServiceProvider> {
    provider: ServiceProvider,
    receiver: Receiver<TournamentEvent>,
}

impl<ServiceProvider: ProvidePrivateTableServices> TournamentEventHandler<ServiceProvider> {
    pub fn new(provider: ServiceProvider, receiver: Receiver<TournamentEvent>) -> Self {
        Self { provider, receiver }
    }

    pub async fn handle_tournament_events(&mut self) {
        while let Ok(event) = self.receiver.recv().await {
            match event.event_type {
                TournamentEventType::TournamentCreated { table_spec, table_ids } => {
                    log::info!("Tournament created, opening tables ...");
                    let service = self.provider.open_tables_service();
                    // TODO: handle error
                    _ = service.open_tables(event.tournament_id, table_ids, table_spec).await;
                },
                _ => {},
            }
        }
    }
}
