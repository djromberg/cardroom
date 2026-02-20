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

use crate::domain::TableMessageReceiver;
use crate::domain::TournamentMessage;


pub trait PublishTournamentMessages {
    fn publish_tournament_messages(&self, messages: Vec<TournamentMessage>);
}


pub trait SubscribeTableMessages {
    fn subscribe_table_messages(&mut self, tournament_id: Uuid, table_number: usize) -> TableMessageReceiver;
}


pub trait AccessTableMessageBroadcast: PublishTournamentMessages + SubscribeTableMessages {}
impl<T: PublishTournamentMessages + SubscribeTableMessages> AccessTableMessageBroadcast for T {}
