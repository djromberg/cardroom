use super::card::Card;

use std::cmp;


#[derive(Debug, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    bet_sum: u32,
    folded: bool,
}

impl Hand {
    pub fn new() -> Self {
        Self { cards: vec![], bet_sum: 0, folded: false }
    }

    pub fn bet_sum(&self) -> u32 {
        self.bet_sum
    }

    pub fn bet(&mut self, amount: u32) {
        self.bet_sum += amount;
    }

    pub fn receive_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn collect(&mut self, amount: u32) -> u32 {
        let result = cmp::min(self.bet_sum, amount);
        self.bet_sum -= result;
        result
    }
}
