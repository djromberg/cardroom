use crate::application::RepositoryError;
use crate::domain::Table;
use crate::domain::TableEvent;
use crate::domain::TableId;
use crate::domain::Tournament;
use crate::domain::TournamentEvent;
use crate::domain::TournamentId;

use async_trait::async_trait;


#[async_trait]
pub trait TableRepository: Clone + Sync + Send + 'static {
    async fn load_table(&self, table_id: TableId) -> Result<Table, RepositoryError>;
    async fn save_table(&self, table: Table) -> Result<Vec<TableEvent>, RepositoryError>;
    async fn save_tables(&self, tables: Vec<Table>) -> Result<Vec<TableEvent>, RepositoryError>;
}


#[async_trait]
pub trait TournamentRepository: Clone + Sync + Send + 'static {
    async fn load_tournament(&self, id: TournamentId) -> Result<Tournament, RepositoryError>;
    async fn save_tournament(&self, tournament: Tournament) -> Result<Vec<TournamentEvent>, RepositoryError>;
}
