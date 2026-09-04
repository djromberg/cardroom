use super::super::card::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluatedHand {
    pub category: HandCategory,
    pub best_five: [Card; 5],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandCategory {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RankedHand {
    pub(super) score: u64,
    pub(super) evaluated: EvaluatedHand,
}

pub(super) fn evaluate(cards: &[Card]) -> RankedHand {
    assert!(cards.len() >= 5, "at least five cards are required");
    let mut best = 0;
    let mut best_five = [cards[0]; 5];
    for a in 0..cards.len() - 4 {
        for b in a + 1..cards.len() - 3 {
            for c in b + 1..cards.len() - 2 {
                for d in c + 1..cards.len() - 1 {
                    for e in d + 1..cards.len() {
                        let candidate = [cards[a], cards[b], cards[c], cards[d], cards[e]];
                        let score = score_five(candidate);
                        if score > best {
                            best = score;
                            best_five = candidate;
                        }
                    }
                }
            }
        }
    }
    RankedHand {
        score: best,
        evaluated: EvaluatedHand {
            category: HandCategory::from_score(best),
            best_five,
        },
    }
}

impl HandCategory {
    fn from_score(mut score: u64) -> Self {
        for _ in 0..5 {
            score /= 15;
        }
        match score {
            0 => Self::HighCard,
            1 => Self::OnePair,
            2 => Self::TwoPair,
            3 => Self::ThreeOfAKind,
            4 => Self::Straight,
            5 => Self::Flush,
            6 => Self::FullHouse,
            7 => Self::FourOfAKind,
            8 => Self::StraightFlush,
            _ => unreachable!("invalid hand category"),
        }
    }
}

fn score_five(cards: [Card; 5]) -> u64 {
    let mut ranks: Vec<u8> = cards.iter().map(|card| card.rank().strength()).collect();
    ranks.sort_unstable_by(|left, right| right.cmp(left));
    let flush = cards.iter().all(|card| card.suit() == cards[0].suit());
    let mut unique = ranks.clone();
    unique.dedup();
    let straight = if unique == [14, 5, 4, 3, 2] {
        Some(5)
    } else if unique.len() == 5 && unique[0] - unique[4] == 4 {
        Some(unique[0])
    } else {
        None
    };
    let mut groups: Vec<_> = unique
        .iter()
        .map(|rank| {
            (
                ranks.iter().filter(|candidate| *candidate == rank).count(),
                *rank,
            )
        })
        .collect();
    groups.sort_unstable_by(|left, right| right.cmp(left));
    let (category, values): (u8, Vec<u8>) = if flush && let Some(high) = straight {
        (8, vec![high])
    } else if groups[0].0 == 4 {
        (7, vec![groups[0].1, groups[1].1])
    } else if groups[0].0 == 3 && groups[1].0 == 2 {
        (6, vec![groups[0].1, groups[1].1])
    } else if flush {
        (5, ranks)
    } else if let Some(high) = straight {
        (4, vec![high])
    } else if groups[0].0 == 3 {
        (3, groups.iter().map(|group| group.1).collect())
    } else if groups[0].0 == 2 && groups[1].0 == 2 {
        (2, groups.iter().map(|group| group.1).collect())
    } else if groups[0].0 == 2 {
        (1, groups.iter().map(|group| group.1).collect())
    } else {
        (0, ranks)
    };
    let mut score = category as u64;
    for index in 0..5 {
        score = score * 15 + values.get(index).copied().unwrap_or(0) as u64;
    }
    score
}
