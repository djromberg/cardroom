use super::super::super::chips::Chips;
use super::super::SeatNo;
use super::super::card::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::domain::table) struct ParticipantInfo {
    pub(in crate::domain::table) seat_no: SeatNo,
    pub(in crate::domain::table) stack: Chips,
}

#[derive(Debug, Clone)]
pub(super) struct Participant {
    pub(super) seat_no: SeatNo,
    pub(super) stack: Chips,
    pub(super) current_bet: Chips,
    pub(super) committed: Chips,
    pub(super) folded: bool,
    pub(super) cards: Vec<Card>,
    pub(super) pending: bool,
}

impl Participant {
    pub(super) fn new(info: &ParticipantInfo) -> Self {
        Self {
            seat_no: info.seat_no,
            stack: info.stack,
            current_bet: Chips(0),
            committed: Chips(0),
            folded: false,
            cards: vec![],
            pending: false,
        }
    }

    pub(super) fn can_act(&self) -> bool {
        !self.folded && self.stack > Chips(0)
    }
}
