#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card(u8);

impl Card {
    pub fn new(value: u8) -> Self {
        assert!(value < 52, "invalid card value: {value}");
        Self(value)
    }

    pub fn order(&self) -> usize {
        self.0 as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "invalid card value: 52")]
    fn new_with_invalid_value() {
        Card::new(52);
    }

    #[test]
    fn new() {
        let card = Card::new(51);
        assert_eq!(card.order(), 51);
    }
}
