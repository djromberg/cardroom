use crate::domain::{
    chips::Chips,
    table::{SeatNo, hand::participant::Participant},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pot {
    pub amount: Chips,
    pub eligible_seats: Vec<SeatNo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PotAward {
    pub seat_no: SeatNo,
    pub amount: Chips,
}

pub(super) fn calculate(participants: &[Participant]) -> Vec<Pot> {
    let mut levels: Vec<_> = participants
        .iter()
        .map(|participant| participant.committed.0)
        .filter(|amount| *amount > 0)
        .collect();
    levels.sort_unstable();
    levels.dedup();

    let mut previous = 0;
    let mut pots: Vec<Pot> = vec![];
    for level in levels {
        let contributors = participants
            .iter()
            .filter(|participant| participant.committed.0 >= level)
            .count() as u64;
        let amount = Chips((level - previous) * contributors);
        previous = level;
        let eligible_seats = participants
            .iter()
            .filter(|participant| !participant.folded && participant.committed.0 >= level)
            .map(|participant| participant.seat_no)
            .collect::<Vec<_>>();
        if eligible_seats.is_empty() {
            continue;
        }
        if let Some(pot) = pots
            .last_mut()
            .filter(|pot| pot.eligible_seats == eligible_seats)
        {
            pot.amount = pot.amount + amount;
        } else {
            pots.push(Pot {
                amount,
                eligible_seats,
            });
        }
    }
    pots
}
