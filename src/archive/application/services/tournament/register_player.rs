use crate::application::AuthInfo;
use crate::application::ApplicationError;
use crate::application::EventBus;
use crate::application::TournamentRepository;

use crate::domain::Nickname;
use crate::domain::TournamentEvent;
use crate::domain::TournamentId;

use async_trait::async_trait;
use uuid::Uuid;


#[async_trait]
pub trait RegisterPlayer {
    async fn register_player(&self, tournament_id: Uuid, auth_info: &AuthInfo) -> Result<(), ApplicationError>;
}


#[derive(Debug, Clone)]
pub struct RegisterPlayerService<Repository> {
    repository: Repository,
    event_bus: EventBus<TournamentEvent>,
}

impl<Repository: TournamentRepository> RegisterPlayerService<Repository> {
    pub fn new(repository: Repository, event_bus: EventBus<TournamentEvent>) -> Self {
        Self { repository, event_bus }
    }
}

#[async_trait]
impl<Repository: TournamentRepository> RegisterPlayer for RegisterPlayerService<Repository> {
    async fn register_player(&self, tournament_id: Uuid, auth_info: &AuthInfo) -> Result<(), ApplicationError> {
        let player_id = auth_info.expect_participant()?;
        let nickname = Nickname::new(auth_info.given_name())?;
        let mut tournament = self.repository.load_tournament(TournamentId::from_uuid(tournament_id)).await?;
        tournament.register_player(player_id, nickname)?;
        let events = self.repository.save_tournament(tournament).await?;
        log::info!("{:} registered in tournament {:}", auth_info.given_name(), tournament_id);
        self.event_bus.send(events);
        Ok(())
    }
}
