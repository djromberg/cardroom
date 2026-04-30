use crate::application::CreateTournament;
use crate::application::CreateTournamentService;
use crate::application::TableRepository;
use crate::application::TournamentRepository;
use crate::application::OpenTablesService;
use crate::application::ApplicationState;

use crate::domain::TableEvent;
use crate::domain::TableEventType;
use crate::domain::TournamentEvent;
use crate::domain::TournamentEventType;


pub trait ProcessEvents {
    fn process_tournament_events(&self);
    fn process_table_events(&self);
}


pub trait ProvideServices {
    type CreateTournamentServiceType: CreateTournament + Clone + Send + Sync + 'static;

    fn create_tournament_service(&self) -> Self::CreateTournamentServiceType;
}


#[derive(Debug)]
pub struct ServiceProvider<ToR, TaR> {
    tournaments: ApplicationState<ToR, TournamentEvent>,
    tables: ApplicationState<TaR, TableEvent>,
}

impl<ToR: TournamentRepository, TaR: TableRepository> ServiceProvider<ToR, TaR> {
    pub fn new(tournament_repository: ToR, table_repository: TaR) -> Self {
        let tournaments = ApplicationState::new(tournament_repository);
        let tables = ApplicationState::new(table_repository);
        Self { tournaments, tables }
    }
}


impl<ToR: TournamentRepository, TaR> ProvideServices for ServiceProvider<ToR, TaR> {
    type CreateTournamentServiceType = CreateTournamentService<ToR>;
    fn create_tournament_service(&self) -> Self::CreateTournamentServiceType {
        CreateTournamentService::new(self.tournaments.repository(), self.tournaments.event_bus())
    }
}


impl<ToR: TournamentRepository, TaR: TableRepository> ProcessEvents for ServiceProvider<ToR, TaR> {
    fn process_tournament_events(&self) {
        // TODO: make this abortable
        while let Some(event) = self.tournaments.receive_event() {
            match event.event_type {
                TournamentEventType::TournamentCreated { table_spec, table_ids } => {
                    log::info!("Tournament created, opening tables ...");
                    let service = OpenTablesService::new(self.tables.repository(), self.tables.event_bus());
                    service.open_tables(event.tournament_id, table_ids, table_spec).unwrap(); // TODO: handle errors
                },
                _ => {},
            }
        }
    }

    fn process_table_events(&self) {
        // // TODO: make this abortable
        // loop {
        //     let event = self.tables.receive_event();
        //     match event.event_type {
        //         TableEventType::TableOpened { seat_count } => {
        //             log::info!("Table opened for tournament {:?}", event.tournament_id);
        //         }
        //         _ => {}
        //     }
        // }
    }
}
