use super::nickname::Nickname;
use super::table::Table;
use super::table::TableError;
use super::table::TableEvent;
use super::table::TableSpecification;
use super::table::TableSpecificationError;

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
}


#[derive(Debug, Error)]
pub enum TournamentError {
    #[error("Tournament already started")]
    TournamentAlreadyStarted,
    #[error("Player already joined")]
    PlayerAlreadyJoined,
    #[error("Player not present")]
    PlayerNotPresent,
    #[error(transparent)]
    TableError(#[from] TableError),
}


#[derive(Debug, Clone)]
pub struct Tournament {
    id: Uuid,
    stage: TournamentStage,
    tables: Vec<Table>,
    events: Vec<TableEvent>,
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

    pub fn table_ids(&self) -> Vec<Uuid> {
        self.tables.iter().map(|table| table.id()).collect()
    }

    pub fn table_count(&self) -> usize {
        self.tables.len()
    }

    pub fn table_seat_count(&self) -> u8 {
        self.tables[0].seat_count()
    }

    pub fn is_ready_to_start(&self) -> bool {
        self.stage == TournamentStage::ReadyToStart
    }

    pub fn join(&mut self, player_id: Uuid, nickname: Nickname) -> Result<Uuid, TournamentError> {
        println!("[Tournament] join: {:?}, {:?}", player_id, nickname);
        if self.stage == TournamentStage::WaitingForPlayers {
            if self.has_player(player_id) {
                Err(TournamentError::PlayerAlreadyJoined)
            } else {
                let table_id = self.seat_player(player_id, nickname);
                if self.all_seats_are_taken() {
                    self.stage = TournamentStage::ReadyToStart;
                }
                Ok(table_id)
            }
        } else {
            Err(TournamentError::TournamentAlreadyStarted)
        }
    }

    pub fn leave(&mut self, player_id: Uuid) -> Result<(), TournamentError> {
        println!("[Tournament] leave: {:?}", player_id);
        if self.stage == TournamentStage::Running {
            Err(TournamentError::TournamentAlreadyStarted)
        } else {
            self.leave_player(player_id)
        }
    }

    pub fn start(&mut self) {
        assert!(self.is_ready_to_start());
        for table in &mut self.tables {
            table.start_game();
            self.events.extend(table.collect_events());
        }
        self.stage = TournamentStage::Running;
    }

    pub fn collect_events(&mut self) -> Vec<TableEvent> {
        std::mem::take(&mut self.events)
    }

    fn seat_player(&mut self, player_id: Uuid, nickname: Nickname) -> Uuid {
        let (table_id, table_events) = {
            let table = self.find_free_seat();
            table.sit_down(player_id, nickname.clone(), 1500);
            let table_events = table.collect_events();
            (table.id(), table_events)
        };
        self.events.extend(table_events);
        table_id
    }

    fn leave_player(&mut self, player_id: Uuid) -> Result<(), TournamentError> {
        let table_events = {
            if let Some(table) = self.find_players_table_mut(player_id) {
                table.stand_up(player_id);
                Ok(table.collect_events())
            } else {
                Err(TournamentError::PlayerNotPresent)
            }
        }?;
        self.events.extend(table_events);
        self.stage = TournamentStage::WaitingForPlayers;
        Ok(())
    }

    fn all_seats_are_taken(&self) -> bool {
        self.tables.iter().all(|table| !table.has_free_seat())
    }

    fn find_free_seat(&mut self) -> &mut Table {
        self.tables.iter_mut().find(|table| table.has_free_seat()).unwrap()
    }

    fn find_players_table_mut(&mut self, player_id: Uuid) -> Option<&mut Table> {
        self.tables.iter_mut().find(|table| table.has_player(player_id))
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
