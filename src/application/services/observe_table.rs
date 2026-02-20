use crate::application::AuthError;
use crate::application::AuthInfo;
use crate::domain::LoadTournament;
use crate::domain::LoadTournamentError;
use crate::domain::SubscribeTableMessages;
use crate::domain::TableMessageReceiver;
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
    pub receiver: TableMessageReceiver,
    pub table_state: TableState,
}


pub trait ObserveTable {
    fn observe_table(
        &mut self,
        request: ObserveTableRequest,
        auth_info: &AuthInfo
    ) -> Result<ObserveTableResponse, ObserveTableError>;
}


pub(in crate::application) fn observe_table<Repository: LoadTournament, Broadcast: SubscribeTableMessages>(
    request: ObserveTableRequest,
    _auth_info: &AuthInfo,
    repository: &Repository,
    broadcast: &mut Broadcast,
) -> Result<ObserveTableResponse, ObserveTableError> {
    // TODO: Think about whether public / unauthenticated observation should
    //       also be handled here. We do not want to duplicate service code.
    //       An authenticated request whose author sits at the table could
    //       receive private messages.
    // _ = auth_info.ensure_authenticated()?;
    let tournament = repository.load_tournament(request.tournament_id)?;
    let table_state = tournament.table_state(request.table_number)?;
    let receiver = broadcast.subscribe_table_messages(request.tournament_id, request.table_number);
    Ok(ObserveTableResponse { receiver, table_state })
}


#[cfg(test)]
mod tests {
}
