use super::tournament::Tournament;

use thiserror::Error;
use uuid::Uuid;


#[derive(Debug, Error, Clone, Copy)]
pub enum LoadTournamentError {
    #[error("Tournament not found")]
    TournamentNotFound,
    #[error("Cannot access tournament database for reading")]
    DatabaseReadingError,
}

pub trait LoadTournament {
    fn load_tournament(&self, tournament_id: Uuid) -> Result<Tournament, LoadTournamentError>;
}


#[derive(Debug, Error, Clone, Copy)]
pub enum SaveTournamentError {
    #[error("There is a newer version of the given tournament in the database")]
    TournamentOutdated,
    #[error("Cannot access tournament database for writing")]
    DatabaseWritingError,
}

pub trait SaveTournament {
    fn save_tournament(&mut self, tournament: Tournament) -> Result<(), SaveTournamentError>;
}


#[derive(Debug, Error)]
pub enum QueryTournamentsError {
    #[error("Cannot access tournament database for querying")]
    DatabaseQueryError,
}

pub trait QueryTournaments {
    fn query_tournaments(&self) -> Result<Vec<Tournament>, QueryTournamentsError>;
}


pub trait AccessTournaments: LoadTournament + SaveTournament + QueryTournaments {}
impl<T: LoadTournament + SaveTournament + QueryTournaments> AccessTournaments for T {}


// ----------------------- tryout:

use crate::domain::table::TableEvent;


pub trait PublishTableEvents {
    fn publish_table_events(&self, events: Vec<TableEvent>);
}
