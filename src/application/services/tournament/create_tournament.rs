use crate::application::AuthInfo;
use crate::application::ApplicationError;
use crate::application::EventBus;
use crate::application::TournamentRepository;

use crate::domain::TableSpecification;
use crate::domain::Tournament;
use crate::domain::TournamentEvent;
use crate::domain::TournamentId;
use crate::domain::TournamentSpecification;

use async_trait::async_trait;
use uuid::Uuid;


#[async_trait]
pub trait CreateTournament {
    async fn create_tournament(&self, table_count: u8, table_seat_count: u8, auth_info: &AuthInfo) -> Result<Uuid, ApplicationError>;
}


#[derive(Debug, Clone)]
pub struct CreateTournamentService<Repository> {
    repository: Repository,
    event_bus: EventBus<TournamentEvent>,
}

impl<Repository: TournamentRepository> CreateTournamentService<Repository> {
    pub fn new(repository: Repository, event_bus: EventBus<TournamentEvent>) -> Self {
        Self { repository, event_bus }
    }
}

#[async_trait]
impl<Repository: TournamentRepository> CreateTournament for CreateTournamentService<Repository> {
    async fn create_tournament(&self, table_count: u8, table_seat_count: u8, auth_info: &AuthInfo) -> Result<Uuid, ApplicationError> {
        let _account_id = auth_info.expect_organizer()?;
        let table_spec = TableSpecification::new(table_seat_count)?;
        let tournament_spec = TournamentSpecification::new(table_count, table_spec)?;
        let tournament_id = TournamentId::new();
        let tournament = Tournament::new(tournament_id, &tournament_spec);
        let events = self.repository.save_tournament(tournament).await?;
        log::info!("Tournament {:?} created by {:}", tournament_id, auth_info.given_name());
        self.event_bus.send(events);
        Ok(tournament_id.uuid())
    }
}
