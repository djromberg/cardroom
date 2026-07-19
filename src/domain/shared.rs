use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TableId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SeatNo(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chips(pub u64);


impl Sub for Chips {
    type Output = Chips;

    fn sub(self, rhs: Self) -> Self::Output {
        Chips(self.0 - rhs.0)
    }
}

impl Add for Chips {
    type Output = Chips;

    fn add(self, rhs: Self) -> Self::Output {
        Chips(self.0 + rhs.0)
    }
}
