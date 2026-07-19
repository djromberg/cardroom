use std::cmp::min;
use std::assert_matches;

use super::card::Card;
use super::dealer::Dealer;
use super::deck::Deck;
use super::shared::Chips;
use super::shared::SeatNo;


#[derive(Debug, Clone)]
pub struct Hand {
    dealer: Dealer,
    participants: Vec<Participant>,
    current_pos: usize,
    street: Street,
    is_started: bool,
}

impl Hand {
    // TODO: add Deck and Blinds to construction
    pub fn new(deck: Deck, participants: Vec<ParticipantInfo>) -> Self {
        assert!(participants.len() >= 2 && participants.len() <= 10);
        Self {
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

    pub fn whos_turn(&self) -> Option<SeatNo> {
        None
    }

    pub fn check(&mut self, seat_no: SeatNo) -> Vec<HandEvent> {
        assert_eq!(self.participants[self.current_pos].seat_no(), seat_no);
        vec![]
    }

    // TODO: make blinds configurable at construction
    fn pay_blinds(&mut self) -> Vec<HandEvent> {
        let mut events = vec![];
        events.push(self.pay_blind(Chips(50)));
        self.advance_pos();
        events.push(self.pay_blind(Chips(100)));
        events
    }

    fn pay_blind(&mut self, amount: Chips) -> HandEvent {
        let participant = &mut self.participants[self.current_pos];
        let placed_chips = participant.take_chips(amount);
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
        let seat_nos = self.participants
            .iter()
            .map(|p| p.seat_no)
            .collect();
        HandEvent::HoleCardsDealt { seat_nos }
    }

    fn deal_hole_card(&mut self) {
        // TODO: think about starting card dealing under the gun
        //       does it make a statistic/stochastic difference?
        for participant in &mut self.participants {
            participant.deal_card(self.dealer.deal_card());
        }
    }

    fn run(&mut self) -> Vec<HandEvent> {
        let mut events = vec![];
        if self.is_any_participant_active() {
            events.extend(self.finish_street_if_possible());
            events.extend(self.move_to_next_participant());
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

    fn is_any_participant_active(&self) -> bool {
        true
    }

    fn finish_street_if_possible(&mut self) -> Vec<HandEvent> {
        vec![]
    }

    fn move_to_next_participant(&mut self) -> Vec<HandEvent> {
        vec![]
    }

    fn showdown(&mut self) -> Vec<HandEvent> {
        vec![]
    }

    fn advance_pos(&mut self) {
        self.current_pos = (self.current_pos + 1) % self.participants.len();
    }
}


#[derive(Debug, Clone)]
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
    CardDealt {
        seat_no: SeatNo,
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

    pub fn take_chips(&mut self, amount: Chips) -> Chips {
        let valid_amount = min(amount, self.stack);
        self.stack = self.stack - valid_amount;
        valid_amount
    }

    pub fn increase_action_count(&mut self) {
        self.action_count += 1;
    }
}
