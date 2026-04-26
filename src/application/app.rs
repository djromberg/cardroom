use crate::{application::{ApplicationError, ServiceProvider, TableRepository, TournamentRepository, service::{TableService, TournamentService}}, domain::{TableEvent, TableId, TableSpecification, TournamentEvent, TournamentEventType, TournamentId}};


#[derive(Debug)]
pub struct Application<ToR, TaR> {
    tournament_service: TournamentService<ToR>,
    table_service: TableService<TaR>,
}

impl<ToR: TournamentRepository, TaR: TableRepository> Application<ToR, TaR> {
    pub fn new(tournament_repository: ToR, table_repository: TaR) -> Self {
        Self {
            tournament_service: TournamentService::new(tournament_repository),
            table_service: TableService::new(table_repository),
        }
    }

    fn create_tournament_tables(&self, tournament_id: TournamentId, table_spec: &TableSpecification, table_ids: &Vec<TableId>) -> Result<(), ApplicationError> {
        self.process_table_events(
            self.table_service.create_tables(tournament_id, table_spec, table_ids)?
        );
        Ok(())
    }

    fn process_tournament_events(&self, events: Vec<TournamentEvent>) {
        for event in events {
            match event.event_type {
                TournamentEventType::TournamentCreated { table_spec, table_ids } => {
                    self.create_tournament_tables(event.tournament_id, &table_spec, &table_ids);
                },
                _ => {}
            }
        }
    }

    fn process_table_events(&self, events: Vec<TableEvent>) {
        for event in events {
            // notify spectators
        }
    }
}

impl<ToR: TournamentRepository, TaR: TableRepository> ServiceProvider for Application<ToR, TaR> {
    fn print_my_name(&self) {
        println!("MY NAME");
    }

    fn create_tournament(&self, table_count: u8, table_seat_count: u8) -> Result<(), ApplicationError> {
        log::debug!("create tournament with {} tables and {} seats per table", table_count, table_seat_count);
        self.process_tournament_events(
            self.tournament_service.create_tournament(table_count, table_seat_count)?
        );
        Ok(())
    }
}
