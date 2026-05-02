use crate::application::AuthInfo;
use crate::application::ApplicationError;
use crate::application::EventBus;
use crate::application::TournamentRepository;

use crate::domain::Nickname;
use crate::domain::TournamentEvent;
use crate::domain::TournamentId;

use uuid::Uuid;


pub trait RegisterPlayer {
    fn register_player(&self, tournament_id: Uuid, auth_info: &AuthInfo) -> Result<(), ApplicationError>;
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

impl<Repository: TournamentRepository> RegisterPlayer for RegisterPlayerService<Repository> {
    fn register_player(&self, tournament_id: Uuid, auth_info: &AuthInfo) -> Result<(), ApplicationError> {
        let player_id = auth_info.expect_participant()?;
        let nickname = Nickname::new(auth_info.given_name())?;
        let mut tournament = self.repository.load_tournament(TournamentId::from_uuid(tournament_id))?;
        tournament.register_player(player_id, nickname)?;
        let events = self.repository.save_tournament(tournament)?;
        log::info!("{:} registered in tournament {:}", auth_info.given_name(), tournament_id);
        self.event_bus.send(events);
        Ok(())
    }
}
