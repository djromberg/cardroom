use crate::application::TableRepository;
use crate::application::RepositoryError;
use crate::application::TournamentRepository;
use crate::domain::Table;
use crate::domain::TableEvent;
use crate::domain::TableId;
use crate::domain::Tournament;
use crate::domain::TournamentEvent;
use crate::domain::TournamentId;

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex as AsyncMutex;


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


#[derive(Debug, Clone)]
pub struct InMemoryTableRepository {
    database: Arc<AsyncMutex<InMemoryTableDatabase>>,
}

impl InMemoryTableRepository {
    pub fn new() -> Self {
        Self { database: Arc::new(AsyncMutex::new(InMemoryTableDatabase::new())) }
    }
}


#[async_trait]
impl TableRepository for InMemoryTableRepository {
    async fn load_table(&self, table_id: TableId) -> Result<Table, RepositoryError> {
        let db = self.database.lock().await;
        let x = db.load(table_id).unwrap();
        Ok(x)
    }

    async fn save_table(&self, mut table: Table) -> Result<Vec<TableEvent>, RepositoryError> {
        let mut db = self.database.lock().await;
        let events = table.consume_events();
        db.save(table);
        Ok(events)
    }

    async fn save_tables(&self, tables: Vec<Table>) -> Result<Vec<TableEvent>, RepositoryError> {
        // TODO: in reality this would be a transaction
        let mut db = self.database.lock().await;
        let mut events = vec![];
        for mut table in tables {
            events.extend(table.consume_events());
            db.save(table);
        }
        Ok(events)
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


#[derive(Debug, Clone)]
pub struct InMemoryTournamentRepository {
    // database: Arc<Mutex<InMemoryTournamentDatabase>>,
    database: Arc<AsyncMutex<InMemoryTournamentDatabase>>,
}

impl InMemoryTournamentRepository {
    pub fn new() -> Self {
        Self { database: Arc::new(AsyncMutex::new(InMemoryTournamentDatabase::new())) }
    }
}

#[async_trait]
impl TournamentRepository for InMemoryTournamentRepository {
    async fn load_tournament(&self, id: TournamentId) -> Result<Tournament, RepositoryError> {
        let db = self.database.lock().await;
        db.load(id).ok_or_else(|| RepositoryError::ResourceNotFound)
    }

    async fn save_tournament(&self, tournament: Tournament) -> Result<Vec<TournamentEvent>, RepositoryError> {
        // let mut db = self.database.lock().unwrap();
        let mut db = self.database.lock().await;
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

    #[tokio::test]
    async fn multiple_table_transaction() {
        let repository = InMemoryTableRepository::new();
        let tournament_id = TournamentId::new();
        let table_ids = vec![TableId::new(), TableId::new(), TableId::new()];
        let tables = create_tables(tournament_id, table_ids);
        let events = repository.save_tables(tables).await.unwrap();
        assert_eq!(events.len(), 3);
    }

    #[tokio::test]
    async fn tournament_saving() {
        let repository = InMemoryTournamentRepository::new();
        let table_spec = TableSpecification::new(5).unwrap();
        let tournament_spec = TournamentSpecification::new(5, table_spec).unwrap();
        let tournament = Tournament::new(TournamentId::new(), &tournament_spec);
        let events = repository.save_tournament(tournament).await.unwrap();
        assert_eq!(events.len(), 1);
    }

    fn create_tables(tournament_id: TournamentId, table_ids: Vec<TableId>) -> Vec<Table> {
        let table_spec = TableSpecification::new(9).unwrap();
        let mut tables = vec![];
        for table_id in table_ids {
            let table = Table::new(table_id, tournament_id, &table_spec);
            tables.push(table);
        }
        return tables
    }
}
