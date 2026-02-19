use crate::application::AuthError;
use crate::application::AuthInfo;
use crate::domain::LoadTournament;
use crate::domain::LoadTournamentError;
use crate::domain::SubscribeTableEvents;
use crate::domain::TableEventReceiver;
use crate::domain::TableState;
use crate::domain::TournamentError;

use thiserror::Error;
use uuid::Uuid;


#[derive(Debug, Error)]
pub enum ObserveTableError {
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    LoadTournamentError(#[from] LoadTournamentError),
    #[error(transparent)]
    TournamentError(#[from] TournamentError)
}


#[derive(Debug)]
pub struct ObserveTableRequest {
    pub tournament_id: Uuid,
    pub table_number: usize,
}


#[derive(Debug)]
pub struct ObserveTableResponse {
    pub receiver: TableEventReceiver,
    pub table_state: TableState,
}


pub trait ObserveTable {
    fn observe_table(
        &self,
        request: ObserveTableRequest,
        auth_info: &AuthInfo
    ) -> Result<ObserveTableResponse, ObserveTableError>;
}


pub(in crate::application) fn observe_table<Repository: LoadTournament, Broadcast: SubscribeTableEvents>(
    request: ObserveTableRequest,
    _auth_info: &AuthInfo,
    repository: &Repository,
    broadcast: &Broadcast,
) -> Result<ObserveTableResponse, ObserveTableError> {
    // TODO: Think about whether public / unauthenticated observation should
    //       also be handled here. We do not want to duplicate service code.
    //       An authenticated request whose author sits at the table could
    //       receive private events.
    // _ = auth_info.ensure_authenticated()?;
    let tournament = repository.load_tournament(request.tournament_id)?;
    let table_state = tournament.table_state(request.table_number)?;
    if let Some(receiver) = broadcast.subscribe_table_events(request.tournament_id, request.table_number) {
        Ok(ObserveTableResponse { receiver, table_state })
    } else {
        Err(ObserveTableError::AuthError(AuthError::AuthenticationRequired))
    }
}


#[cfg(test)]
mod tests {
}
