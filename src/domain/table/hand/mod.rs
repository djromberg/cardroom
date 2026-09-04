use std::cmp::min;

use super::super::shared::{Blinds, Chips, SeatNo};
use super::card::Card;
use super::deck::Deck;
use dealer::Dealer;
use participant::Participant;
use ranking::evaluate;

pub(super) use participant::ParticipantInfo;
pub use pot::{Pot, PotAward};
pub use ranking::{EvaluatedHand, HandCategory};

mod dealer;
mod participant;
mod pot;
mod ranking;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub(super) struct Hand {
    blinds: Blinds,
    dealer: Dealer,
    participants: Vec<Participant>,
    board: Vec<Card>,
    street: Street,
    current: Option<usize>,
    max_bet: Chips,
    min_raise: Chips,
    betting_round_open: bool,
    pots: Vec<Pot>,
    started: bool,
    finished: bool,
}

impl Hand {
    /// `participants` are clockwise from the dealer/button.
    pub(super) fn new(deck: Deck, blinds: Blinds, participants: Vec<ParticipantInfo>) -> Self {
        assert!((2..=10).contains(&participants.len()));
        assert!(blinds.small > Chips(0) && blinds.big >= blinds.small);
        let mut seats: Vec<_> = participants.iter().map(|p| p.seat_no).collect();
        seats.sort_by_key(|seat| seat.0);
        seats.dedup();
        assert_eq!(
            seats.len(),
            participants.len(),
            "duplicate participant seat"
        );
        assert!(participants.iter().all(|p| p.stack > Chips(0)));
        Self {
            blinds,
            dealer: Dealer::new(deck),
            participants: participants.iter().map(Participant::new).collect(),
            board: vec![],
            street: Street::Preflop,
            current: None,
            max_bet: blinds.big,
            min_raise: blinds.big,
            betting_round_open: true,
            pots: vec![],
            started: false,
            finished: false,
        }
    }

    pub(super) fn start(&mut self) -> Vec<HandEvent> {
        assert!(!self.started, "hand already started");
        self.started = true;
        let small = if self.participants.len() == 2 { 0 } else { 1 };
        let big = (small + 1) % self.participants.len();
        let mut events = vec![];
        events.extend(self.post_blind(small, Blind::Small, self.blinds.small));
        events.extend(self.post_blind(big, Blind::Big, self.blinds.big));
        events.push(self.deal_hole_cards());
        for participant in &mut self.participants {
            participant.pending = participant.can_act();
        }
        self.current = self.next_pending(big);
        events.extend(self.progress());
        events
    }

    pub(super) fn act(
        &mut self,
        seat_no: SeatNo,
        action: Action,
    ) -> Result<Vec<HandEvent>, HandError> {
        if !self.started {
            return Err(HandError::NotStarted);
        }
        if self.finished {
            return Err(HandError::Finished);
        }
        let position = self.current.ok_or(HandError::NoActionRequested)?;
        let expected = self.participants[position].seat_no;
        if expected != seat_no {
            return Err(HandError::NotPlayersTurn {
                expected,
                actual: seat_no,
            });
        }

        let mut events = vec![];
        match action {
            Action::Fold => {
                self.participants[position].folded = true;
                self.participants[position].pending = false;
            }
            Action::Check => {
                if self.participants[position].current_bet != self.max_bet {
                    return Err(HandError::CannotCheck);
                }
                self.participants[position].pending = false;
            }
            Action::Call => {
                let amount = self.max_bet - self.participants[position].current_bet;
                if amount == Chips(0) {
                    return Err(HandError::NothingToCall);
                }
                events.push(self.place_chips(position, amount));
                self.participants[position].pending = false;
            }
            Action::Bet(to) => {
                if self.max_bet != Chips(0) {
                    return Err(HandError::BetNotAllowed);
                }
                self.raise_to(position, to, &mut events)?;
            }
            Action::RaiseTo(to) => {
                if self.max_bet == Chips(0) {
                    return Err(HandError::RaiseNotAllowed);
                }
                self.raise_to(position, to, &mut events)?;
            }
        }
        events.insert(0, HandEvent::PlayerActed { seat_no, action });
        self.current = self.next_pending(position);
        events.extend(self.progress());
        Ok(events)
    }

    pub(super) fn is_finished(&self) -> bool {
        self.finished
    }
    fn stacks(&self) -> Vec<(SeatNo, Chips)> {
        self.participants
            .iter()
            .map(|p| (p.seat_no, p.stack))
            .collect()
    }

    pub(super) fn into_result(self) -> HandResult {
        assert!(self.finished, "cannot settle an unfinished hand");
        HandResult {
            stacks: self
                .participants
                .into_iter()
                .map(|participant| (participant.seat_no, participant.stack))
                .collect(),
        }
    }

