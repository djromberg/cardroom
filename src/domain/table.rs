use std::collections::HashMap;

use crate::domain::DomainError;
use crate::domain::Player;
use crate::domain::PlayerId;
use crate::domain::PlayerInfo;
use crate::domain::TournamentId;

use uuid::Uuid;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TableId(Uuid);

impl TableId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TableSpecification {
    seat_count: u8,
}

impl TableSpecification {
    pub fn new(seat_count: u8) -> Result<Self, DomainError> {
        if seat_count < 2 || seat_count > 10 {
            Err(DomainError::InvalidTableSpecification)
        } else {
            Ok(Self { seat_count })
        }
    }

    pub fn seat_count(&self) -> u8 {
        self.seat_count
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct TableEvent {
    pub table_id: TableId,
    pub tournament_id: TournamentId,
    pub event_type: TableEventType,
}


#[derive(Debug, Clone, PartialEq)]
pub enum TableEventType {
    TableOpened {
        seat_count: u8,
    },
    PlayerSeated {
        position: u8,
        player_info: PlayerInfo,
    },
    GameStarted {
        button_position: u8,
        /* involved players, etc. */
    },
    PlayerActionRequested {
        position: u8,
        /* min bet, min raise, etc. */
    },
    PlayerLeft {
        position: u8,
        player_info: PlayerInfo,
    },
}


#[derive(Debug, Clone)]
pub struct Table {
    id: TableId,
    tournament_id: TournamentId,
    seats: Vec<Option<Player>>,
    button: u8,
    game: Option<Game>,
    events: Vec<TableEvent>,
}

impl Table {
    pub fn new(id: TableId, tournament_id: TournamentId, spec: &TableSpecification) -> Self {
        let mut seats = vec![];
        for _ in 0..spec.seat_count {
            seats.push(None);
        }
        let mut table = Self { id, tournament_id, seats, button: 0, game: None, events: vec![] };
        table.record_event(TableEventType::TableOpened { seat_count: spec.seat_count });
        table
    }

    pub fn id(&self) -> TableId {
        self.id
    }

    pub fn seat_player(&mut self, player_info: PlayerInfo) {
        let position = self.seats.iter_mut().position(|seat| seat.is_none()).unwrap();
        let player = Player::new(&player_info);
        _ = self.seats[position].insert(player);
        self.record_event(TableEventType::PlayerSeated {
            position: position as u8,
            player_info
        });
    }

    pub fn start_game(&mut self /* deck, blinds */) {
        assert!(self.game.is_none());
        let game = Game::new(/* deck */);
        /* for player in seats: player.start game(game.deal_card()) */
        /* for player in seats: game.deal_card */
    }

    pub fn act(&mut self, player_id: PlayerId, action: u8 /* use enum */) -> Result<(), DomainError> {
        Ok(())
    }

    pub fn consume_events(&mut self) -> Vec<TableEvent> {
        std::mem::take(&mut self.events)
    }

    fn record_event(&mut self, event_type: TableEventType) {
        self.events.push(
            TableEvent { table_id: self.id, tournament_id: self.tournament_id, event_type }
        );
    }
}


#[derive(Debug, Clone)]
struct Game {
    /* deck */
    shares: HashMap<PlayerId, u32>,
    board_cards: Vec<Card>,
}

impl Game {
    pub fn new(/* deck */) -> Self {
        Self { shares: HashMap::new(), board_cards: vec![] }
    }
}


pub type Card = u8;
