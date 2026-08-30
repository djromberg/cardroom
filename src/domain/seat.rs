use super::shared::SeatNo;
use super::table::PlayerInfo;

#[derive(Debug, Clone)]
pub struct Seat {
    seat_no: SeatNo,
    player_info: Option<PlayerInfo>,
}

impl Seat {
    pub fn new(seat_no: SeatNo) -> Self {
        Self {
            seat_no,
            player_info: None,
        }
    }

    pub fn seat_no(&self) -> SeatNo {
        self.seat_no
    }

    pub fn player_info(&self) -> Option<&PlayerInfo> {
        self.player_info.as_ref()
    }

    pub fn is_free(&self) -> bool {
        self.player_info.is_none()
    }

    pub fn take(&mut self, player_info: PlayerInfo) {
        assert!(self.is_free());
        self.player_info = Some(player_info);
    }

    pub fn take_stack(&mut self) -> super::shared::Chips {
        self.player_info
            .as_mut()
            .expect("cannot take a stack from a free seat")
            .take_stack()
    }

    pub fn return_stack(&mut self, stack: super::shared::Chips) {
        self.player_info
            .as_mut()
            .expect("cannot return a stack to a free seat")
            .return_stack(stack);
    }
}
