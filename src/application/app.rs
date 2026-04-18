use std::collections::HashMap;

use crate::application::ApplicationError;
use crate::application::TableRepository;
use crate::application::TableSpectator;
use crate::application::TournamentRepository;

use crate::domain::Table;
use crate::domain::TableEvent;
use crate::domain::TableId;
use crate::domain::TableSpecification;
use crate::domain::TournamentEvent;
use crate::domain::Nickname;
use crate::domain::PlayerId;
use crate::domain::Tournament;
use crate::domain::TournamentEventType;
use crate::domain::TournamentId;
use crate::domain::TournamentSpecification;


pub struct Application<R1, R2, Spectator> {
    tournament_repository: R1,
    table_repository: R2,
    tournament_events: Vec<TournamentEvent>,
    table_events: Vec<TableEvent>,
    spectators: HashMap<TableId, Vec<Spectator>>,
}

impl<R1: TournamentRepository, R2: TableRepository, Spectator: TableSpectator> Application<R1, R2, Spectator>
{
    pub fn new(tournament_repository: R1, table_repository: R2) -> Self {
        Self {
            tournament_repository,
            table_repository,
            tournament_events: vec![],
            table_events: vec![],
            spectators: HashMap::new()
        }
    }

    pub fn create_tournament(&mut self, table_count: u8, table_seat_count: u8) -> Result<TournamentId, ApplicationError> {
        let table_spec = TableSpecification::new(table_seat_count)?;
        let tournament_spec = TournamentSpecification::new(table_count, table_spec)?;
        let tournament_id = TournamentId::new();
        let tournament = Tournament::new(tournament_id, &tournament_spec);
        let events = self.tournament_repository.save_tournament(tournament)?;
        self.tournament_events.extend(events);
        Ok(tournament_id)
    }

    pub fn register_player(&mut self, tournament_id: TournamentId, player_id: PlayerId, nickname: impl Into<String>) -> Result<(), ApplicationError> {
        let nickname = Nickname::new(nickname)?;
        let mut tournament = self.tournament_repository.load_tournament(tournament_id)?;
        tournament.register_player(player_id, nickname)?;
        let events = self.tournament_repository.save_tournament(tournament)?;
        self.tournament_events.extend(events);
        Ok(())
    }

    pub fn act_on_table(&mut self, table_id: TableId, player_id: PlayerId, action: u8) -> Result<(), ApplicationError> {
        let mut table = self.table_repository.load_table(table_id)?;
        table.act(player_id, action)?;
        let events = self.table_repository.save_table(table)?;
        self.table_events.extend(events);
        Ok(())
    }

    pub fn process_events(&mut self) {
        self.process_tournament_events();
        self.process_table_events();
    }

    fn process_tournament_events(&mut self) {
        let mut unprocessed_events = vec![];
        for event in &self.tournament_events {
            match &event.event_type {
                TournamentEventType::TournamentCreated { table_spec, table_ids } => {
                    for table_id in table_ids {
                        let table = Table::new(*table_id, &table_spec);
                        if let Ok(events) = self.table_repository.save_table(table) {
                            self.table_events.extend(events);
                        } else {
                            unprocessed_events.push(event);
                        }
                    }
                },
                TournamentEventType::PlayerRegistered { table_id, player_info } => {
                    if let Ok(mut table) = self.table_repository.load_table(*table_id) {
                        table.seat_player(player_info.clone());
                    } else {
                        unprocessed_events.push(event);
                    }
                }
                _ => {}
            }
        }
    }

    fn process_table_events(&mut self) {
    }

    fn notify_spectators(&self, event: &TableEvent) {
    }
}
