use crate::application::ApplicationError;
use crate::application::TableRepository;
use crate::application::TournamentRepository;

use crate::domain::Table;
use crate::domain::TableEvent;
use crate::domain::TableId;
use crate::domain::TableSpecification;
use crate::domain::TournamentEvent;
use crate::domain::Tournament;
use crate::domain::TournamentId;
use crate::domain::TournamentSpecification;


#[derive(Debug)]
pub struct TournamentService<Repository> {
    repository: Repository,
}

impl<Repository: TournamentRepository> TournamentService<Repository> {
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }

    pub fn create_tournament(&self, table_count: u8, table_seat_count: u8) -> Result<Vec<TournamentEvent>, ApplicationError> {
        let table_spec = TableSpecification::new(table_seat_count)?;
        let tournament_spec = TournamentSpecification::new(table_count, table_spec)?;
        let tournament_id = TournamentId::new();
        let tournament = Tournament::new(tournament_id, &tournament_spec);
        let events = self.repository.save_tournament(tournament)?;
        Ok(events)
    }
}


#[derive(Debug)]
pub struct TableService<Repository> {
    repository: Repository,
}

impl<Repository: TableRepository> TableService<Repository>
{
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }

    pub fn create_tables(&self, tournament_id: TournamentId, table_spec: &TableSpecification, table_ids: &Vec<TableId>) -> Result<Vec<TableEvent>, ApplicationError> {
        let events = self.repository.with_tx(|tx| {
            for table_id in table_ids {
                let table = Table::new(*table_id, tournament_id, &table_spec);
                tx.save_table(table)?;
            }
            Ok(())
        })?;
        Ok(events)
    }
}


#[cfg(test)]
mod tests {
    use crate::{domain::TableEventType, infrastructure::InMemoryTableRepository};

    use super::*;

    #[test]
    fn table_service() {
        let repository = InMemoryTableRepository::new();
        let service = TableService::new(repository);
        let tournament_id = TournamentId::new();
        let table_ids = vec![TableId::new(), TableId::new()];
        let table_spec = TableSpecification::new(5).unwrap();
        let events = service.create_tables(tournament_id, &table_spec, &table_ids).unwrap();
        assert_eq!(events, vec![
            TableEvent {
                event_type: TableEventType::TableOpened { seat_count: 5 },
                table_id: table_ids[0],
                tournament_id,
            },
            TableEvent {
                event_type: TableEventType::TableOpened { seat_count: 5 },
                table_id: table_ids[1],
                tournament_id,
            },
        ])
    }
}
