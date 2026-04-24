use crate::application::TableRepository;
use crate::application::TableRepositoryTransaction;
use crate::application::RepositoryError;
use crate::application::ApplicationError;
use crate::application::TournamentRepository;
use crate::domain::Table;
use crate::domain::TableEvent;
use crate::domain::TableId;
use crate::domain::Tournament;
use crate::domain::TournamentEvent;
use crate::domain::TournamentId;

use std::collections::HashMap;
use std::sync::Mutex;


#[derive(Debug)]
struct InMemoryTableDatabase {
    tables: HashMap<TableId, Table>,
}

impl InMemoryTableDatabase {
    fn new() -> Self {
        Self { tables: HashMap::new() }
    }

    fn load(&self, table_id: TableId) -> Option<Table> {
        if let Some(table) = self.tables.get(&table_id) {
            Some(table.clone())
        } else {
            None
        }
    }

    fn save(&mut self, table: Table) {
        _ = self.tables.insert(table.id(), table);
    }
}


#[derive(Debug)]
struct InMemoryTableTransaction<'a> {
    database: &'a mut InMemoryTableDatabase,
    events: Vec<TableEvent>,
}

impl<'a> InMemoryTableTransaction<'a> {
    pub fn new(database: &'a mut InMemoryTableDatabase) -> Self {
        Self { database, events: vec![] }
    }

    pub fn consume_events(&mut self) -> Vec<TableEvent> {
        std::mem::take(&mut self.events)
    }
}

impl TableRepositoryTransaction for InMemoryTableTransaction<'_> {
    fn load_table(&self, table_id: TableId) -> Result<Table, RepositoryError> {
        self.database.load(table_id).ok_or_else(|| RepositoryError::ResourceNotFound)
    }

    fn save_table(&mut self, table: Table) -> Result<(), RepositoryError> {
        let mut table = table; // TODO: maybe change consume_events() to events() and copy them
        self.events.extend(table.consume_events());
        // TODO: think about optimistic locking approach
        self.database.save(table);
        Ok(())
    }
}


#[derive(Debug)]
pub struct InMemoryTableRepository {
    database: Mutex<InMemoryTableDatabase>,
}

impl InMemoryTableRepository {
    pub fn new() -> Self {
        Self { database: Mutex::new(InMemoryTableDatabase::new()) }
    }
}

impl TableRepository for InMemoryTableRepository {
    fn with_tx<F>(&self, f: F) -> Result<Vec<TableEvent>, ApplicationError>
        where
            F: FnOnce(&mut dyn TableRepositoryTransaction) -> Result<(), ApplicationError> {
        let mut db = self.database.lock().unwrap();
        let mut tx = InMemoryTableTransaction::new(&mut db);
        f(&mut tx)?;
        // here is the place where real transaction successfully commit or will be rolled back
        Ok(tx.consume_events())
    }
}


#[derive(Debug)]
struct InMemoryTournamentDatabase {
    tournaments: HashMap<TournamentId, Tournament>,
}

impl InMemoryTournamentDatabase {
    fn new() -> Self {
        Self { tournaments: HashMap::new() }
    }

    fn load(&self, tournament_id: TournamentId) -> Option<Tournament> {
        if let Some(tournament) = self.tournaments.get(&tournament_id) {
            Some(tournament.clone())
        } else {
            None
        }
    }

    fn save(&mut self, tournament: Tournament) {
        _ = self.tournaments.insert(tournament.id(), tournament);
    }
}


#[derive(Debug)]
pub struct InMemoryTournamentRepository {
    database: Mutex<InMemoryTournamentDatabase>,
}

impl InMemoryTournamentRepository {
    pub fn new() -> Self {
        Self { database: Mutex::new(InMemoryTournamentDatabase::new()) }
    }
}

impl TournamentRepository for InMemoryTournamentRepository {
    fn load_tournament(&self, id: TournamentId) -> Result<Tournament, RepositoryError> {
        let db = self.database.lock().unwrap();
        db.load(id).ok_or_else(|| RepositoryError::ResourceNotFound)
    }

    fn save_tournament(&self, tournament: Tournament) -> Result<Vec<TournamentEvent>, RepositoryError> {
        let mut db = self.database.lock().unwrap();
        let mut tournament = tournament; // TODO: change consume_events(), see TableRepo
        let events = tournament.consume_events();
        db.save(tournament);
        Ok(events)
    }
}


#[cfg(test)]
mod tests {
    use crate::domain::{TableSpecification, TournamentId, TournamentSpecification};

    use super::*;

    #[test]
    fn multiple_table_transaction() {
        let repository = InMemoryTableRepository::new();
        let events = save_multiple_tables(&repository).unwrap();
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn tournament_saving() {
        let mut repository = InMemoryTournamentRepository::new();
        let table_spec = TableSpecification::new(5).unwrap();
        let tournament_spec = TournamentSpecification::new(5, table_spec).unwrap();
        let tournament = Tournament::new(TournamentId::new(), &tournament_spec);
        let events = repository.save_tournament(tournament).unwrap();
        assert_eq!(events.len(), 1);
    }

    fn save_multiple_tables<R: TableRepository>(repository: &R) -> Result<Vec<TableEvent>, ApplicationError> {
        repository.with_tx(|tx| {
            let table_spec = TableSpecification::new(9)?;
            let tournament_id = TournamentId::new();
            for _ in 0..3 {
                let table = Table::new(TableId::new(), tournament_id, &table_spec);
                tx.save_table(table)?;
            }
            Ok(())
        })
    }
}
