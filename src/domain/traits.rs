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

use crate::domain::TableEventReceiver;
use crate::domain::TournamentEvent;


pub trait PublishTournamentEvents {
    fn publish_tournament_events(&self, events: Vec<TournamentEvent>);
}


pub trait SubscribeTableEvents {
    fn subscribe_table_events(&mut self, tournament_id: Uuid, table_number: usize) -> TableEventReceiver;
}


pub trait AccessTableEventBroadcast: PublishTournamentEvents + SubscribeTableEvents {}
impl<T: PublishTournamentEvents + SubscribeTableEvents> AccessTableEventBroadcast for T {}
