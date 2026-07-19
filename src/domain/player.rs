// use super::hand::Hand;

use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct Player {
    position: u8,
    data: PlayerData,
    // hand: Option<Hand>
}

impl Player {
    pub fn new(position: u8, data: PlayerData) -> Self {
        Self { position, data }
    }

    pub fn position(&self) -> u8 {
        self.position
    }

    pub fn id(&self) -> Uuid {
        self.data.player_id
    }

    pub fn stack(&self) -> u32 {
        self.data.stack
    }

    // pub fn current_bet_sum(&self) -> Option<u32> {
    //     self.hand.as_ref().map(|hand| hand.bet_sum())
    // }

    // pub fn start_hand(&mut self) {
    //     assert!(self.hand.is_none());
    //     self.hand = Some(Hand::new());
    // }

    // pub fn deal_card(&mut self, card: Card) {
    //     let hand = self.hand.as_mut().unwrap();
    //     hand.receive_card(card);
    // }

    // pub fn place_bet(&mut self, amount: u32) -> u32 {
    //     let hand = self.hand.as_mut().unwrap();
    //     let amount = std::cmp::min(self.data.stack, amount);
    //     self.data.stack -= amount;
    //     hand.bet(amount);
    //     amount
    // }
}


#[derive(Debug, Clone, PartialEq)]
pub struct PlayerData {
    player_id: Uuid,
    nickname: String,
    stack: u32,
}

impl PlayerData {
    pub fn new(player_id: Uuid, nickname: String, stack: u32) -> Self {
        Self { player_id, nickname, stack }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }
}
