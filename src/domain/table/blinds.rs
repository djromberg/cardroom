use crate::domain::chips::Chips;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Blinds {
    pub small: Chips,
    pub big: Chips,
}
