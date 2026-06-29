use super::card::Card;
use super::deck::Deck;


#[derive(Debug, Clone)]
pub struct Game {
    deck: Deck,
    deck_index: usize,
    board: Vec<Card>,
    pots: Vec<u32>,
}

impl Game {
    pub fn new(deck: Deck) -> Self {
        assert!(deck.is_untouched());
        Self { deck, board: vec![], pots: vec![] }
    }

    pub fn deal_player_card(&mut self) -> Card {
        self.deck.draw_card()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

}
