use std::cmp::min;

use super::card::Card;
use super::dealer::Dealer;
use super::deck::Deck;
use super::shared::{Blinds, Chips, SeatNo};

#[derive(Debug, Clone)]
pub struct Hand {
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
    pub fn new(deck: Deck, blinds: Blinds, participants: Vec<ParticipantInfo>) -> Self {
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

    pub fn start(&mut self) -> Vec<HandEvent> {
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

    pub fn act(&mut self, seat_no: SeatNo, action: Action) -> Result<Vec<HandEvent>, HandError> {
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

    pub fn is_finished(&self) -> bool {
        self.finished
    }
    pub fn stacks(&self) -> Vec<(SeatNo, Chips)> {
        self.participants
            .iter()
            .map(|p| (p.seat_no, p.stack))
            .collect()
    }

    pub fn into_result(self) -> HandResult {
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
                hand: self.evaluated_hand(*position).1,
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
                .map(|position| (*position, self.evaluated_hand(*position).0))
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

    fn evaluated_hand(&self, position: usize) -> (u64, EvaluatedHand) {
        let mut cards = self.board.clone();
        cards.extend(self.participants[position].cards.iter().copied());
        let mut best = 0;
        let mut best_five = [cards[0]; 5];
        for a in 0..cards.len() - 4 {
            for b in a + 1..cards.len() - 3 {
                for c in b + 1..cards.len() - 2 {
                    for d in c + 1..cards.len() - 1 {
                        for e in d + 1..cards.len() {
                            let candidate = [cards[a], cards[b], cards[c], cards[d], cards[e]];
                            let score = score_five(candidate);
                            if score > best {
                                best = score;
                                best_five = candidate;
                            }
                        }
                    }
                }
            }
        }
        (
            best,
            EvaluatedHand {
                category: HandCategory::from_score(best),
                best_five,
            },
        )
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

        self.pots = self.calculate_pots();
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

    fn calculate_pots(&self) -> Vec<Pot> {
        let mut levels: Vec<_> = self
            .participants
            .iter()
            .map(|participant| participant.committed.0)
            .filter(|amount| *amount > 0)
            .collect();
        levels.sort_unstable();
        levels.dedup();

        let mut previous = 0;
        let mut pots: Vec<Pot> = vec![];
        for level in levels {
            let contributors = self
                .participants
                .iter()
                .filter(|participant| participant.committed.0 >= level)
                .count() as u64;
            let amount = Chips((level - previous) * contributors);
            previous = level;
            let eligible_seats = self
                .participants
                .iter()
                .filter(|participant| !participant.folded && participant.committed.0 >= level)
                .map(|participant| participant.seat_no)
                .collect::<Vec<_>>();
            if eligible_seats.is_empty() {
                continue;
            }
            if let Some(pot) = pots
                .last_mut()
                .filter(|pot| pot.eligible_seats == eligible_seats)
            {
                pot.amount = pot.amount + amount;
            } else {
                pots.push(Pot {
                    amount,
                    eligible_seats,
                });
            }
        }
        pots
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PotAward {
    pub seat_no: SeatNo,
    pub amount: Chips,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluatedHand {
    pub category: HandCategory,
    pub best_five: [Card; 5],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandCategory {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

impl HandCategory {
    fn from_score(mut score: u64) -> Self {
        for _ in 0..5 {
            score /= 15;
        }
        match score {
            0 => Self::HighCard,
            1 => Self::OnePair,
            2 => Self::TwoPair,
            3 => Self::ThreeOfAKind,
            4 => Self::Straight,
            5 => Self::Flush,
            6 => Self::FullHouse,
            7 => Self::FourOfAKind,
            8 => Self::StraightFlush,
            _ => unreachable!("invalid hand category"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pot {
    pub amount: Chips,
    pub eligible_seats: Vec<SeatNo>,
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
pub struct ParticipantInfo {
    pub seat_no: SeatNo,
    pub stack: Chips,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandResult {
    stacks: Vec<(SeatNo, Chips)>,
}

impl HandResult {
    pub fn into_stacks(self) -> Vec<(SeatNo, Chips)> {
        self.stacks
    }
}

#[derive(Debug, Clone)]
struct Participant {
    seat_no: SeatNo,
    stack: Chips,
    current_bet: Chips,
    committed: Chips,
    folded: bool,
    cards: Vec<Card>,
    pending: bool,
}
impl Participant {
    fn new(info: &ParticipantInfo) -> Self {
        Self {
            seat_no: info.seat_no,
            stack: info.stack,
            current_bet: Chips(0),
            committed: Chips(0),
            folded: false,
            cards: vec![],
            pending: false,
        }
    }
    fn can_act(&self) -> bool {
        !self.folded && self.stack > Chips(0)
    }
}

// Cards use four contiguous suits of thirteen ranks (deuce through ace).
fn score_five(cards: [Card; 5]) -> u64 {
    let mut ranks: Vec<u8> = cards
        .iter()
        .map(|card| (card.order() % 13) as u8 + 2)
        .collect();
    ranks.sort_unstable_by(|left, right| right.cmp(left));
    let flush = cards
        .iter()
        .all(|card| card.order() / 13 == cards[0].order() / 13);
    let mut unique = ranks.clone();
    unique.dedup();
    let straight = if unique == [14, 5, 4, 3, 2] {
        Some(5)
    } else if unique.len() == 5 && unique[0] - unique[4] == 4 {
        Some(unique[0])
    } else {
        None
    };
    let mut groups: Vec<_> = unique
        .iter()
        .map(|rank| {
            (
                ranks.iter().filter(|candidate| *candidate == rank).count(),
                *rank,
            )
        })
        .collect();
    groups.sort_unstable_by(|left, right| right.cmp(left));
    let (category, values): (u8, Vec<u8>) = if flush && straight.is_some() {
        (8, vec![straight.unwrap()])
    } else if groups[0].0 == 4 {
        (7, vec![groups[0].1, groups[1].1])
    } else if groups[0].0 == 3 && groups[1].0 == 2 {
        (6, vec![groups[0].1, groups[1].1])
    } else if flush {
        (5, ranks)
    } else if let Some(high) = straight {
        (4, vec![high])
    } else if groups[0].0 == 3 {
        (3, groups.iter().map(|group| group.1).collect())
    } else if groups[0].0 == 2 && groups[1].0 == 2 {
        (2, groups.iter().map(|group| group.1).collect())
    } else if groups[0].0 == 2 {
        (1, groups.iter().map(|group| group.1).collect())
    } else {
        (0, ranks)
    };
    let mut score = category as u64;
    for index in 0..5 {
        score = score * 15 + values.get(index).copied().unwrap_or(0) as u64;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    fn hand(count: u8) -> Hand {
        Hand::new(
            Deck::new(std::array::from_fn(|i| Card::new(i as u8))),
            Blinds {
                small: Chips(50),
                big: Chips(100),
            },
            (0..count)
                .map(|i| ParticipantInfo {
                    seat_no: SeatNo(i),
                    stack: Chips(1000),
                })
                .collect(),
        )
    }
    #[test]
    fn starts_three_handed() {
        let mut h = hand(3);
        assert_eq!(
            h.start().last(),
            Some(&HandEvent::ActionRequested {
                seat_no: SeatNo(0),
                to_call: Chips(100),
                min_raise_to: Chips(200)
            })
        );
    }

    #[test]
    fn four_handed_hand_is_recorded_from_blinds_through_showdown() {
        let mut hand = hand(4);
        let mut events = hand.start();

        for (seat_no, action) in [
            (3, Action::Call),
            (0, Action::Call),
            (1, Action::Call),
            (2, Action::Check),
            (1, Action::Check),
            (2, Action::Bet(Chips(100))),
            (3, Action::Fold),
            (0, Action::Call),
            (1, Action::Fold),
            (2, Action::Check),
            (0, Action::Check),
            (2, Action::Check),
            (0, Action::Check),
        ] {
            events.extend(hand.act(SeatNo(seat_no), action).unwrap());
        }

        assert_eq!(
            events,
            vec![
                HandEvent::BlindPosted {
                    seat_no: SeatNo(1),
                    blind: Blind::Small,
                },
                HandEvent::ChipsCommitted {
                    seat_no: SeatNo(1),
                    amount: Chips(50),
                    current_bet: Chips(50),
                    remaining_stack: Chips(950),
                },
                HandEvent::BlindPosted {
                    seat_no: SeatNo(2),
                    blind: Blind::Big,
                },
                HandEvent::ChipsCommitted {
                    seat_no: SeatNo(2),
                    amount: Chips(100),
                    current_bet: Chips(100),
                    remaining_stack: Chips(900),
                },
                HandEvent::HoleCardsDealt {
                    seat_nos: vec![SeatNo(1), SeatNo(2), SeatNo(3), SeatNo(0)],
                },
                HandEvent::ActionRequested {
                    seat_no: SeatNo(3),
                    to_call: Chips(100),
                    min_raise_to: Chips(200),
                },
                HandEvent::PlayerActed {
                    seat_no: SeatNo(3),
                    action: Action::Call,
                },
                HandEvent::ChipsCommitted {
                    seat_no: SeatNo(3),
                    amount: Chips(100),
                    current_bet: Chips(100),
                    remaining_stack: Chips(900),
                },
                HandEvent::ActionRequested {
                    seat_no: SeatNo(0),
                    to_call: Chips(100),
                    min_raise_to: Chips(200),
                },
                HandEvent::PlayerActed {
                    seat_no: SeatNo(0),
                    action: Action::Call,
                },
                HandEvent::ChipsCommitted {
                    seat_no: SeatNo(0),
                    amount: Chips(100),
                    current_bet: Chips(100),
                    remaining_stack: Chips(900),
                },
                HandEvent::ActionRequested {
                    seat_no: SeatNo(1),
                    to_call: Chips(50),
                    min_raise_to: Chips(200),
                },
                HandEvent::PlayerActed {
                    seat_no: SeatNo(1),
                    action: Action::Call,
                },
                HandEvent::ChipsCommitted {
                    seat_no: SeatNo(1),
                    amount: Chips(50),
                    current_bet: Chips(100),
                    remaining_stack: Chips(900),
                },
                HandEvent::ActionRequested {
                    seat_no: SeatNo(2),
                    to_call: Chips(0),
                    min_raise_to: Chips(200),
                },
                HandEvent::PlayerActed {
                    seat_no: SeatNo(2),
                    action: Action::Check,
                },
                HandEvent::BettingRoundCompleted {
                    street: Street::Preflop,
                    pots: vec![Pot {
                        amount: Chips(400),
                        eligible_seats: vec![SeatNo(0), SeatNo(1), SeatNo(2), SeatNo(3)],
                    }],
                },
                HandEvent::CommunityCardsDealt {
                    street: Street::Flop,
                    cards: vec![Card::new(9), Card::new(10), Card::new(11)],
                },
                HandEvent::ActionRequested {
                    seat_no: SeatNo(1),
                    to_call: Chips(0),
                    min_raise_to: Chips(100),
                },
                HandEvent::PlayerActed {
                    seat_no: SeatNo(1),
                    action: Action::Check,
                },
                HandEvent::ActionRequested {
                    seat_no: SeatNo(2),
                    to_call: Chips(0),
                    min_raise_to: Chips(100),
                },
                HandEvent::PlayerActed {
                    seat_no: SeatNo(2),
                    action: Action::Bet(Chips(100)),
                },
                HandEvent::ChipsCommitted {
                    seat_no: SeatNo(2),
                    amount: Chips(100),
                    current_bet: Chips(100),
                    remaining_stack: Chips(800),
                },
                HandEvent::ActionRequested {
                    seat_no: SeatNo(3),
                    to_call: Chips(100),
                    min_raise_to: Chips(200),
                },
                HandEvent::PlayerActed {
                    seat_no: SeatNo(3),
                    action: Action::Fold,
                },
                HandEvent::ActionRequested {
                    seat_no: SeatNo(0),
                    to_call: Chips(100),
                    min_raise_to: Chips(200),
                },
                HandEvent::PlayerActed {
                    seat_no: SeatNo(0),
                    action: Action::Call,
                },
                HandEvent::ChipsCommitted {
                    seat_no: SeatNo(0),
                    amount: Chips(100),
                    current_bet: Chips(100),
                    remaining_stack: Chips(800),
                },
                HandEvent::ActionRequested {
                    seat_no: SeatNo(1),
                    to_call: Chips(100),
                    min_raise_to: Chips(200),
                },
                HandEvent::PlayerActed {
                    seat_no: SeatNo(1),
                    action: Action::Fold,
                },
                HandEvent::BettingRoundCompleted {
                    street: Street::Flop,
                    pots: vec![Pot {
                        amount: Chips(600),
                        eligible_seats: vec![SeatNo(0), SeatNo(2)],
                    }],
                },
                HandEvent::CommunityCardsDealt {
                    street: Street::Turn,
                    cards: vec![Card::new(13)],
                },
                HandEvent::ActionRequested {
                    seat_no: SeatNo(2),
                    to_call: Chips(0),
                    min_raise_to: Chips(100),
                },
                HandEvent::PlayerActed {
                    seat_no: SeatNo(2),
                    action: Action::Check,
                },
                HandEvent::ActionRequested {
                    seat_no: SeatNo(0),
                    to_call: Chips(0),
                    min_raise_to: Chips(100),
                },
                HandEvent::PlayerActed {
                    seat_no: SeatNo(0),
                    action: Action::Check,
                },
                HandEvent::BettingRoundCompleted {
                    street: Street::Turn,
                    pots: vec![Pot {
                        amount: Chips(600),
                        eligible_seats: vec![SeatNo(0), SeatNo(2)],
                    }],
                },
                HandEvent::CommunityCardsDealt {
                    street: Street::River,
                    cards: vec![Card::new(15)],
                },
                HandEvent::ActionRequested {
                    seat_no: SeatNo(2),
                    to_call: Chips(0),
                    min_raise_to: Chips(100),
                },
                HandEvent::PlayerActed {
                    seat_no: SeatNo(2),
                    action: Action::Check,
                },
                HandEvent::ActionRequested {
                    seat_no: SeatNo(0),
                    to_call: Chips(0),
                    min_raise_to: Chips(100),
                },
                HandEvent::PlayerActed {
                    seat_no: SeatNo(0),
                    action: Action::Check,
                },
                HandEvent::BettingRoundCompleted {
                    street: Street::River,
                    pots: vec![Pot {
                        amount: Chips(600),
                        eligible_seats: vec![SeatNo(0), SeatNo(2)],
                    }],
                },
                HandEvent::ShowdownStarted {
                    seat_nos: vec![SeatNo(0), SeatNo(2)],
                },
                HandEvent::HoleCardsShown {
                    seat_no: SeatNo(0),
                    cards: [Card::new(3), Card::new(7)],
                    hand: EvaluatedHand {
                        category: HandCategory::Flush,
                        best_five: [
                            Card::new(9),
                            Card::new(10),
                            Card::new(11),
                            Card::new(3),
                            Card::new(7),
                        ],
                    },
                },
                HandEvent::HoleCardsShown {
                    seat_no: SeatNo(2),
                    cards: [Card::new(1), Card::new(5)],
                    hand: EvaluatedHand {
                        category: HandCategory::Flush,
                        best_five: [
                            Card::new(9),
                            Card::new(10),
                            Card::new(11),
                            Card::new(1),
                            Card::new(5),
                        ],
                    },
                },
                HandEvent::PotAwarded {
                    amount: Chips(600),
                    eligible_seats: vec![SeatNo(0), SeatNo(2)],
                    awards: vec![PotAward {
                        seat_no: SeatNo(0),
                        amount: Chips(600),
                    }],
                },
                HandEvent::HandFinished,
            ]
        );
    }
    #[test]
    fn fold_ends_heads_up_hand() {
        let mut h = hand(2);
        h.start();
        let events = h.act(SeatNo(0), Action::Fold).unwrap();
        assert!(events.contains(&HandEvent::ChipsReturned {
            seat_no: SeatNo(1),
            amount: Chips(50),
            remaining_stack: Chips(950),
        }));
        assert!(events.contains(&HandEvent::BettingRoundCompleted {
            street: Street::Preflop,
            pots: vec![Pot {
                amount: Chips(100),
                eligible_seats: vec![SeatNo(1)],
            }],
        }));
        assert!(events.contains(&HandEvent::PotAwarded {
            amount: Chips(100),
            eligible_seats: vec![SeatNo(1)],
            awards: vec![PotAward {
                seat_no: SeatNo(1),
                amount: Chips(100),
            }],
        }));
    }

    #[test]
    fn returns_uncalled_chips_and_completes_only_the_actual_betting_round() {
        let mut hand = Hand::new(
            Deck::new(std::array::from_fn(|index| Card::new(index as u8))),
            Blinds {
                small: Chips(50),
                big: Chips(100),
            },
            vec![
                ParticipantInfo {
                    seat_no: SeatNo(0),
                    stack: Chips(1000),
                },
                ParticipantInfo {
                    seat_no: SeatNo(1),
                    stack: Chips(1000),
                },
                ParticipantInfo {
                    seat_no: SeatNo(2),
                    stack: Chips(500),
                },
            ],
        );
        hand.start();
        hand.act(SeatNo(0), Action::RaiseTo(Chips(1000))).unwrap();
        hand.act(SeatNo(1), Action::Fold).unwrap();

        let events = hand.act(SeatNo(2), Action::Call).unwrap();

        let returned = events
            .iter()
            .position(|event| matches!(event, HandEvent::ChipsReturned { .. }))
            .unwrap();
        let completed = events
            .iter()
            .position(|event| matches!(event, HandEvent::BettingRoundCompleted { .. }))
            .unwrap();
        let flop = events
            .iter()
            .position(|event| {
                matches!(
                    event,
                    HandEvent::CommunityCardsDealt {
                        street: Street::Flop,
                        ..
                    }
                )
            })
            .unwrap();
        assert!(returned < completed && completed < flop);
        assert_eq!(
            events[returned],
            HandEvent::ChipsReturned {
                seat_no: SeatNo(0),
                amount: Chips(500),
                remaining_stack: Chips(500),
            }
        );
        assert_eq!(
            events[completed],
            HandEvent::BettingRoundCompleted {
                street: Street::Preflop,
                pots: vec![Pot {
                    amount: Chips(1050),
                    eligible_seats: vec![SeatNo(0), SeatNo(2)],
                }],
            }
        );
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event, HandEvent::BettingRoundCompleted { .. }))
                .count(),
            1
        );
    }

    #[test]
    fn start_finishes_hand_when_both_participants_are_all_in_from_blinds() {
        let mut hand = Hand::new(
            Deck::new(std::array::from_fn(|index| Card::new(index as u8))),
            Blinds {
                small: Chips(50),
                big: Chips(100),
            },
            vec![
                ParticipantInfo {
                    seat_no: SeatNo(0),
                    stack: Chips(50),
                },
                ParticipantInfo {
                    seat_no: SeatNo(1),
                    stack: Chips(50),
                },
            ],
        );

        let events = hand.start();

        assert_eq!(
            &events[..5],
            &[
                HandEvent::BlindPosted {
                    seat_no: SeatNo(0),
                    blind: Blind::Small,
                },
                HandEvent::ChipsCommitted {
                    seat_no: SeatNo(0),
                    amount: Chips(50),
                    current_bet: Chips(50),
                    remaining_stack: Chips(0),
                },
                HandEvent::BlindPosted {
                    seat_no: SeatNo(1),
                    blind: Blind::Big,
                },
                HandEvent::ChipsCommitted {
                    seat_no: SeatNo(1),
                    amount: Chips(50),
                    current_bet: Chips(50),
                    remaining_stack: Chips(0),
                },
                HandEvent::HoleCardsDealt {
                    seat_nos: vec![SeatNo(1), SeatNo(0)],
                },
            ]
        );
        assert!(events.iter().any(|event| matches!(
            event,
            HandEvent::CommunityCardsDealt {
                street: Street::Flop,
                ..
            }
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            HandEvent::CommunityCardsDealt {
                street: Street::Turn,
                ..
            }
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            HandEvent::CommunityCardsDealt {
                street: Street::River,
                ..
            }
        )));
        assert!(events.contains(&HandEvent::ShowdownStarted {
            seat_nos: vec![SeatNo(0), SeatNo(1)],
        }));
        assert!(
            !events
                .iter()
                .any(|event| matches!(event, HandEvent::ActionRequested { .. }))
        );
        assert_eq!(events.last(), Some(&HandEvent::HandFinished));
        assert!(hand.is_finished());
        assert_eq!(
            hand.stacks().iter().map(|(_, stack)| stack.0).sum::<u64>(),
            100
        );
    }

    #[test]
    fn five_handed_hand_with_two_flop_folds_reaches_three_way_showdown() {
        let mut hand = hand(5);
        let mut events = hand.start();

        // Preflop: UTG calls, the cutoff raises, and all five players see the flop.
        for (seat_no, action) in [
            (3, Action::Call),
            (4, Action::RaiseTo(Chips(200))),
            (0, Action::Call),
            (1, Action::Call),
            (2, Action::Call),
            (3, Action::Call),
        ] {
            events.extend(hand.act(SeatNo(seat_no), action).unwrap());
        }
        assert!(events.contains(&HandEvent::CommunityCardsDealt {
            street: Street::Flop,
            cards: vec![Card::new(11), Card::new(12), Card::new(13)],
        }));

        // The big blind leads, one player folds, the button raises, and a second
        // player folds. Seats 0, 2, and 3 continue to the turn.
        for (seat_no, action) in [
            (1, Action::Check),
            (2, Action::Bet(Chips(100))),
            (3, Action::Call),
            (4, Action::Fold),
            (0, Action::RaiseTo(Chips(300))),
            (1, Action::Fold),
            (2, Action::Call),
            (3, Action::Call),
        ] {
            events.extend(hand.act(SeatNo(seat_no), action).unwrap());
        }
        assert!(events.contains(&HandEvent::CommunityCardsDealt {
            street: Street::Turn,
            cards: vec![Card::new(15)],
        }));

        // The remaining three players check the turn and river to showdown.
        for seat_no in [2, 3, 0, 2, 3, 0] {
            events.extend(hand.act(SeatNo(seat_no), Action::Check).unwrap());
        }

        assert!(events.contains(&HandEvent::CommunityCardsDealt {
            street: Street::River,
            cards: vec![Card::new(17)],
        }));
        assert!(events.contains(&HandEvent::ShowdownStarted {
            seat_nos: vec![SeatNo(0), SeatNo(2), SeatNo(3)],
        }));
        let seat_zero_winnings = events
            .iter()
            .filter_map(|event| match event {
                HandEvent::PotAwarded { awards, .. } => awards
                    .iter()
                    .find(|award| award.seat_no == SeatNo(0))
                    .map(|award| award.amount.0),
                _ => None,
            })
            .sum::<u64>();
        assert_eq!(seat_zero_winnings, 1900);
        let awarded_pots: Vec<_> = events
            .iter()
            .filter_map(|event| match event {
                HandEvent::PotAwarded { amount, awards, .. } => Some((amount, awards)),
                _ => None,
            })
            .collect();
        let final_pot_count = events
            .iter()
            .rev()
            .find_map(|event| match event {
                HandEvent::BettingRoundCompleted { pots, .. } => Some(pots.len()),
                _ => None,
            })
            .unwrap();
        assert_eq!(awarded_pots.len(), final_pot_count);
        assert!(awarded_pots.iter().all(|(amount, awards)| {
            awards.iter().map(|award| award.amount.0).sum::<u64>() == amount.0
        }));
        assert_eq!(events.last(), Some(&HandEvent::HandFinished));
        assert!(hand.is_finished());
        assert_eq!(
            hand.stacks(),
            vec![
                (SeatNo(0), Chips(2400)),
                (SeatNo(1), Chips(800)),
                (SeatNo(2), Chips(500)),
                (SeatNo(3), Chips(500)),
                (SeatNo(4), Chips(800)),
            ]
        );
    }

    #[test]
    fn pocket_kings_crack_pocket_aces_after_both_players_are_all_in_preflop() {
        // Heads-up cards are dealt to seat 1 first. The first four cards therefore
        // give kings to seat 1 and aces to seat 0. Card 37 puts a third king on
        // the flop; the mixed-suit board cannot improve the aces past three kings.
        // Positions 4, 8, and 10 are burn cards.
        let prefix = [11, 12, 24, 25, 0, 37, 14, 28, 1, 43, 2, 19];
        let mut card_values = prefix.to_vec();
        card_values.extend((0..52).filter(|value| !prefix.contains(value)));
        let deck = Deck::new(std::array::from_fn(|index| Card::new(card_values[index])));
        let mut hand = Hand::new(
            deck,
            Blinds {
                small: Chips(50),
                big: Chips(100),
            },
            vec![
                ParticipantInfo {
                    seat_no: SeatNo(0),
                    stack: Chips(1000),
                },
                ParticipantInfo {
                    seat_no: SeatNo(1),
                    stack: Chips(1000),
                },
            ],
        );

        hand.start();
        assert_eq!(
            hand.participants[0].cards,
            vec![Card::new(12), Card::new(25)]
        );
        assert_eq!(
            hand.participants[1].cards,
            vec![Card::new(11), Card::new(24)]
        );

        hand.act(SeatNo(0), Action::RaiseTo(Chips(1000))).unwrap();
        let events = hand.act(SeatNo(1), Action::Call).unwrap();

        assert!(events.contains(&HandEvent::CommunityCardsDealt {
            street: Street::Flop,
            cards: vec![Card::new(37), Card::new(14), Card::new(28)],
        }));
        assert!(events.contains(&HandEvent::ShowdownStarted {
            seat_nos: vec![SeatNo(0), SeatNo(1)],
        }));
        assert!(
            events.contains(&HandEvent::PotAwarded {
                amount: Chips(2000),
                eligible_seats: vec![SeatNo(0), SeatNo(1)],
                awards: vec![PotAward {
                    seat_no: SeatNo(1),
                    amount: Chips(2000),
                }],
            }),
            "unexpected settlement events: {events:?}"
        );
        assert_eq!(events.last(), Some(&HandEvent::HandFinished));
        assert_eq!(
            hand.stacks(),
            vec![(SeatNo(0), Chips(0)), (SeatNo(1), Chips(2000))]
        );
    }
}
