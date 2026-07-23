use std::cmp::min;

use super::card::Card;
use super::dealer::Dealer;
use super::deck::Deck;
use super::shared::Blinds;
use super::shared::Chips;
use super::shared::SeatNo;


#[derive(Debug, Clone)]
pub struct Hand {
    blinds: Blinds,
    dealer: Dealer,
    participants: Vec<Participant>,
    current_pos: usize,
    stage: Stage,
}

impl Hand {
    // TODO: add Deck and Blinds to construction
    pub fn new(deck: Deck, blinds: Blinds, participants: Vec<ParticipantInfo>) -> Self {
        assert!(participants.len() >= 2 && participants.len() <= 10);
        Self {
            blinds,
            dealer: Dealer::new(deck),
            participants: participants
                .iter()
                .map(|p| Participant::new(&p))
                .collect(),
            current_pos: if participants.len() == 2 { 0 } else { 1 },
            stage: Stage::Ready,
        }
    }

    pub fn start(&mut self) -> Vec<HandEvent> {
        assert_eq!(self.stage, Stage::Ready);
        self.stage = self.stage.next();
        let mut events = vec![];
        events.extend(self.pay_blinds());
        events.push(self.deal_hole_cards());
        events.extend(self.run());
        events
    }

    fn pay_blinds(&mut self) -> Vec<HandEvent> {
        let mut events = vec![];
        events.push(self.pay_blind(self.blinds.small));
        self.advance_pos();
        events.push(self.pay_blind(self.blinds.big));
        self.advance_pos();
        events
    }

    fn pay_blind(&mut self, amount: Chips) -> HandEvent {
        let participant = &mut self.participants[self.current_pos];
        let placed_chips = participant.place_chips(amount);
        HandEvent::ChipsPlaced {
            seat_no: participant.seat_no,
            amount: placed_chips,
            current_bet: participant.current_bet,
            remaining_stack: participant.stack,
        }
    }

    fn deal_hole_cards(&mut self) -> HandEvent {
        self.deal_hole_card();
        self.deal_hole_card();

        let (dealer, after_dealer) = self.participants.split_at(1);
        let seat_nos: Vec<_> = after_dealer
            .iter()
            .chain(dealer.iter())
            .map(|participant| participant.seat_no())
            .collect();
        HandEvent::HoleCardsDealt { seat_nos }
    }

    fn deal_hole_card(&mut self) {
        for position in 1..self.participants.len() - 1 {
            self.participants[position].deal_card(self.dealer.deal_card());
        }
        self.participants[0].deal_card(self.dealer.deal_card());
    }

    fn run(&mut self) -> Vec<HandEvent> {
        let mut events = vec![];
        if self.is_only_one_participant_left() {
            events.extend(self.end_early());
        } else if self.is_participant_action_required() {
            events.push(self.request_action_from_next_active_participant());
        } else {
            events.extend(self.run_until_finished());
        }
        events
    }

    fn run_until_finished(&mut self) -> Vec<HandEvent> {
        let mut events = vec![];
        while self.is_round_in_progress() {
            events.extend(self.finish_round());
        }
        events
    }

    fn finish_round(&mut self) -> Vec<HandEvent> {
        let mut events = vec![];
        // events.extend(self.collect_bets());
        self.stage = self.stage.next();
        if self.is_round_in_progress() {
            self.current_pos = 0;
        } else {
            events.extend(self.showdown());
            // events.extend(self.payout());
        }
        events
    }

    fn is_round_in_progress(&self) -> bool {
        matches!(self.stage, Stage::Street(_))
    }

    fn is_participant_action_required(&self) -> bool {
        true
        // self.action_count_and_bet_of_active_players_match
    }

    fn is_only_one_participant_left(&mut self) -> bool {
        self.participants.iter().filter(|p| p.has_folded()).count() == 1
    }

    fn are_at_least_two_participants_active(&self) -> bool {
        self.participants.iter().filter(|p| p.can_act()).count() >= 2
    }

    fn is_heads_up(&self) -> bool {
        self.participants.len() == 2
    }

    fn is_any_participant_active(&self) -> bool {
        true
    }

    fn end_early(&mut self) -> Vec<HandEvent> {
        let mut events = vec![];
        // events.extend(self.collect_bets());
        // events.extend(self.payout());
        self.stage = Stage::Over;
        events
    }

    fn showdown(&mut self) -> Vec<HandEvent> {
        vec![]
    }

    fn request_action_from_next_active_participant(&mut self) -> HandEvent {
        while !self.current_participant().can_act() {
            self.advance_pos();
        }

        // temp workaround just for testing
        let round_state = RoundState {
            max_bet: self.blinds.big,
            min_raise: self.blinds.big,
        };
        self.current_participant_mut().request_action(round_state)
    }

    fn current_participant(&self) -> &Participant {
        &self.participants[self.current_pos]
    }

    fn current_participant_mut(&mut self) -> &mut Participant {
        &mut self.participants[self.current_pos]
    }

