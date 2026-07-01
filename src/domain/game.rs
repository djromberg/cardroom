use super::card::Card;
use super::dealer::Dealer;
use super::deck::Deck;


#[derive(Debug, Clone)]
pub struct Game {
    dealer: Dealer,
    board: Vec<Card>,
    pots: Vec<u32>,
}

impl Game {
    pub fn new(deck: Deck) -> Self {
        let dealer = Dealer::new(deck);
        Self { dealer, board: vec![], pots: vec![] }
    }

    pub fn deal_player_card(&mut self) -> Card {
        self.dealer.deal_card()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

}
