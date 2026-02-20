use super::nickname::Nickname;
use super::player::Player;

use thiserror::Error;
use uuid::Uuid;


#[derive(Debug, Error)]
pub enum TableSpecificationError {
    #[error("There must be at least two seats, but found {found}")]
    NotEnoughSeats { found: u8 },
    #[error("There must not be more than 10 seats, but found {found}")]
    TooManySeats { found: u8 }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableSpecification {
    seat_count: u8,
}

impl TableSpecification {
    pub fn new(seat_count: u8) -> Result<Self, TableSpecificationError> {
        if seat_count < 2 {
            Err(TableSpecificationError::NotEnoughSeats { found: seat_count })
        } else if seat_count > 10 {
            Err(TableSpecificationError::TooManySeats { found: seat_count })
        } else {
            Ok(Self { seat_count })
        }
    }
}


#[derive(Debug, Error)]
pub enum TableError {
    #[error("Not player's turn")]
    NotPlayersTurn,
}


#[derive(Debug)]
pub struct TableState {
}


#[derive(Debug, Clone)]
pub struct Table {
    seats: Vec<Option<Player>>,
    messages: Vec<TableMessage>,
}

impl Table {
    pub fn new(spec: &TableSpecification) -> Self {
        let mut seats = vec![];
        for _ in 0..spec.seat_count {
            seats.push(None);
        }
        Self { seats, messages: vec![] }
    }

    pub fn spec(&self) -> TableSpecification {
        TableSpecification { seat_count: self.seats.len() as u8 }
    }

    pub fn state(&self) -> TableState {
        TableState {}
    }

    pub fn has_free_seat(&self) -> bool {
        self.seats.iter().any(|seat| seat.is_none())
    }

    pub fn seat_count(&self) -> u8 {
        self.seats.len() as u8
    }

    pub fn player_count(&self) -> u8 {
        self.seats.iter().flatten().count() as u8
    }

    pub fn has_player(&self, account_id: Uuid) -> bool {
        self.seats.iter().flatten().any(|player| player.account_id() == account_id)
    }

    pub fn sit_down(&mut self, account_id: Uuid, nickname: Nickname, stack: u32) {
        let position = self.seats.iter_mut().position(|seat| seat.is_none()).unwrap();
        let player = Player::new(account_id, nickname.clone(), stack);
        _ = self.seats[position].insert(player);
        self.messages.push(
            TableMessage::PlayerSeated {
                nickname,
                stack,
                position,
            }
        );
    }

    pub fn stand_up(&mut self, account_id: Uuid) {
        let position = self.player_position(account_id).unwrap();
        self.seats[position].take();
        self.messages.push(
            TableMessage::PlayerLeft {
                position,
            }
        );
    }

    pub fn can_start_game(&self) -> bool {
        // TODO: add check that no game is currently running
        self.player_count() >= 2
    }

    pub fn start_game(&mut self) {
        assert!(self.can_start_game());
        // TODO: really implement game logic
        self.messages.push(
            TableMessage::GameStarted { button: 0 }
        )
    }

    pub fn collect_messages(&mut self) -> Vec<TableMessage> {
        std::mem::take(&mut self.messages)
    }

    fn player_position(&self, account_id: Uuid) -> Option<usize> {
        self.seats.iter().position(|seat| {
            seat.as_ref().is_some_and(|player| player.account_id() == account_id)
        })
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum TableMessage {
    PlayerSeated {
        nickname: Nickname,
        stack: u32,
        position: usize,
    },
    PlayerLeft {
        position: usize,
    },
    GameStarted {
        button: u8,
        // TODO: further information, blinds, etc.
    },
}
