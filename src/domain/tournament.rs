use super::nickname::Nickname;
use super::table::Table;
use super::table::TableError;
use super::table::TableEvent;
use super::table::TableSpecification;
use super::table::TableSpecificationError;
use super::table::TableState;

use log::debug;
use thiserror::Error;
use uuid::Uuid;


#[derive(Debug, Error)]
pub enum TournamentSpecificationError {
    #[error("There must be at least {min} table(s), but found {found}")]
    NotEnoughTables { min: u8, found: u8 },
    #[error("There must not be more than {max} tables, but found {found}")]
    TooManyTables { max: u8, found: u8 },
    #[error(transparent)]
    TableSpecificationError(#[from] TableSpecificationError)
}

#[derive(Debug)]
pub struct TournamentSpecification {
    table_count: u8,
    table_spec: TableSpecification,
}

impl TournamentSpecification {
    pub fn new(table_count: u8, table_seat_count: u8) -> Result<Self, TournamentSpecificationError> {
        const MIN_TABLES: u8 = 1;
        const MAX_TABLES: u8 = 100;
        if table_count < MIN_TABLES {
            Err(TournamentSpecificationError::NotEnoughTables { min: MIN_TABLES, found: table_count })
        } else if table_count > MAX_TABLES {
            Err(TournamentSpecificationError::TooManyTables { max: MAX_TABLES, found: table_count })
        } else {
            let table_spec = TableSpecification::new(table_seat_count)?;
            Ok(Self { table_count, table_spec })
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum TournamentStage {
    WaitingForPlayers,
    ReadyToStart,
    Running,
    Finished,
}


#[derive(Debug, Error)]
pub enum TournamentError {
    #[error("Tournament already started")]
    TournamentAlreadyStarted,
    #[error("Player already joined")]
    PlayerAlreadyJoined,
    #[error("No such table")]
    NotSuchTable,
    #[error(transparent)]
    TableError(#[from] TableError),
}


#[derive(Debug, Clone)]
pub struct Tournament {
    id: Uuid,
    stage: TournamentStage,
    tables: Vec<Table>,
    events: Vec<TournamentEvent>,
}

impl Tournament {
    pub fn new(spec: &TournamentSpecification) -> Self {
        let mut tables = vec![];
        for _ in 0..spec.table_count {
            tables.push(Table::new(&spec.table_spec));
        }
        Self { id: Uuid::new_v4(), stage: TournamentStage::WaitingForPlayers, tables, events: vec![] }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn table_count(&self) -> usize {
        self.tables.len()
    }

    pub fn table_seat_count(&self) -> u8 {
        self.tables[0].seat_count()
    }

    pub fn player_count(&self) -> usize {
        self.tables.iter().map(|table| table.player_count() as usize).sum()
    }

    pub fn is_waiting_for_players(&self) -> bool {
        self.stage == TournamentStage::WaitingForPlayers
    }

    pub fn is_finished(&self) -> bool {
        self.stage == TournamentStage::Finished
    }

    pub fn players_table_number(&self, player_id: Uuid) -> Option<usize> {
        self.tables.iter().position(|table| table.has_player(player_id))
    }

    pub fn is_ready_to_start(&self) -> bool {
        self.stage == TournamentStage::ReadyToStart
    }

    pub fn table_state(&self, table_number: usize) -> Result<TableState, TournamentError> {
        let table = self.tables.get(table_number).ok_or_else(|| TournamentError::NotSuchTable)?;
        Ok(table.state())
    }

    pub fn join(&mut self, player_id: Uuid, nickname: Nickname) -> Result<usize, TournamentError> {
        debug!("join player_id {} with nickname {} within tournament {}", player_id, nickname, self.id);
        if self.stage == TournamentStage::WaitingForPlayers {
            if self.has_player(player_id) {
                Err(TournamentError::PlayerAlreadyJoined)
            } else {
                let table_number = self.seat_player(player_id, nickname);
                if self.all_seats_are_taken() {
                    self.stage = TournamentStage::ReadyToStart;
                }
                Ok(table_number)
            }
        } else {
            Err(TournamentError::TournamentAlreadyStarted)
        }
    }

    pub fn start(&mut self) {
        assert!(self.is_ready_to_start());
        for (table_number, table) in self.tables.iter_mut().enumerate() {
            table.start_game();

            let table_events = table.collect_events();
            let tournament_events = table_events.iter().map(|table_event|
                TournamentEvent {
                    tournament_id: self.id,
                    event_type: TournamentEventType::TableEvent {
                        table_number, event_type: table_event.clone()
                    },
                }
            );
            self.events.extend(tournament_events);
        }
        self.stage = TournamentStage::Running;
    }

    pub fn collect_events(&mut self) -> Vec<TournamentEvent> {
        std::mem::take(&mut self.events)
    }

    fn seat_player(&mut self, player_id: Uuid, nickname: Nickname) -> usize {
        let (table_number, table_events) = {
            let table_number = self.find_table_with_free_seats();
            let table = &mut self.tables[table_number];
            table.sit_down(player_id, nickname.clone(), 1500);
            let table_events = table.collect_events();
            (table_number, table_events)
        };
        let tournament_events = table_events.iter().map(|table_event| TournamentEvent {
            tournament_id: self.id,
            event_type: TournamentEventType::TableEvent {
                table_number, event_type: table_event.clone()
            },
        });
        self.events.extend(tournament_events);
        table_number
    }

    fn all_seats_are_taken(&self) -> bool {
        self.tables.iter().all(|table| !table.has_free_seat())
    }

    fn find_table_with_free_seats(&self) -> usize {
        self.tables.iter().enumerate().find(|(_, table)| table.has_free_seat()).map(|(index, _)| index).unwrap()
    }

    fn has_player(&self, player_id: Uuid) -> bool {
        self.tables.iter().any(|table| table.has_player(player_id))
    }
}

impl PartialEq for Tournament {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct TournamentEvent {
    pub tournament_id: Uuid,
    pub event_type: TournamentEventType,
}


#[derive(Debug, Clone, PartialEq)]
pub enum TournamentEventType {
    TableEvent {
        table_number: usize,
        event_type: TableEvent,
    },
}
