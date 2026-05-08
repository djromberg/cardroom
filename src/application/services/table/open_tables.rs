use crate::application::ApplicationError;
use crate::application::EventBus;
use crate::application::TableRepositorySimple;
use crate::domain::Table;
use crate::domain::TableEvent;
use crate::domain::TableId;
use crate::domain::TableSpecification;
use crate::domain::TournamentId;

use async_trait::async_trait;


#[async_trait]
pub trait OpenTables {
    async fn open_tables(&self, tournament_id: TournamentId, table_ids: Vec<TableId>, table_spec: TableSpecification) -> Result<(), ApplicationError>;
}


#[derive(Debug, Clone)]
pub struct OpenTablesService<Repository> {
    repository: Repository,
    event_bus: EventBus<TableEvent>,
}

impl<Repository: TableRepositorySimple> OpenTablesService<Repository> {
    pub fn new(repository: Repository, event_bus: EventBus<TableEvent>) -> Self {
        Self { repository, event_bus }
    }

}

#[async_trait]
impl<Repository: TableRepositorySimple> OpenTables for OpenTablesService<Repository> {
    async fn open_tables(&self, tournament_id: TournamentId, table_ids: Vec<TableId>, table_spec: TableSpecification) -> Result<(), ApplicationError> {
        let mut tables = vec![];
        for table_id in table_ids {
            log::info!("Creating table {:?}", table_id);
            let table = Table::new(table_id, tournament_id, &table_spec);
            tables.push(table);
        }
        let events = self.repository.save_tables(tables).await?;
        log::info!("Tables opened for tournament {:?}", tournament_id);
        self.event_bus.send(events);
        Ok(())
    }
}
