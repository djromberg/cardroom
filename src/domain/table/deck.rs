use super::card::Card;

const DECK_SIZE: usize = 52;

#[derive(Debug, Clone)]
pub struct Deck {
    cards: [Card; DECK_SIZE],
}

impl Deck {
    pub fn new(cards: [Card; DECK_SIZE]) -> Result<Self, InvalidDeck> {
        Self::validate(&cards)?;
        Ok(Self { cards })
    }

    pub(super) fn at(&self, position: usize) -> Card {
        self.cards[position]
    }

    fn validate(cards: &[Card; DECK_SIZE]) -> Result<(), InvalidDeck> {
        let mut seen = [false; DECK_SIZE];

        for card in cards {
            let index = card.index();
            if seen[index] {
                return Err(InvalidDeck::DuplicateCard(*card));
            }
            seen[index] = true;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidDeck {
    DuplicateCard(Card),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::table::card::{Rank, Suit};

    #[test]
    fn new_with_duplicate_cards() {
        let duplicate = Card::new(Rank::Two, Suit::Clubs);
        let cards = std::array::from_fn(|_| duplicate);

        assert_eq!(
            Deck::new(cards).unwrap_err(),
            InvalidDeck::DuplicateCard(duplicate)
        );
    }

    #[test]
    fn at() {
        let cards = std::array::from_fn(|index| {
            Card::new(
                Rank::ALL[index % Rank::ALL.len()],
                Suit::ALL[index / Rank::ALL.len()],
            )
        });
        let sequence = Deck::new(cards).unwrap();

        assert_eq!(sequence.at(42), Card::new(Rank::Five, Suit::Spades));
    }
}
