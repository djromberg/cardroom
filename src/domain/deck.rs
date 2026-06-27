use super::card::Card;

const DECK_SIZE: usize = 52;


#[derive(Debug)]
pub struct Deck {
    shuffle: [usize; DECK_SIZE],
    next: usize,
}

impl Deck {
    pub fn new(shuffle: [usize; DECK_SIZE]) -> Self {
        assert_valid_shuffle(&shuffle);
        Self { shuffle, next: 0 }
    }

    pub fn cards_drawn(&self) -> usize {
        self.next
    }

    pub fn draw_card(&mut self) -> Card {
        let result = Card(self.next as u8);
        self.next += 1;
        result
    }
}


fn assert_valid_shuffle(indices: &[usize; DECK_SIZE]) {
    let mut seen = [false; DECK_SIZE];

    for &index in indices {
        assert!(
            index < DECK_SIZE,
            "invalid card: {index}; expected 0..{}",
            DECK_SIZE - 1
        );

        assert!(
            !seen[index],
            "duplicate cards: {index}"
        );

        seen[index] = true;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "duplicate cards: 0")]
    fn new_with_duplicate_cards() {
        let shuffle= [0usize; DECK_SIZE];
        Deck::new(shuffle);
    }

    #[test]
    #[should_panic(expected = "invalid card: 52; expected 0..51")]
    fn new_with_invalid_card() {
        let mut shuffle: [usize; 52] = std::array::from_fn(|i| i);
        shuffle[42] = 52;
        Deck::new(shuffle);
    }

    #[test]
    fn new() {
        let shuffle: [usize; 52] = std::array::from_fn(|i| i);
        let deck = Deck::new(shuffle);
        assert_eq!(deck.cards_drawn(), 0);
    }

    #[test]
    fn draw_card() {
        let shuffle: [usize; 52] = std::array::from_fn(|i| i);
        let mut deck = Deck::new(shuffle);
        let card = deck.draw_card();
        assert_eq!(card, Card(0));
        assert_eq!(deck.cards_drawn(), 1);
    }

}