#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card(u8);


impl Card {
    pub fn new(value: u8) -> Self {
        assert!(value < 52, "invalid card value: {value}");
        Self(value)
    }

    pub fn value(&self) -> u8 {
        self.0
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
        assert_eq!(card.value(), 51);
    }
}
