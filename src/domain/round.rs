use std::collections::HashMap;

use crate::domain::shared::{Chips, SeatNo};


#[derive(Debug, Clone)]
pub struct Round {
    bet_amounts: HashMap<SeatNo, Chips>,
}

impl Round {
    pub fn new() -> Self {
        Self { bet_amounts: HashMap::new() }
    }

    pub fn add_chips(&mut self, seat_no: SeatNo, amount: Chips) {
        let current_amount = self.bet_amounts[&seat_no];
        self.bet_amounts.insert(seat_no, current_amount + amount);
    }
}
