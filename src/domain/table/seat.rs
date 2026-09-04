use super::hand::ParticipantInfo;
use super::player::{Player, PlayerInfo};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SeatNo(pub u8);

#[derive(Debug, Clone)]
pub(super) struct Seat {
    seat_no: SeatNo,
    player: Option<Player>,
}

impl Seat {
    pub(super) fn new(seat_no: SeatNo) -> Self {
        Self {
            seat_no,
            player: None,
        }
    }

    pub(super) fn seat_no(&self) -> SeatNo {
        self.seat_no
    }

    pub(super) fn player(&self) -> Option<&Player> {
        self.player.as_ref()
    }

    pub(super) fn is_free(&self) -> bool {
        self.player.is_none()
    }

    pub(super) fn take(&mut self, player_info: PlayerInfo) {
        assert!(self.is_free());
        self.player = Some(Player::new(player_info));
    }

    pub(super) fn participate_in_hand(&mut self) -> Option<ParticipantInfo> {
        self.player.as_mut().map(|player| ParticipantInfo {
            seat_no: self.seat_no,
            stack: player.take_stack(),
        })
    }

    pub(super) fn return_from_hand(&mut self, participant: ParticipantInfo) {
        assert_eq!(self.seat_no, participant.seat_no);
        self.player
            .as_mut()
            .expect("cannot return a stack to a free seat")
            .return_stack(participant.stack);
    }
}
