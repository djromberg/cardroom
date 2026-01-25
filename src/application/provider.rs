use crate::application::*;

use crate::domain::AccessTournaments;


#[derive(Debug)]
pub struct ServiceProvider<Repository: AccessTournaments> {
    repository: Repository,
}

impl<Repository: AccessTournaments> ServiceProvider<Repository> {
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }
}


impl<Repository: AccessTournaments> CreateTournament for ServiceProvider<Repository> {
    fn create_tournament(&mut self, request: CreateTournamentRequest, auth_info: &AuthInfo) -> Result<CreateTournamentResponse, CreateTournamentError> {
        create_tournament(request, auth_info, &mut self.repository)
    }
}
