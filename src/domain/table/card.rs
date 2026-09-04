#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub const fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    pub const fn rank(self) -> Rank {
        self.rank
    }

    pub const fn suit(self) -> Suit {
        self.suit
    }

    pub(super) const fn index(self) -> usize {
        self.suit.index() * Rank::ALL.len() + self.rank.index()
    }

    #[cfg(test)]
    pub(super) fn from_index(index: u8) -> Self {
        assert!(index < 52, "invalid card index: {index}");
        Self {
            rank: Rank::ALL[index as usize % Rank::ALL.len()],
            suit: Suit::ALL[index as usize / Rank::ALL.len()],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub const ALL: [Self; 13] = [
        Self::Two,
        Self::Three,
        Self::Four,
        Self::Five,
        Self::Six,
        Self::Seven,
        Self::Eight,
        Self::Nine,
        Self::Ten,
        Self::Jack,
        Self::Queen,
        Self::King,
        Self::Ace,
    ];

    pub(super) const fn strength(self) -> u8 {
        self.index() as u8 + 2
    }

    const fn index(self) -> usize {
        match self {
            Self::Two => 0,
            Self::Three => 1,
            Self::Four => 2,
            Self::Five => 3,
            Self::Six => 4,
            Self::Seven => 5,
            Self::Eight => 6,
            Self::Nine => 7,
            Self::Ten => 8,
            Self::Jack => 9,
            Self::Queen => 10,
            Self::King => 11,
            Self::Ace => 12,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub const ALL: [Self; 4] = [Self::Clubs, Self::Diamonds, Self::Hearts, Self::Spades];

    const fn index(self) -> usize {
        match self {
            Self::Clubs => 0,
            Self::Diamonds => 1,
            Self::Hearts => 2,
            Self::Spades => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::table::card::*;

    #[test]
    fn card_exposes_its_rank_and_suit() {
        let card = Card::new(Rank::Ace, Suit::Spades);

        assert_eq!(card.rank(), Rank::Ace);
        assert_eq!(card.suit(), Suit::Spades);
    }

    #[test]
    fn test_index_preserves_the_original_card_order() {
        assert_eq!(Card::from_index(0), Card::new(Rank::Two, Suit::Clubs));
        assert_eq!(Card::from_index(51), Card::new(Rank::Ace, Suit::Spades));
    }
}