    fn raise_to(
        &mut self,
        position: usize,
        to: Chips,
        events: &mut Vec<HandEvent>,
    ) -> Result<(), HandError> {
        let maximum = self.participants[position].current_bet + self.participants[position].stack;
        if to <= self.max_bet || to > maximum {
            return Err(HandError::InvalidAmount);
        }
        let raise = to - self.max_bet;
        if raise < self.min_raise && to != maximum {
            return Err(HandError::RaiseTooSmall {
                minimum: self.max_bet + self.min_raise,
            });
        }
        let additional = to - self.participants[position].current_bet;
        events.push(self.place_chips(position, additional));
        self.max_bet = to;
        if raise >= self.min_raise {
            self.min_raise = raise;
            for (index, participant) in self.participants.iter_mut().enumerate() {
                participant.pending = index != position && participant.can_act();
            }
        } else {
            self.participants[position].pending = false;
        }
        Ok(())
    }

    fn progress(&mut self) -> Vec<HandEvent> {
        let mut events = vec![];
        loop {
            if self.participants.iter().filter(|p| !p.folded).count() == 1 {
                events.extend(self.complete_betting_round());
                let winner = self.participants.iter().position(|p| !p.folded).unwrap();
                let pot = self.pots.iter().fold(Chips(0), |sum, pot| sum + pot.amount);
                self.participants[winner].stack = self.participants[winner].stack + pot;
                self.finished = true;
                events.push(HandEvent::PotAwarded {
                    amount: pot,
                    eligible_seats: vec![self.participants[winner].seat_no],
                    awards: vec![PotAward {
                        seat_no: self.participants[winner].seat_no,
                        amount: pot,
                    }],
                });
                events.push(HandEvent::HandFinished);
                break;
            }
            if let Some(position) = self.current {
                let participant = &self.participants[position];
                events.push(HandEvent::ActionRequested {
                    seat_no: participant.seat_no,
                    to_call: min(self.max_bet - participant.current_bet, participant.stack),
                    min_raise_to: self.max_bet + self.min_raise,
                });
                break;
            }
            events.extend(self.complete_betting_round());
            if self.street == Street::River {
                events.extend(self.showdown());
                break;
            }
            events.push(self.advance_street());
            self.current = self.next_pending(0);
        }
        events
    }

    fn advance_street(&mut self) -> HandEvent {
        self.street = self.street.next().unwrap();
        let can_bet = self
            .participants
            .iter()
            .filter(|participant| participant.can_act())
            .count()
            >= 2;
        for participant in &mut self.participants {
            participant.current_bet = Chips(0);
            participant.pending = can_bet && participant.can_act();
        }
        self.betting_round_open = can_bet;
        self.max_bet = Chips(0);
        self.min_raise = self.blinds.big;
        self.dealer.deal_card(); // burn
        let count = if self.street == Street::Flop { 3 } else { 1 };
        let cards: Vec<_> = (0..count).map(|_| self.dealer.deal_card()).collect();
        self.board.extend(cards.iter().copied());
        HandEvent::CommunityCardsDealt {
            street: self.street,
            cards,
        }
    }

    fn showdown(&mut self) -> Vec<HandEvent> {
        let contenders: Vec<_> = self
            .participants
            .iter()
            .enumerate()
            .filter(|(_, participant)| !participant.folded)
            .map(|(position, _)| position)
            .collect();
        let seat_nos = contenders
            .iter()
            .map(|position| self.participants[*position].seat_no)
            .collect();
        let mut events = vec![HandEvent::ShowdownStarted { seat_nos }];

        for position in &contenders {
            let participant = &self.participants[*position];
            let cards = participant.cards.as_slice().try_into().unwrap();
            events.push(HandEvent::HoleCardsShown {
                seat_no: participant.seat_no,
                cards,
                hand: self.evaluated_hand(*position).evaluated,
            });
        }

        for pot in self.pots.clone() {
            let eligible: Vec<_> = contenders
                .iter()
                .copied()
                .filter(|position| {
                    pot.eligible_seats
                        .contains(&self.participants[*position].seat_no)
                })
                .collect();
            debug_assert!(!eligible.is_empty());
            let evaluations: Vec<_> = eligible
                .iter()
                .map(|position| (*position, self.evaluated_hand(*position).score))
                .collect();
            let best = evaluations.iter().map(|(_, score)| *score).max().unwrap();
            let winners: Vec<_> = evaluations
                .into_iter()
                .filter(|(_, score)| *score == best)
                .map(|(position, _)| position)
                .collect();
            let share = pot.amount.0 / winners.len() as u64;
            let remainder = pot.amount.0 % winners.len() as u64;
            let mut awards = Vec::with_capacity(winners.len());
            for (order, position) in winners.into_iter().enumerate() {
                let amount = Chips(share + u64::from((order as u64) < remainder));
                self.participants[position].stack = self.participants[position].stack + amount;
                awards.push(PotAward {
                    seat_no: self.participants[position].seat_no,
                    amount,
                });
            }
            events.push(HandEvent::PotAwarded {
                amount: pot.amount,
                eligible_seats: pot.eligible_seats,
                awards,
            });
        }
        self.finished = true;
        self.current = None;
        events.push(HandEvent::HandFinished);
        events
    }

