use super::error::DomainError;
use super::event::TournamentEvent;
use super::event::TournamentEventType;
use super::player::TournamentPlayer;
use super::table::TournamentTable;

use uuid::Uuid;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TournamentId(Uuid);

impl TournamentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}


#[derive(Debug, Clone)]
pub struct Tournament {
    id: TournamentId,
    stage: TournamentStage,
    tables: Vec<TournamentTable>,
    events: Vec<TournamentEvent>,
}

impl Tournament {
    pub fn new(table_count: u16, table_seat_count: u8) -> Self {
        let mut tables = vec![];
        for _ in 0..table_count {
            let table = TournamentTable::new(table_seat_count);
            tables.push(table);
        }
        Self {
            id: TournamentId::new(),
            stage: TournamentStage::Registration,
            tables,
            events: vec![]
        }
    }

    pub fn id(&self) -> TournamentId {
        self.id
    }

    pub fn register_player(&mut self, player_id: Uuid, nickname: String) -> Result<(), DomainError> {
        if self.stage.is_registration_allowed() {
            let player = TournamentPlayer::new(player_id, nickname, 1500)?;
            self.seat_player(player);
            if self.is_ready_to_start() {
                self.start();
            }
            Ok(())
        } else {
            Err(DomainError::TournamentAlreadyStarted)
        }
    }

    pub fn consume_events(&mut self) -> Vec<TournamentEvent> {
        std::mem::take(&mut self.events)
    }

    fn open_table(&mut self, table_seat_count: u8) {
        let table = TournamentTable::new(table_seat_count);
        let table_id = table.id();
        self.tables.push(table);
        self.record_event(
            TournamentEventType::TableOpened {
                table_id,
                seat_count: table_seat_count,
            }
        );
    }

    fn is_ready_to_start(&self) -> bool {
        !self.stage.is_registration_allowed() && self.player_count() == self.seat_count()
    }

    fn table_count(&self) -> u8 {
        self.tables.len() as u8
    }

    fn table_seat_count(&self) -> u8 {
        self.tables[0].seat_count()
    }

    fn seat_count(&self) -> u16 {
        self.table_count() as u16 * self.table_seat_count() as u16
    }

    fn player_count(&self) -> u16 {
        self.tables.iter().map(|table| table.player_count() as u16).product()
    }

    fn start(&mut self) {
        self.stage = TournamentStage::Running;
        self.record_event(TournamentEventType::TournamentStarted);
    }

    fn seat_player(&mut self, player: TournamentPlayer) {
        let table_index = self.tables.iter().position(|table| table.has_free_seat()).unwrap();
        self.tables[table_index].seat_player(player);
        let table_id = self.tables[table_index].id();
        self.record_event(TournamentEventType::PlayerRegistered { table_id });
    }

    fn record_event(&mut self, event_type: TournamentEventType) {
        self.events.push(
            TournamentEvent { tournament_id: self.id, event_type }
        );
    }
}


#[derive(Debug, Clone)]
enum TournamentStage {
    Registration,
    Running,
    Finished,
}

impl TournamentStage {
    pub fn new() -> Self {
        Self::Registration
    }

    pub fn is_registration_allowed(&self) -> bool {
        matches!(self, Self::Registration)
    }

    pub fn next(&self) -> Self {
        match self {
            Self::Registration => Self::Running,
            Self::Running => Self::Finished,
            Self::Finished => Self::Finished,
        }
    }
}
