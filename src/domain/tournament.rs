use super::nickname::Nickname;
use super::table::Table;
use super::table::TableError;
use super::table::TableMessage;
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

#[derive(Debug, Clone, PartialEq)]
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


#[derive(Debug, Clone, PartialEq)]
pub enum TournamentEvent {
    TournamentCreated {
        id: Uuid,
        spec: TournamentSpecification
    },
    PlayerJoined {
        account_id: Uuid,
        nickname: Nickname,
    }
}


#[derive(Debug, Clone)]
pub struct Tournament {
    id: Uuid,
    stage: TournamentStage,
    tables: Vec<Table>,
    messages: Vec<TournamentMessage>,
    events: Vec<TournamentEvent>,
}

impl Tournament {
    pub fn new(spec: &TournamentSpecification) -> Self {
        Self::create(Uuid::new_v4(), spec)
    }

    pub fn restore(events: impl IntoIterator<Item = TournamentEvent>) -> Self {
        let mut event_iterator = events.into_iter();
        let first_event = event_iterator.next().unwrap();
        let mut tournament = match first_event {
            TournamentEvent::TournamentCreated { id, spec } => {
                Self::create(id, &spec)
            },
            _ => panic!("programming error")
        };
        for event in event_iterator {
            tournament.apply(event);
        }
        tournament
    }

    pub fn events(&self) -> Vec<TournamentEvent> {
        self.events.clone()
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn spec(&self) -> TournamentSpecification {
        TournamentSpecification {
            table_count: self.tables.len() as u8,
            table_spec: self.tables[0].spec(),
        }
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

    pub fn players_table_number(&self, account_id: Uuid) -> Option<usize> {
        self.tables.iter().position(|table| table.has_player(account_id))
    }

    pub fn is_ready_to_start(&self) -> bool {
        self.stage == TournamentStage::ReadyToStart
    }

    pub fn table_state(&self, table_number: usize) -> Result<TableState, TournamentError> {
        let table = self.tables.get(table_number).ok_or_else(|| TournamentError::NotSuchTable)?;
        Ok(table.state())
    }

    pub fn join(&mut self, account_id: Uuid, nickname: Nickname) -> Result<usize, TournamentError> {
        debug!("join account_id {} with nickname {} within tournament {}", account_id, nickname, self.id);
        if self.stage == TournamentStage::WaitingForPlayers {
            if self.has_player(account_id) {
                Err(TournamentError::PlayerAlreadyJoined)
            } else {
                let table_number = self.seat_player(account_id, nickname.clone());
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

            let table_messages = table.collect_messages();
            let tournament_messages = table_messages.iter().map(|table_message|
                TournamentMessage {
                    tournament_id: self.id,
                    message_type: TournamentMessageType::TableMessage {
                        table_number, message_type: table_message.clone()
                    },
                }
            );
            self.messages.extend(tournament_messages);
        }
        self.stage = TournamentStage::Running;
    }

    pub fn collect_messages(&mut self) -> Vec<TournamentMessage> {
        std::mem::take(&mut self.messages)
    }

    fn create(id: Uuid, spec: &TournamentSpecification) -> Self {
        let mut tables = vec![];
        for _ in 0..spec.table_count {
            tables.push(Table::new(&spec.table_spec));
        }
        Self {
            id,
            stage: TournamentStage::WaitingForPlayers,
            tables,
            messages: vec![],
            events: vec![TournamentEvent::TournamentCreated { id, spec: spec.clone() }],
        }
    }

    fn apply(&mut self, event: TournamentEvent) {
        match event {
            TournamentEvent::PlayerJoined { account_id, nickname } => {
                _ = self.join(account_id, nickname).unwrap()
            },
            TournamentEvent::TournamentCreated { .. } => panic!("programming error")
        }
    }

    fn seat_player(&mut self, account_id: Uuid, nickname: Nickname) -> usize {
        let (table_number, table_messages) = {
            let table_number = self.find_table_with_free_seats();
            let table = &mut self.tables[table_number];
            table.sit_down(account_id, nickname.clone(), 1500);
            let table_messages = table.collect_messages();
            (table_number, table_messages)
        };
        let tournament_messages = table_messages.iter().map(|table_message| TournamentMessage {
            tournament_id: self.id,
            message_type: TournamentMessageType::TableMessage {
                table_number, message_type: table_message.clone()
            },
        });
        self.messages.extend(tournament_messages);
        self.events.push(TournamentEvent::PlayerJoined { account_id, nickname });
        table_number
    }

    fn all_seats_are_taken(&self) -> bool {
        self.tables.iter().all(|table| !table.has_free_seat())
    }

    fn find_table_with_free_seats(&self) -> usize {
        self.tables.iter().enumerate().find(|(_, table)| table.has_free_seat()).map(|(index, _)| index).unwrap()
    }

    fn has_player(&self, account_id: Uuid) -> bool {
        self.tables.iter().any(|table| table.has_player(account_id))
    }
}

impl PartialEq for Tournament {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct TournamentMessage {
    pub tournament_id: Uuid,
    pub message_type: TournamentMessageType,
}


#[derive(Debug, Clone, PartialEq)]
pub enum TournamentMessageType {
    TableMessage {
        table_number: usize,
        message_type: TableMessage,
    },
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tournament_creation_and_join() {
        let spec = TournamentSpecification::new(1, 3).unwrap();
        let tournament = Tournament::new(&spec);
        assert_eq!(tournament.events(), vec![TournamentEvent::TournamentCreated { id: tournament.id(), spec: spec.clone() }]);
        let mut tournament = tournament;
        let account_id = Uuid::new_v4();
        let nickname = Nickname::new("Daniel").unwrap();
        let _ = tournament.join(account_id, nickname.clone()); // TODO: do not return value in join
        assert_eq!(tournament.events(), vec![
            TournamentEvent::TournamentCreated { id: tournament.id(), spec },
            TournamentEvent::PlayerJoined { account_id, nickname }
        ]);
    }

    #[test]
    fn tournament_restore() {
        let spec = TournamentSpecification::new(1, 3).unwrap();
        let tournament_id = Uuid::new_v4();
        let account_id = Uuid::new_v4();
        let nickname = Nickname::new("Daniel").unwrap();
        let events = vec![
            TournamentEvent::TournamentCreated { id: tournament_id, spec: spec.clone() },
            TournamentEvent::PlayerJoined { account_id, nickname }
        ];
        let tournament = Tournament::restore(events.clone());
        assert_eq!(tournament.id(), tournament_id);
        assert_eq!(tournament.spec(), spec);
        assert!(tournament.has_player(account_id));
        assert_eq!(tournament.events(), events);
    }
}