    fn evaluated_hand(&self, position: usize) -> ranking::RankedHand {
        let mut cards = self.board.clone();
        cards.extend(self.participants[position].cards.iter().copied());
        evaluate(&cards)
    }

    fn deal_hole_cards(&mut self) -> HandEvent {
        let mut seat_nos = vec![];
        for round in 0..2 {
            for offset in 1..=self.participants.len() {
                let position = offset % self.participants.len();
                self.participants[position]
                    .cards
                    .push(self.dealer.deal_card());
                if round == 0 {
                    seat_nos.push(self.participants[position].seat_no);
                }
            }
        }
        HandEvent::HoleCardsDealt { seat_nos }
    }

    fn place_chips(&mut self, position: usize, requested: Chips) -> HandEvent {
        let participant = &mut self.participants[position];
        let amount = min(requested, participant.stack);
        participant.stack = participant.stack - amount;
        participant.current_bet = participant.current_bet + amount;
        participant.committed = participant.committed + amount;
        HandEvent::ChipsCommitted {
            seat_no: participant.seat_no,
            amount,
            current_bet: participant.current_bet,
            remaining_stack: participant.stack,
        }
    }

    fn post_blind(&mut self, position: usize, blind: Blind, amount: Chips) -> Vec<HandEvent> {
        vec![
            HandEvent::BlindPosted {
                seat_no: self.participants[position].seat_no,
                blind,
            },
            self.place_chips(position, amount),
        ]
    }

    fn next_pending(&self, after: usize) -> Option<usize> {
        (1..=self.participants.len())
            .map(|offset| (after + offset) % self.participants.len())
            .find(|position| {
                self.participants[*position].can_act() && self.participants[*position].pending
            })
    }

    fn complete_betting_round(&mut self) -> Vec<HandEvent> {
        if !self.betting_round_open {
            return vec![];
        }

        let street = self.street;
        let mut events = vec![];
        if let Some((position, amount)) = self.uncalled_chips() {
            let participant = &mut self.participants[position];
            participant.stack = participant.stack + amount;
            participant.current_bet = participant.current_bet - amount;
            participant.committed = participant.committed - amount;
            events.push(HandEvent::ChipsReturned {
                seat_no: participant.seat_no,
                amount,
                remaining_stack: participant.stack,
            });
        }

        self.pots = pot::calculate(&self.participants);
        self.betting_round_open = false;
        events.push(HandEvent::BettingRoundCompleted {
            street,
            pots: self.pots.clone(),
        });
        events
    }

    fn uncalled_chips(&self) -> Option<(usize, Chips)> {
        let mut bets: Vec<_> = self
            .participants
            .iter()
            .enumerate()
            .map(|(position, participant)| (participant.current_bet, position))
            .collect();
        bets.sort_unstable_by_key(|(amount, _)| *amount);
        let (highest, position) = bets[bets.len() - 1];
        let second_highest = bets[bets.len() - 2].0;
        (highest > second_highest).then_some((position, highest - second_highest))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Fold,
    Check,
    Call,
    Bet(Chips),
    RaiseTo(Chips),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blind {
    Small,
    Big,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandError {
    NotStarted,
    Finished,
    NoActionRequested,
    NotPlayersTurn { expected: SeatNo, actual: SeatNo },
    CannotCheck,
    NothingToCall,
    BetNotAllowed,
    RaiseNotAllowed,
    InvalidAmount,
    RaiseTooSmall { minimum: Chips },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandEvent {
    BlindPosted {
        seat_no: SeatNo,
        blind: Blind,
    },
    ChipsCommitted {
        seat_no: SeatNo,
        amount: Chips,
        current_bet: Chips,
        remaining_stack: Chips,
    },
    HoleCardsDealt {
        seat_nos: Vec<SeatNo>,
    },
    ActionRequested {
        seat_no: SeatNo,
        to_call: Chips,
        min_raise_to: Chips,
    },
    PlayerActed {
        seat_no: SeatNo,
        action: Action,
    },
    ChipsReturned {
        seat_no: SeatNo,
        amount: Chips,
        remaining_stack: Chips,
    },
    BettingRoundCompleted {
        street: Street,
        pots: Vec<Pot>,
    },
    CommunityCardsDealt {
        street: Street,
        cards: Vec<Card>,
    },
    ShowdownStarted {
        seat_nos: Vec<SeatNo>,
    },
    HoleCardsShown {
        seat_no: SeatNo,
        cards: [Card; 2],
        hand: EvaluatedHand,
    },
    PotAwarded {
        amount: Chips,
        eligible_seats: Vec<SeatNo>,
        awards: Vec<PotAward>,
    },
    HandFinished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}
impl Street {
    fn next(self) -> Option<Self> {
        match self {
            Self::Preflop => Some(Self::Flop),
            Self::Flop => Some(Self::Turn),
            Self::Turn => Some(Self::River),
            Self::River => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct HandResult {
    stacks: Vec<(SeatNo, Chips)>,
}

impl HandResult {
    pub(super) fn into_stacks(self) -> Vec<(SeatNo, Chips)> {
        self.stacks
    }
}
