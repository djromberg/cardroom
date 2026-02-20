use crate::application::*;

use crate::domain::AccessTableMessageBroadcast;
use crate::domain::AccessTournaments;


#[derive(Debug)]
pub struct ServiceProvider<Repository: AccessTournaments, Broadcast: AccessTableMessageBroadcast> {
    repository: Repository,
    broadcast: Broadcast,
}

impl<Repository: AccessTournaments, Broadcast: AccessTableMessageBroadcast> ServiceProvider<Repository, Broadcast> {
    pub fn new(repository: Repository, broadcast: Broadcast) -> Self {
        Self { repository, broadcast }
    }
}


impl<Repository: AccessTournaments, Broadcast: AccessTableMessageBroadcast> FindTournaments for ServiceProvider<Repository, Broadcast> {
    fn find_tournaments(&self, request: FindTournamentsRequest, auth_info: &AuthInfo) -> Result<FindTournamentsResponse, FindTournamentsError> {
        find_tournaments(request, auth_info, &self.repository)
    }
}


impl<Repository: AccessTournaments, Broadcast: AccessTableMessageBroadcast> CreateTournament for ServiceProvider<Repository, Broadcast> {
    fn create_tournament(&mut self, request: CreateTournamentRequest, auth_info: &AuthInfo) -> Result<CreateTournamentResponse, CreateTournamentError> {
        create_tournament(request, auth_info, &mut self.repository)
    }
}


impl<Repository: AccessTournaments, Broadcast: AccessTableMessageBroadcast> JoinTournament for ServiceProvider<Repository, Broadcast> {
    fn join_tournament(&mut self, request: JoinTournamentRequest, auth_info: &AuthInfo) -> Result<JoinTournamentResponse, JoinTournamentError> {
        join_tournament(request, auth_info, &mut self.repository, &self.broadcast)
    }
}


impl<Repository: AccessTournaments, Broadcast: AccessTableMessageBroadcast> ObserveTable for ServiceProvider<Repository, Broadcast> {
    fn observe_table(&mut self, request: ObserveTableRequest, auth_info: &AuthInfo) -> Result<ObserveTableResponse, ObserveTableError> {
        observe_table(request, auth_info, &self.repository, &mut self.broadcast)
    }
}
