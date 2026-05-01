use tokio::sync::broadcast::Receiver;

use crate::application::CreateTournament;
use crate::application::CreateTournamentService;
use crate::application::EventBus;
use crate::application::OpenTables;
use crate::application::OpenTablesService;
use crate::application::TableRepository;
use crate::application::TournamentRepository;


use crate::domain::TableEvent;
use crate::domain::TournamentEvent;


pub trait ProvideTournamentServices {
    type CreateTournamentServiceType: CreateTournament + Clone + Send + Sync + 'static;

    fn create_tournament_service(&self) -> Self::CreateTournamentServiceType;
}

pub trait ProvidePrivateTableServices {
    type OpenTablesServiceType: OpenTables + Clone + Send + Sync + 'static;

    fn open_tables_service(&self) -> Self::OpenTablesServiceType;
}

pub trait ProvideTournamentEvents {
    fn subscribe_events(&self) -> Receiver<TournamentEvent>;
}


#[derive(Debug, Clone)]
pub struct TournamentServiceProvider<Repository> {
    repository: Repository,
    event_bus: EventBus<TournamentEvent>,
}

impl<Repository> TournamentServiceProvider<Repository> {
    pub fn new(repository: Repository) -> Self {
        Self { repository, event_bus: EventBus::new() }
    }
}


impl<Repository: TournamentRepository> ProvideTournamentServices for TournamentServiceProvider<Repository> {
    type CreateTournamentServiceType = CreateTournamentService<Repository>;

    fn create_tournament_service(&self) -> Self::CreateTournamentServiceType {
        CreateTournamentService::new(self.repository.clone(), self.event_bus.clone())
    }
}


impl<Repository: TournamentRepository> ProvideTournamentEvents for TournamentServiceProvider<Repository> {
    fn subscribe_events(&self) -> Receiver<TournamentEvent> {
        self.event_bus.subscribe()
    }
}





#[derive(Debug, Clone)]
pub struct TableServiceProvider<Repository> {
    repository: Repository,
    event_bus: EventBus<TableEvent>,
}

impl<Repository> TableServiceProvider<Repository> {
    pub fn new(repository: Repository) -> Self {
        Self { repository, event_bus: EventBus::new() }
    }
}


impl<Repository: TableRepository> ProvidePrivateTableServices for TableServiceProvider<Repository> {
    type OpenTablesServiceType = OpenTablesService<Repository>;

    fn open_tables_service(&self) -> Self::OpenTablesServiceType {
        OpenTablesService::new(self.repository.clone(), self.event_bus.clone())
    }
}
