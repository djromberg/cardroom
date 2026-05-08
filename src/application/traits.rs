use crate::application::RepositoryError;
use crate::application::ApplicationError;
use crate::domain::Table;
use crate::domain::TableEvent;
use crate::domain::TableId;
use crate::domain::Tournament;
use crate::domain::TournamentEvent;
use crate::domain::TournamentId;

use async_trait::async_trait;


pub trait TableRepositoryTransaction {
    fn load_table(&self, id: TableId) -> Result<Table, RepositoryError>;
    fn save_table(&mut self, table: Table) -> Result<(), RepositoryError>;
}


#[async_trait]
pub trait TableRepository: Clone + Sync + Send + 'static {
    async fn with_tx<F>(&self, f: F) -> Result<Vec<TableEvent>, ApplicationError>
    where
        F: FnOnce(&mut dyn TableRepositoryTransaction) -> Result<(), ApplicationError> + Send;
}


#[async_trait]
pub trait TournamentRepository: Clone + Sync + Send + 'static {
    async fn load_tournament(&self, id: TournamentId) -> Result<Tournament, RepositoryError>;
    async fn save_tournament(&self, tournament: Tournament) -> Result<Vec<TournamentEvent>, RepositoryError>;
}
