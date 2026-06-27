use super::card::Card;

const DECK_SIZE: usize = 52;


#[derive(Debug, Clone)]
pub struct Deck {
    cards: [Card; DECK_SIZE],
    next: usize,
}

impl Deck {
    pub fn new(cards: [Card; DECK_SIZE]) -> Self {
        assert_valid_cards(&cards);
        Self { cards, next: 0 }
    }

    pub fn is_untouched(&self) -> bool {
        self.next == 0
    }

    pub fn draw_card(&mut self) -> Card {
        let card = self.cards[self.next];
        self.next += 1;
        card
    }
}


trait Shuffle {
    fn shuffle(&self) -> Deck;
}


fn assert_valid_cards(cards: &[Card; DECK_SIZE]) {
    let mut seen = [false; DECK_SIZE];

    for card in cards {
        let value = card.value() as usize;
        assert!(!seen[value], "duplicate card: {value}");
        seen[value] = true;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "duplicate card: 0")]
    fn new_with_duplicate_cards() {
        let cards = std::array::from_fn(|_| Card::new(0));
        Deck::new(cards);
    }

    #[test]
    fn new() {
        let cards = std::array::from_fn(|i| Card::new(i as u8));
        let deck = Deck::new(cards);
        assert!(deck.is_untouched());
    }

    #[test]
    fn draw_card() {
        let cards = std::array::from_fn(|i| Card::new(i as u8));
        let mut deck = Deck::new(cards);
        let card = deck.draw_card();
        assert_eq!(card, Card::new(0));
        assert!(!deck.is_untouched());
    }

}