use crate::application::*;

use crate::domain::AccessTableEventBroadcast;
use crate::domain::AccessTournaments;


#[derive(Debug)]
pub struct ServiceProvider<Repository: AccessTournaments, Broadcast: AccessTableEventBroadcast> {
    repository: Repository,
    broadcast: Broadcast,
}

impl<Repository: AccessTournaments, Broadcast: AccessTableEventBroadcast> ServiceProvider<Repository, Broadcast> {
    pub fn new(repository: Repository, broadcast: Broadcast) -> Self {
        Self { repository, broadcast }
    }
}


impl<Repository: AccessTournaments, Broadcast: AccessTableEventBroadcast> CreateTournament for ServiceProvider<Repository, Broadcast> {
    fn create_tournament(&mut self, request: CreateTournamentRequest, auth_info: &AuthInfo) -> Result<CreateTournamentResponse, CreateTournamentError> {
        create_tournament(request, auth_info, &mut self.repository, &mut self.broadcast)
    }
}


impl<Repository: AccessTournaments, Broadcast: AccessTableEventBroadcast> JoinTournament for ServiceProvider<Repository, Broadcast> {
    fn join_tournament(&mut self, request: JoinTournamentRequest, auth_info: &AuthInfo) -> Result<JoinTournamentResponse, JoinTournamentError> {
        join_tournament(request, auth_info, &mut self.repository, &self.broadcast)
    }
}


impl<Repository: AccessTournaments, Broadcast: AccessTableEventBroadcast> ObserveTable for ServiceProvider<Repository, Broadcast> {
    fn observe_table(&self, request: ObserveTableRequest, auth_info: &AuthInfo) -> Result<ObserveTableResponse, ObserveTableError> {
        observe_table(request, auth_info, &self.broadcast)
    }
}
