use crate::application::AuthInfo;
use crate::application::AuthRole;
use crate::application::ApplicationError;
use crate::application::TournamentEventBus;
use crate::application::TournamentRepository;
use crate::domain::TableSpecification;
use crate::domain::Tournament;
use crate::domain::TournamentId;
use crate::domain::TournamentSpecification;

use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTournamentRequest {
    pub table_count: u8,
    pub table_seat_count: u8,
}


#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTournamentResponse {
    pub tournament_id: TournamentId,
}


pub trait CreateTournament {
    fn create_tournament(&mut self, request: CreateTournamentRequest, auth_info: &AuthInfo) -> Result<CreateTournamentResponse, ApplicationError>;
}


#[derive(Debug)]
pub struct CreateTournamentService<Repository> {
    repository: Repository,
    event_bus: TournamentEventBus,
}

impl<Repository: TournamentRepository> CreateTournamentService<Repository> {
    pub fn new(repository: Repository, event_bus: TournamentEventBus) -> Self {
        Self { repository, event_bus }
    }
}

impl<Repository: TournamentRepository> CreateTournament for CreateTournamentService<Repository> {
    fn create_tournament(&mut self, request: CreateTournamentRequest, auth_info: &AuthInfo) -> Result<CreateTournamentResponse, ApplicationError> {
        let account_id = auth_info.expect_role(AuthRole::Organizer)?;
        let table_spec = TableSpecification::new(request.table_seat_count)?;
        let tournament_spec = TournamentSpecification::new(request.table_count, table_spec)?;
        let tournament_id = TournamentId::new();
        let tournament = Tournament::new(tournament_id, &tournament_spec);
        let events = self.repository.save_tournament(tournament)?;
        self.event_bus.send(events);
        Ok(CreateTournamentResponse { tournament_id })
    }
}
