use crate::application::AuthError;
use crate::application::AuthInfo;

use crate::domain::QueryTournaments;
use crate::domain::QueryTournamentsError;
use crate::domain::Tournament;

use thiserror::Error;
use uuid::Uuid;


#[derive(Debug, Error)]
pub enum FindTournamentsError {
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    QueryTournamentsError(#[from] QueryTournamentsError),
}


#[derive(Debug)]
pub struct FindTournamentsRequest {
}


#[derive(Debug)]
pub enum TournamentStage {
    WaitingForPlayers(Option<usize>), // Not yet started, player might have joined table numer (usize)
    Running(Option<usize>),           // Running, player might be playing on table number (usize)
    Finished                          // Finished
}


#[derive(Debug)]
pub struct TournamentInfo {
    pub id: Uuid,
    pub table_count: usize,
    pub table_seat_count: u8,
    pub player_count: usize,
    pub stage: TournamentStage,
}


#[derive(Debug)]
pub struct FindTournamentsResponse {
    pub infos: Vec<TournamentInfo>
}


pub trait FindTournaments {
    fn find_tournaments(&self, request: FindTournamentsRequest, auth_info: &AuthInfo) -> Result<FindTournamentsResponse, FindTournamentsError>;
}


pub(in crate::application) fn find_tournaments<Repository: QueryTournaments>(
    request: FindTournamentsRequest,
    auth_info: &AuthInfo,
    repository: &Repository,
) -> Result<FindTournamentsResponse, FindTournamentsError> {
    let account_id = auth_info.ensure_authenticated()?;

    let tournaments = repository.query_tournaments()?;

    let infos = tournaments.iter().map(|tournament| {
        TournamentInfo {
            id: tournament.id(),
            table_count: tournament.table_count(),
            table_seat_count: tournament.table_seat_count(),
            player_count: tournament.player_count(),
            stage: get_tournament_stage(tournament, account_id)
        }
    }).collect();

    Ok(FindTournamentsResponse { infos })
}


fn get_tournament_stage(tournament: &Tournament, account_id: Uuid) -> TournamentStage {
    let table_number = tournament.players_table_number(account_id);
    if tournament.is_waiting_for_players() {
        TournamentStage::WaitingForPlayers(table_number)
    } else if tournament.is_finished() {
        TournamentStage::Finished
    } else {
        TournamentStage::Running(table_number)
    }
}


#[cfg(test)]
mod tests {
}