    fn advance_pos(&mut self) {
        self.current_pos = (self.current_pos + 1) % self.participants.len();
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum HandEvent {
    ChipsPlaced {
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
        min_bet: Chips,
        min_raise: Chips,
    },
}


#[derive(Debug, Clone)]
pub struct ParticipantInfo {
    seat_no: SeatNo,
    stack: Chips,
}


#[derive(Debug, Clone, PartialEq)]
enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

impl Street {
    pub fn next(&self) -> Option<Street> {
        match self {
            Self::Preflop => Some(Self::Flop),
            Self::Flop => Some(Self::Turn),
            Self::Turn => Some(Self::River),
            Self::River => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Stage {
    Ready,
    Street(Street),
    Over,
}

impl Stage {
    pub fn next(&self) -> Stage {
        match self {
            Self::Ready => Self::Street(Street::Preflop),
            Self::Street(street) => street.next().map_or(Self::Over, |next| Self::Street(next)),
            Self::Over => panic!(),
        }
    }
}


#[derive(Debug, Clone)]
struct RoundState {
    max_bet: Chips,
    min_raise: Chips,
}


#[derive(Debug, Clone, PartialEq)]
struct ActionRequest {
    min_bet: Chips,
    min_raise: Chips,
}


#[derive(Debug, Clone)]
struct Participant {
    seat_no: SeatNo,
    stack: Chips,
    current_bet: Chips,
    has_folded: bool,
    cards: Vec<Card>,
    action_request: Option<ActionRequest>,
    action_count: u16,
}


impl Participant {
    pub fn new(info: &ParticipantInfo) -> Self {
        Self {
            seat_no: info.seat_no,
            stack: info.stack,
            current_bet: Chips(0),
            has_folded: false,
            cards: vec![],
            action_request: None,
            action_count: 0,
        }
    }

    pub fn seat_no(&self) -> SeatNo {
        self.seat_no
    }

    pub fn has_folded(&self) -> bool {
        self.has_folded
    }

    pub fn can_act(&self) -> bool {
        !self.has_folded && self.stack > Chips(0)
    }

    pub fn request_action(&mut self, round_state: RoundState) -> HandEvent {
        assert!(self.action_request.is_none());
        let min_bet = min(round_state.max_bet - self.current_bet, self.stack);
        let min_raise = round_state.min_raise;
        self.action_request = Some(
            ActionRequest { min_bet, min_raise }
        );
        HandEvent::ActionRequested {
            seat_no: self.seat_no,
            min_bet,
            min_raise,
        }
    }

    pub fn deal_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn place_chips(&mut self, amount: Chips) -> Chips {
        let capped_amount = min(amount, self.stack);
        self.stack = self.stack - capped_amount;
        self.current_bet = self.current_bet + capped_amount;
        capped_amount
    }

    pub fn increase_action_count(&mut self) {
        self.action_count += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_regular() {
        let mut hand = create_hand(
            3,
            Chips(1000),
            Blinds {
                small: Chips(50),
                big: Chips(100),
            });

        assert_eq!(
            hand.start(),
            vec![
                HandEvent::ChipsPlaced { seat_no: SeatNo(1), amount: Chips(50), current_bet: Chips(50), remaining_stack: Chips(950) },
                HandEvent::ChipsPlaced { seat_no: SeatNo(2), amount: Chips(100), current_bet: Chips(100), remaining_stack: Chips(900) },
                HandEvent::HoleCardsDealt { seat_nos: vec![SeatNo(1), SeatNo(2), SeatNo(0)] },
                HandEvent::ActionRequested { seat_no: SeatNo(0), min_bet: Chips(100), min_raise: Chips(100) }
            ]
        );
    }

    #[test]
    fn start_heads_up() {
        let mut hand = create_hand(
            2,
            Chips(1000),
            Blinds {
                small: Chips(50),
                big: Chips(100),
            });

        assert_eq!(
            hand.start(),
            vec![
                HandEvent::ChipsPlaced { seat_no: SeatNo(0), amount: Chips(50), current_bet: Chips(50), remaining_stack: Chips(950) },
                HandEvent::ChipsPlaced { seat_no: SeatNo(1), amount: Chips(100), current_bet: Chips(100), remaining_stack: Chips(900) },
                HandEvent::HoleCardsDealt { seat_nos: vec![SeatNo(1), SeatNo(0)] },
                HandEvent::ActionRequested { seat_no: SeatNo(0), min_bet: Chips(50), min_raise: Chips(100) }
            ]
        );
    }

    fn create_hand(participant_count: u8, participant_stack: Chips, blinds: Blinds) -> Hand {
        let cards = std::array::from_fn(|i| Card::new(i as u8));
        let deck = Deck::new(cards);
        let mut participants = vec![];
        for i in 0..participant_count {
            participants.push(
                ParticipantInfo {
                    seat_no: SeatNo(i),
                    stack: participant_stack,
                }
            )
        }
        Hand::new(deck, blinds, participants)
    }

}