use crate::application::AuthError;
use crate::application::AuthInfo;

use crate::application::AuthRole;
use crate::domain::QueryTournaments;
use crate::domain::QueryTournamentsError;
use crate::domain::Tournament;

use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;


#[derive(Debug, Error)]
pub enum FindTournamentsError {
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    QueryTournamentsError(#[from] QueryTournamentsError),
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FindTournamentsRequest {
    min_table_count: Option<u16>,
    max_table_count: Option<u16>
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TournamentSummary {
    pub tournament_id: String,
    pub seat_count: u32,
    pub table_count: u16,
    pub table_seat_count: u8,
    pub player_count: u32,
    pub creation_date: String,
    pub phase: String,
    pub player_is_involved: bool,
}


pub type FindTournamentsResponse = Vec<TournamentSummary>;


pub trait FindTournaments {
    fn find_tournaments(&self, request: FindTournamentsRequest, auth_info: &AuthInfo) -> Result<FindTournamentsResponse, FindTournamentsError>;
}


pub(in crate::application) fn find_tournaments<Repository: QueryTournaments>(
    request: FindTournamentsRequest,
    auth_info: &AuthInfo,
    repository: &Repository,
) -> Result<FindTournamentsResponse, FindTournamentsError> {
    let account_id = auth_info.expect_role(AuthRole::Observer)?;

    let tournaments = repository.query_tournaments()?;

    let summaries = tournaments.iter().map(|tournament| get_tournament_summary(tournament, account_id)).collect();

    Ok(summaries)
}


pub(in crate::application) fn get_tournament_summary(tournament: &Tournament, account_id: Uuid) -> TournamentSummary {
    TournamentSummary {
        tournament_id: tournament.id().to_string(),
        table_count: tournament.table_count() as u16,
        table_seat_count: tournament.table_seat_count(),
        player_count: tournament.player_count() as u32,
        phase: "WaitingForPlayers".to_string(),
        creation_date: "2024-08-01T14:38:32.499588".to_string(),
        player_is_involved: tournament.has_player(account_id),
        seat_count: 23,
    }
}


#[cfg(test)]
mod tests {
}
