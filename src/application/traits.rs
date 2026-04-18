use crate::domain::Table;
use crate::domain::TableEvent;
use crate::domain::TableId;
use crate::domain::Tournament;
use crate::domain::TournamentEvent;
use crate::domain::TournamentId;

use thiserror::Error;


#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Resource not found")]
    ResourceNotFound,
    #[error("Internal error")]
    InternalError,
}


pub trait TournamentRepository {
    fn load_tournament(&self, id: TournamentId) -> Result<Tournament, RepositoryError>;
    fn save_tournament(&mut self, tournament: Tournament) -> Result<Vec<TournamentEvent>, RepositoryError>;
}


pub trait TableRepository {
    fn load_table(&self, id: TableId) -> Result<Table, RepositoryError>;
    fn save_table(&mut self, table: Table) -> Result<Vec<TableEvent>, RepositoryError>;
}


pub trait TableSpectator {
    fn send_state(&self);
    fn player_seated(&self, nickname: &String, stack: u32, seat_number: u8);
}
