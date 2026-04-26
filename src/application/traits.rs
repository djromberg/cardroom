use crate::application::RepositoryError;
use crate::application::ApplicationError;
use crate::domain::Table;
use crate::domain::TableEvent;
use crate::domain::TableId;
use crate::domain::Tournament;
use crate::domain::TournamentEvent;
use crate::domain::TournamentId;


pub trait TableRepositoryTransaction {
    fn load_table(&self, id: TableId) -> Result<Table, RepositoryError>;
    fn save_table(&mut self, table: Table) -> Result<(), RepositoryError>;
}


pub trait TableRepository {
    fn with_tx<F>(&self, f: F) -> Result<Vec<TableEvent>, ApplicationError>
    where
        F: FnOnce(&mut dyn TableRepositoryTransaction) -> Result<(), ApplicationError>;
}


pub trait TournamentRepository {
    fn load_tournament(&self, id: TournamentId) -> Result<Tournament, RepositoryError>;
    fn save_tournament(&self, tournament: Tournament) -> Result<Vec<TournamentEvent>, RepositoryError>;
}


pub trait CreateTournament {
    fn create_tournament(&self, table_count: u8, table_seat_count: u8) -> Result<(), ApplicationError>;
}
