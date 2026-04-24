use crate::application::ApplicationError;
use crate::application::TableRepository;
use crate::application::TournamentRepository;

use crate::domain::Table;
use crate::domain::TableEvent;
use crate::domain::TableId;
use crate::domain::TableSpecification;
use crate::domain::TournamentEvent;
use crate::domain::Tournament;
use crate::domain::TournamentEventType;
use crate::domain::TournamentId;
use crate::domain::TournamentSpecification;


pub struct Service<TourR, TablR> {
    tournament_repository: TourR,
    table_repository: TablR,
}

impl<TourR: TournamentRepository, TablR: TableRepository> Service<TourR, TablR>
{
    pub fn new(tournament_repository: TourR, table_repository: TablR) -> Self {
        Self {
            tournament_repository,
            table_repository,
        }
    }

    pub fn create_tournament(&self, table_count: u8, table_seat_count: u8) -> Result<TournamentId, ApplicationError> {
        let table_spec = TableSpecification::new(table_seat_count)?;
        let tournament_spec = TournamentSpecification::new(table_count, table_spec)?;
        let tournament_id = TournamentId::new();
        let tournament = Tournament::new(tournament_id, &tournament_spec);
        let events = self.tournament_repository.save_tournament(tournament)?;
        self.process_tournament_events(events);
        Ok(tournament_id)
    }

    fn create_tournament_tables(&self, tournament_id: TournamentId, table_spec: TableSpecification, table_ids: Vec<TableId>) -> Result<(), ApplicationError> {
        let events = self.table_repository.with_tx(|tx| {
            for table_id in table_ids {
                let table = Table::new(table_id, tournament_id, &table_spec);
                tx.save_table(table)?;
            }
            Ok(())
        })?;
        self.process_table_events(events);
        Ok(())
    }

    fn process_table_events(&self, events: Vec<TableEvent>) {
        for event in events {
            log::debug!("{:?}", event);
        }
    }

    fn process_tournament_events(&self, events: Vec<TournamentEvent>) {
        for event in events {
            match event.event_type {
                TournamentEventType::TournamentCreated { table_spec, table_ids } => {
                    // TODO: in case this fails, keep tournament events for retry...
                    _ = self.create_tournament_tables(event.tournament_id, table_spec, table_ids);
                },
                _ => {}
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::infrastructure::{InMemoryTableRepository, InMemoryTournamentRepository};

    use super::*;

    #[test]
    fn create_tournament_opens_tables() {
        let tournament_repo = InMemoryTournamentRepository::new();
        let table_repo = InMemoryTableRepository::new();
        let service = Service::new(tournament_repo, table_repo);
        let tournament_id = service.create_tournament(4, 5).unwrap();
    }
}
