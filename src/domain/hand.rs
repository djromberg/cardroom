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
    street: Street,
    is_started: bool,
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
            street: Street::Preflop,
            is_started: false,
        }
    }

    pub fn start(&mut self) -> Vec<HandEvent> {
        assert!(!self.is_started);
        self.is_started = true;
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
        if self.is_any_participant_active() {
            events.extend(self.run_current_round());
        } else {
            events.extend(self.run_until_complete());
        }
        events
    }

    fn run_until_complete(&mut self) -> Vec<HandEvent> {
        let mut events = vec![];
        while !self.is_finished() {
            events.extend(self.finish_street_if_possible());
        }
        events.extend(self.showdown());
        events
    }

    fn is_finished(&mut self) -> bool {
        false
    }

    fn is_heads_up(&self) -> bool {
        self.participants.len() == 2
    }

    fn is_any_participant_active(&self) -> bool {
        true
    }

    fn finish_street_if_possible(&mut self) -> Vec<HandEvent> {
        vec![]
    }

    fn run_current_round(&mut self) -> Vec<HandEvent> {
        let mut events = vec![];
        events.extend(self.finish_street_if_possible());
        events.push(self.request_action_from_next_active_participant());
        events
    }

    fn showdown(&mut self) -> Vec<HandEvent> {
        vec![]
    }

    fn request_action_from_next_active_participant(&mut self) -> HandEvent {
        self.advance_pos();
        // TODO find next active player
        HandEvent::ActionRequested { seat_no: SeatNo(0) }
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
    },
}


#[derive(Debug, Clone)]
pub struct ParticipantInfo {
    seat_no: SeatNo,
    stack: Chips,
}


#[derive(Debug, Clone)]
enum Street {
    Preflop,
    Flop,
    Turn,
    River,
}

#[derive(Debug, Clone)]
struct Participant {
    seat_no: SeatNo,
    stack: Chips,
    current_bet: Chips,
    has_folded: bool,
    cards: Vec<Card>,
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
            action_count: 0,
        }
    }

    pub fn seat_no(&self) -> SeatNo {
        self.seat_no
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
    fn start() {
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
                HandEvent::ActionRequested { seat_no: SeatNo(0) }
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