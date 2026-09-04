use uuid::Uuid;

use crate::domain::{
    chips::Chips,
    table::{
        deck::Deck,
        hand::{Hand, ParticipantInfo},
        player::Player,
        seat::Seat,
    },
};

pub use crate::domain::table::{
    blinds::Blinds,
    hand::{Action, HandError, HandEvent},
    player::PlayerInfo,
    seat::SeatNo,
};

mod blinds;
pub mod card;
pub mod deck;
mod hand;
mod player;
mod seat;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TableId(Uuid);

impl TableId {
    pub const fn new(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<Uuid> for TableId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<TableId> for Uuid {
    fn from(table_id: TableId) -> Self {
        table_id.0
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    id: TableId,
    seats: Vec<Seat>,
    hand: Option<Hand>,
    events: Vec<TableEvent>,
    dealer_seat: Option<SeatNo>,
}

impl Table {
    pub fn open(id: TableId, seat_count: u8) -> Self {
        assert!((2..=10).contains(&seat_count));
        let mut seats = vec![];
        for i in 0..seat_count {
            seats.push(Seat::new(SeatNo(i)))
        }
        let events = vec![TableEvent::TableOpened {
            table_id: id,
            seat_count,
        }];
        Self {
            id,
            seats,
            hand: None,
            events,
            dealer_seat: None,
        }
    }

    pub fn seat_player(&mut self, player_info: PlayerInfo) {
        let seat = self.seats.iter_mut().find(|seat| seat.is_free()).unwrap();
        seat.take(player_info.clone());
        self.events.push(TableEvent::PlayerSeated {
            table_id: self.id,
            seat_no: seat.seat_no(),
            player_info,
        });
    }

    pub fn start_hand(
        &mut self,
        deck: Deck,
        blinds: Blinds,
    ) -> Result<Vec<TableEvent>, TableError> {
        if self.hand.is_some() {
            return Err(TableError::HandInProgress);
        }
        let occupied: Vec<_> = self
            .seats
            .iter()
            .filter(|seat| {
                seat.player()
                    .and_then(Player::stack)
                    .is_some_and(|stack| stack > Chips(0))
            })
            .map(Seat::seat_no)
            .collect();
        if occupied.len() < 2 {
            return Err(TableError::NotEnoughPlayers);
        }

        let first = match self.dealer_seat {
            None => occupied[0],
            Some(previous) => occupied
                .iter()
                .find(|seat_no| seat_no.0 > previous.0)
                .unwrap_or(&occupied[0])
                .to_owned(),
        };
        self.dealer_seat = Some(first);
        let dealer_index = occupied
            .iter()
            .position(|seat_no| *seat_no == first)
            .unwrap();
        let participants = occupied
            .iter()
            .cycle()
            .skip(dealer_index)
            .take(occupied.len())
            .filter_map(|seat_no| self.seats[seat_no.0 as usize].participate_in_hand())
            .collect();
        let mut hand = Hand::new(deck, blinds, participants);
        let hand_events = hand.start();
        self.hand = Some(hand);
        self.settle_finished_hand();
        let mut events = vec![TableEvent::HandStarted {
            table_id: self.id,
            dealer_seat: first,
        }];
        events.extend(hand_events.into_iter().map(|event| TableEvent::HandEvent {
            table_id: self.id,
            event,
        }));
        self.events.extend(events.iter().cloned());
        Ok(events)
    }

    pub fn act(&mut self, seat_no: SeatNo, action: Action) -> Result<Vec<TableEvent>, TableError> {
        let hand = self.hand.as_mut().ok_or(TableError::NoHand)?;
        let hand_events = hand.act(seat_no, action)?;
        self.settle_finished_hand();
        let events: Vec<_> = hand_events
            .into_iter()
            .map(|event| TableEvent::HandEvent {
                table_id: self.id,
                event,
            })
            .collect();
        self.events.extend(events.iter().cloned());
        Ok(events)
    }

    fn settle_finished_hand(&mut self) {
        if !self.hand.as_ref().is_some_and(Hand::is_finished) {
            return;
        }
        let result = self.hand.take().unwrap().into_result();
        for (seat_no, stack) in result.into_stacks() {
            self.seats[seat_no.0 as usize].return_from_hand(ParticipantInfo { seat_no, stack });
        }
    }

    pub fn collect_events(&mut self) -> Vec<TableEvent> {
        std::mem::take(&mut self.events)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableEvent {
    TableOpened {
        table_id: TableId,
        seat_count: u8,
    },
    PlayerSeated {
        table_id: TableId,
        seat_no: SeatNo,
        player_info: PlayerInfo,
    },
    HandStarted {
        table_id: TableId,
        dealer_seat: SeatNo,
    },
    HandEvent {
        table_id: TableId,
        event: HandEvent,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TableError {
    HandInProgress,
    NotEnoughPlayers,
    NoHand,
    Hand(HandError),
}

impl From<HandError> for TableError {
    fn from(error: HandError) -> Self {
        Self::Hand(error)
    }
}
