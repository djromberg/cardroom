use crate::domain::LoadTournament;
use crate::domain::LoadTournamentError;
use crate::domain::SaveTournament;
use crate::domain::SaveTournamentError;
use crate::domain::QueryTournaments;
use crate::domain::QueryTournamentsError;
use crate::domain::Tournament;

use uuid::Uuid;

use std::collections::HashMap;


#[derive(Debug)]
pub struct InMemoryTournamentRepository {
    tournaments: HashMap<Uuid, Tournament>,
}

impl InMemoryTournamentRepository {
    pub fn new() -> Self {
        Self { tournaments: HashMap::new() }
    }
}


impl LoadTournament for InMemoryTournamentRepository {
    fn load_tournament(&self, tournament_id: Uuid) -> Result<Tournament, LoadTournamentError> {
        println!("[INFRASTRUCTURE] load_tournament");
        if let Some(tournament) = self.tournaments.get(&tournament_id) {
            Ok(tournament.clone())
        } else {
            Err(LoadTournamentError::TournamentNotFound)
        }
    }
}


impl SaveTournament for InMemoryTournamentRepository {
    fn save_tournament(&mut self, tournament: Tournament) -> Result<(), SaveTournamentError> {
        println!("[INFRASTRUCTURE] save_tournament: {:?}", tournament.id());
        self.tournaments.insert(tournament.id(), tournament);
        Ok(())
    }
}


impl QueryTournaments for InMemoryTournamentRepository {
    fn query_tournaments(&self) -> Result<Vec<Tournament>, QueryTournamentsError> {
        println!("[INFRASTRUCTURE] query_tournaments");
        Ok(self.tournaments.values().cloned().collect())
    }
}
