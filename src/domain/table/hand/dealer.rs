use crate::domain::table::{card::Card, deck::Deck};

#[derive(Debug, Clone)]
pub(super) struct Dealer {
    deck: Deck,
    next: usize,
}

impl Dealer {
    pub(super) fn new(deck: Deck) -> Self {
        Self { deck, next: 0 }
    }

    pub(super) fn deal_card(&mut self) -> Card {
        let card = self.deck.at(self.next);
        self.next += 1;
        card
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::table::hand::dealer::*;

    #[test]
    fn deal_card() {
        let cards = std::array::from_fn(|i| Card::from_index(i as u8));
        let deck = Deck::new(cards).unwrap();
        let mut dealer = Dealer::new(deck);
        assert_eq!(dealer.deal_card(), Card::from_index(0));
    }
}
