use crate::application::CreateTournament;
use crate::application::CreateTournamentService;
use crate::application::EventBus;
use crate::application::RegisterPlayer;
use crate::application::RegisterPlayerService;
use crate::application::TournamentRepository;

use crate::domain::TournamentEvent;

use tokio::sync::broadcast::Receiver;


pub trait ProvideTournamentServices {
    type CreateTournamentServiceType: CreateTournament + Clone + Send + Sync + 'static;
    type RegisterPlayerServiceType: RegisterPlayer + Clone + Send + Sync + 'static;

    fn create_tournament_service(&self) -> Self::CreateTournamentServiceType;
    fn register_player_service(&self) -> Self::RegisterPlayerServiceType;
}


pub trait ProvideTournamentEvents {
    fn subscribe_events(&self) -> Receiver<TournamentEvent>;
}


#[derive(Debug)]
pub struct TournamentServiceProvider<Repository> {
    repository: Repository,
    event_bus: EventBus<TournamentEvent>,
}

impl<Repository> TournamentServiceProvider<Repository> {
    pub fn new(repository: Repository) -> Self {
        Self { repository: repository, event_bus: EventBus::new() }
    }
}


impl<Repository: TournamentRepository> ProvideTournamentServices for TournamentServiceProvider<Repository> {
    type CreateTournamentServiceType = CreateTournamentService<Repository>;
    type RegisterPlayerServiceType = RegisterPlayerService<Repository>;

    fn create_tournament_service(&self) -> Self::CreateTournamentServiceType {
        CreateTournamentService::new(self.repository.clone(), self.event_bus.clone())
    }

    fn register_player_service(&self) -> Self::RegisterPlayerServiceType {
        RegisterPlayerService::new(self.repository.clone(), self.event_bus.clone())
    }
}


impl<Repository: TournamentRepository> ProvideTournamentEvents for TournamentServiceProvider<Repository> {
    fn subscribe_events(&self) -> Receiver<TournamentEvent> {
        self.event_bus.subscribe()
    }
}
