use crate::application::ApplicationError;
use crate::application::EventBus;
use crate::application::TableRepository;
use crate::domain::Table;
use crate::domain::TableEvent;
use crate::domain::TableId;
use crate::domain::TableSpecification;
use crate::domain::TournamentId;


#[derive(Debug)]
pub struct OpenTablesService<Repository> {
    repository: Repository,
    event_bus: EventBus<TableEvent>,
}

impl<Repository: TableRepository> OpenTablesService<Repository> {
    pub fn new(repository: Repository, event_bus: EventBus<TableEvent>) -> Self {
        Self { repository, event_bus }
    }

    pub fn open_tables(&self, tournament_id: TournamentId, table_ids: Vec<TableId>, table_spec: TableSpecification) -> Result<(), ApplicationError> {
        let events = self.repository.with_tx(|tx| {
            for table_id in table_ids {
                let table = Table::new(table_id, tournament_id, &table_spec);
                tx.save_table(table)?;
            }
            Ok(())
        })?;
        log::info!("Tables opened for tournament {:?}", tournament_id);
        self.event_bus.send(events);
        Ok(())
    }
}
