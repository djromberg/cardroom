use std::collections::HashMap;

use crate::domain::PlayerInfo;
use crate::domain::DomainError;
use crate::domain::Nickname;
use crate::domain::PlayerId;
use crate::domain::TableId;
use crate::domain::TableSpecification;

use uuid::Uuid;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TournamentId(Uuid);

impl TournamentId {
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn uuid(&self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for TournamentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TournamentSpecification {
    table_count: u8,
    table_spec: TableSpecification,
}

impl TournamentSpecification {
    pub fn new(table_count: u8, table_spec: TableSpecification) -> Result<Self, DomainError> {
        if table_count < 1 || table_count > 100 {
            Err(DomainError::InvalidTournamentSpecification)
        } else {
            Ok(Self { table_count, table_spec })
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
enum TournamentStage {
    Registration,
    Running,
    Finished,
}


#[derive(Debug, Clone)]
pub struct TournamentEvent {
    pub tournament_id: TournamentId,
    pub event_type: TournamentEventType,
}


#[derive(Debug, Clone)]
pub enum TournamentEventType {
    TournamentCreated {
        table_spec: TableSpecification,
        table_ids: Vec<TableId>,
    },
    PlayerRegistered {
        table_id: TableId,
        player_info: PlayerInfo,
    },
    TournamentStarted,
    PlayerKnockedOut {
        player_id: PlayerId,
        rank: u16,
    },
    TableCleared {
        table_id: TableId,
        remaining_player_distribution: HashMap<TableId, Vec<PlayerInfo>>,
    },
    TournamentFinished,
}


#[derive(Debug, Clone)]
pub struct Tournament {
    id: TournamentId,
    stage: TournamentStage,
    tables: Vec<TournamentTable>,
    events: Vec<TournamentEvent>,
}

impl Tournament {
    pub fn new(id: TournamentId, spec: &TournamentSpecification) -> Self {
        let mut tables = vec![];
        for _ in 0..spec.table_count {
            let table = TournamentTable::new(TableId::new(), &spec.table_spec);
            tables.push(table);
        }
        let mut tournament = Self {
            id,
            stage: TournamentStage::Registration,
            tables,
            events: vec![]
        };
        tournament.record_event(TournamentEventType::TournamentCreated {
            table_spec: spec.table_spec,
            table_ids: tournament.table_ids(),
        });
        tournament
    }

    pub fn id(&self) -> TournamentId {
        self.id
    }

    pub fn register_player(&mut self, player_id: PlayerId, nickname: Nickname) -> Result<(), DomainError> {
        if self.stage == TournamentStage::Registration {
            let player_info = PlayerInfo { player_id, nickname, stack: 1500 };
            self.seat_player(player_info);
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

    fn is_ready_to_start(&self) -> bool {
        self.stage == TournamentStage::Registration && self.player_count() == self.seat_count()
    }

    fn table_count(&self) -> u8 {
        self.tables.len() as u8
    }

    fn table_seat_count(&self) -> u8 {
        self.tables[0].seat_count
    }

    fn seat_count(&self) -> u16 {
        self.table_count() as u16 * self.table_seat_count() as u16
    }

    fn player_count(&self) -> u16 {
        self.tables.iter().map(|table| table.player_count()).product()
    }

    fn start(&mut self) {
        self.stage = TournamentStage::Running;
        self.record_event(TournamentEventType::TournamentStarted);
    }

    fn seat_player(&mut self, player_info: PlayerInfo) {
        let table_index = self.tables.iter().position(|table| table.has_free_seat()).unwrap();
        self.tables[table_index].seat_player(player_info.clone());
        let table_id = self.tables[table_index].id();
        self.record_event(TournamentEventType::PlayerRegistered { table_id, player_info });
    }

    fn table_ids(&self) -> Vec<TableId> {
        self.tables.iter().map(|table| table.id()).collect()
    }

    fn record_event(&mut self, event_type: TournamentEventType) {
        self.events.push(
            TournamentEvent { tournament_id: self.id, event_type }
        );
    }
}

impl PartialEq for Tournament {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


#[derive(Debug, Clone)]
struct TournamentTable {
    id: TableId,
    seat_count: u8,
    players: Vec<PlayerInfo>,
}

impl TournamentTable {
    pub fn new(id: TableId, spec: &TableSpecification) -> Self {
        Self { id, seat_count: spec.seat_count(), players: vec![] }
    }

    pub fn id(&self) -> TableId {
        self.id
    }

    pub fn seat_count(&self) -> u8 {
        self.seat_count
    }

    pub fn player_count(&self) -> u16 {
        self.players.len() as u16
    }

    pub fn has_free_seat(&self) -> bool {
        self.players.len() < self.seat_count as usize
    }

    pub fn seat_player(&mut self, info: PlayerInfo) {
        assert!(self.has_free_seat());
        self.players.push(info);
    }
}
