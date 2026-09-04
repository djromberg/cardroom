use std::ops::{Add, Sub};

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
