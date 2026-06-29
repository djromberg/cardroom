use super::card::Card;

const DECK_SIZE: usize = 52;


#[derive(Debug, Clone)]
pub struct Deck {
    cards: [Card; DECK_SIZE],
}

impl Deck {
    pub fn new(cards: [Card; DECK_SIZE]) -> Self {
        Self::assert_valid(&cards);
        Self { cards }
    }

    pub fn at(&self, position: usize) -> Card {
        self.cards[position]
    }

    fn assert_valid(cards: &[Card; DECK_SIZE]) {
        let mut seen = [false; DECK_SIZE];

        for card in cards {
            let order = card.order();
            assert!(!seen[order], "duplicate card found");
            seen[order] = true;
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "duplicate card found")]
    fn new_with_duplicate_cards() {
        let cards = std::array::from_fn(|_| Card::new(0));
        Deck::new(cards);
    }

    #[test]
    fn at() {
        let cards = std::array::from_fn(|i| Card::new(i as u8));
        let sequence = Deck::new(cards);
        assert_eq!(sequence.at(42), Card::new(42));
    }

}
