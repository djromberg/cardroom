use super::error::DomainError;
use super::event::TournamentEvent;
use super::event::TournamentEventType;
use super::player::PlayerSpec;
use super::player::TournamentPlayer;
use super::table::TableSpec;
use super::table::TournamentTable;

use uuid::Uuid;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TournamentSpec {
    table_count: u16,
    table_spec: TableSpec,
}

impl TournamentSpec {
    pub fn new(table_count: u16, table_spec: TableSpec) -> Result<Self, DomainError> {
        if table_count > 0 {
            Ok(Self { table_count, table_spec })
        } else {
            Err(DomainError::InvalidTournamentSpecification)
        }
    }

    pub fn table_spec(&self) -> TableSpec {
        self.table_spec.clone()
    }

    pub fn player_count(&self) -> u16 {
        self.table_count * self.table_spec.seat_count() as u16
    }
}


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
    spec: TournamentSpec,
    stage: TournamentStage,
    tables: Vec<TournamentTable>,
    ranking: Vec<PlayerSpec>,
    events: Vec<TournamentEvent>,
}

impl Tournament {
    pub fn new(spec: TournamentSpec) -> Self {
        Self {
            id: TournamentId::new(),
            spec,
            stage: TournamentStage::begin(),
            tables: vec![],
            ranking: vec![],
            events: vec![],
        }
    }

    pub fn id(&self) -> TournamentId {
        self.id
    }

    pub fn register_player(&mut self, player_spec: PlayerSpec) -> Result<(), DomainError> {
        if self.stage.is_registration_allowed() {
            self.seat_player(player_spec);
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

    fn seat_player(&mut self, player_spec: PlayerSpec) {
        if self.new_table_required() {
            self.open_table();
        }
        let table = self.tables.last_mut().unwrap();
        let table_id = table.id();
        let stack = 1500u32; // TODO: make this part of the tournament spec
        let player = TournamentPlayer::new(&player_spec, stack);
        table.seat_player(player);
        self.record_event(TournamentEventType::PlayerSeatedAtTable {
            player_spec,
            stack,
            table_id,
        });
    }

    fn new_table_required(&self) -> bool {
        self.tables.last().map_or(true, |table| table.all_seats_taken())
    }

    fn open_table(&mut self) {
        let table_spec = self.spec.table_spec();
        let table = TournamentTable::new(&table_spec);
        let table_id = table.id();
        self.tables.push(table);
        self.record_event(
            TournamentEventType::TableOpened {
                table_id,
                table_spec
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
    pub fn begin() -> Self {
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
