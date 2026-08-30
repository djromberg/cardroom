use super::deck::Deck;
use super::hand::{Action, Hand, HandError, HandEvent, ParticipantInfo};
use super::seat::Seat;
use super::shared::Blinds;
use super::shared::Chips;
use super::shared::PlayerId;
use super::shared::SeatNo;
use super::shared::TableId;

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
        assert!(seat_count >= 2 && seat_count <= 10);
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
        if self.hand.as_ref().is_some_and(|hand| !hand.is_finished()) {
            return Err(TableError::HandInProgress);
        }
        let occupied: Vec<_> = self
            .seats
            .iter()
            .filter(|seat| {
                seat.player_info()
                    .is_some_and(|player| player.stack > Chips(0))
            })
            .collect();
        if occupied.len() < 2 {
            return Err(TableError::NotEnoughPlayers);
        }

        let first = match self.dealer_seat {
            None => occupied[0].seat_no(),
            Some(previous) => occupied
                .iter()
                .find(|seat| seat.seat_no().0 > previous.0)
                .unwrap_or(&occupied[0])
                .seat_no(),
        };
        self.dealer_seat = Some(first);
        let dealer_index = occupied
            .iter()
            .position(|seat| seat.seat_no() == first)
            .unwrap();
        let participants = occupied
            .iter()
            .cycle()
            .skip(dealer_index)
            .take(occupied.len())
            .map(|seat| {
                let player = seat.player_info().unwrap();
                ParticipantInfo {
                    seat_no: seat.seat_no(),
                    stack: player.stack,
                }
            })
            .collect();
        let mut hand = Hand::new(deck, blinds, participants);
        let hand_events = hand.start();
        if hand.is_finished() {
            for (seat_no, stack) in hand.stacks() {
                self.seats[seat_no.0 as usize]
                    .player_info_mut()
                    .unwrap()
                    .stack = stack;
            }
        }
        self.hand = Some(hand);
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
        if hand.is_finished() {
            for (seat_no, stack) in hand.stacks() {
                self.seats[seat_no.0 as usize]
                    .player_info_mut()
                    .unwrap()
                    .stack = stack;
            }
        }
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
pub struct PlayerInfo {
    player_id: PlayerId,
    nickname: String,
    stack: Chips,
}

impl PlayerInfo {
    pub fn new(player_id: PlayerId, nickname: String, stack: Chips) -> Self {
        Self {
            player_id,
            nickname,
            stack,
        }
    }
    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }
    pub fn nickname(&self) -> &str {
        &self.nickname
    }
    pub fn stack(&self) -> Chips {
        self.stack
    }
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

#[cfg(test)]
mod tests {
    use super::super::card::Card;
    use super::*;

    fn deck() -> Deck {
        Deck::new(std::array::from_fn(|index| Card::new(index as u8)))
    }

    #[test]
    fn table_controls_hand_and_updates_stacks() {
        let mut table = Table::open(TableId(7), 2);
        table.seat_player(PlayerInfo::new(PlayerId(1), "one".into(), Chips(1000)));
        table.seat_player(PlayerInfo::new(PlayerId(2), "two".into(), Chips(1000)));
        let started = table
            .start_hand(
                deck(),
                Blinds {
                    small: Chips(50),
                    big: Chips(100),
                },
            )
            .unwrap();
        assert!(matches!(
            started.first(),
            Some(TableEvent::HandStarted {
                dealer_seat: SeatNo(0),
                ..
            })
        ));

        table.act(SeatNo(0), Action::Fold).unwrap();
        assert_eq!(table.seats[0].player_info().unwrap().stack, Chips(950));
        assert_eq!(table.seats[1].player_info().unwrap().stack, Chips(1050));
    }

    #[test]
    fn cannot_replace_an_active_hand() {
        let mut table = Table::open(TableId(7), 2);
        table.seat_player(PlayerInfo::new(PlayerId(1), "one".into(), Chips(1000)));
        table.seat_player(PlayerInfo::new(PlayerId(2), "two".into(), Chips(1000)));
        table
            .start_hand(
                deck(),
                Blinds {
                    small: Chips(50),
                    big: Chips(100),
                },
            )
            .unwrap();
        assert_eq!(
            table.start_hand(
                deck(),
                Blinds {
                    small: Chips(50),
                    big: Chips(100)
                }
            ),
            Err(TableError::HandInProgress)
        );
    }
}
