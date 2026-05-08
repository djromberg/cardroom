
use crate::application::AccessStuff;
use crate::application::CreateTournament;
use crate::application::CreateTournamentService;
use crate::application::DoStuff;
use crate::application::DoStuffService;
use crate::application::EventBus;
use crate::application::RegisterPlayer;
use crate::application::RegisterPlayerService;
use crate::application::TournamentRepository;

use crate::domain::TournamentEvent;

use tokio::sync::broadcast::Receiver;


pub trait ProvideTournamentServices {
    type CreateTournamentServiceType: CreateTournament + Clone + Send + Sync + 'static;
    type RegisterPlayerServiceType: RegisterPlayer + Clone + Send + Sync + 'static;
    type DoStuffServiceType: DoStuff;

    fn create_tournament_service(&self) -> Self::CreateTournamentServiceType;
    fn register_player_service(&self) -> Self::RegisterPlayerServiceType;
    fn do_stuff_service(&self) -> Self::DoStuffServiceType;
}


pub trait ProvideTournamentEvents {
    fn subscribe_events(&self) -> Receiver<TournamentEvent>;
}


#[derive(Debug, Clone)]
pub struct TournamentServiceProvider<Repository, Accessor> {
    repository: Repository,
    accessor: Accessor,
    event_bus: EventBus<TournamentEvent>,
}

impl<Repository, Accessor> TournamentServiceProvider<Repository, Accessor> {
    pub fn new(repository: Repository, accessor: Accessor) -> Self {
        Self { repository, accessor, event_bus: EventBus::new() }
    }
}


impl<Repository: TournamentRepository, Accessor: AccessStuff> ProvideTournamentServices for TournamentServiceProvider<Repository, Accessor> {
    type CreateTournamentServiceType = CreateTournamentService<Repository>;
    type RegisterPlayerServiceType = RegisterPlayerService<Repository>;
    type DoStuffServiceType = DoStuffService<Accessor>;

    fn create_tournament_service(&self) -> Self::CreateTournamentServiceType {
        CreateTournamentService::new(self.repository.clone(), self.event_bus.clone())
    }

    fn register_player_service(&self) -> Self::RegisterPlayerServiceType {
        RegisterPlayerService::new(self.repository.clone(), self.event_bus.clone())
    }

    fn do_stuff_service(&self) -> Self::DoStuffServiceType {
        DoStuffService::new(self.accessor.clone(), self.event_bus.clone())
    }
}


impl<Repository: TournamentRepository, Accessor: AccessStuff> ProvideTournamentEvents for TournamentServiceProvider<Repository, Accessor> {
    fn subscribe_events(&self) -> Receiver<TournamentEvent> {
        self.event_bus.subscribe()
    }
}
