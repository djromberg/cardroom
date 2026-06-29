use super::card::Card;
use super::deck::Deck;


#[derive(Debug, Clone)]
pub struct Dealer {
    deck: Deck,
    next: usize,
}

impl Dealer {
    pub fn new(deck: Deck) -> Self {
        Self { deck, next: 0 }
    }

    pub fn deal_card(&mut self) -> Card {
        let card = self.deck.at(self.next);
        self.next += 1;
        card
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deal_card() {
        let cards = std::array::from_fn(|i| Card::new(i as u8));
        let deck = Deck::new(cards);
        let mut dealer = Dealer::new(deck);
        assert_eq!(dealer.deal_card(), Card::new(0));
    }
}
