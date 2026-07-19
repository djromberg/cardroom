use super::hand::Hand;
use super::shared::Chips;
use super::shared::PlayerId;
use super::shared::SeatNo;
use super::shared::TableId;


#[derive(Debug, Clone)]
pub struct Table {
    id: TableId,
    seats: Vec<Seat>,
    hand: Option<Hand>,
    events: Vec<TableEvent>,
}

impl Table {
    pub fn open(id: TableId, seat_count: u8) -> Self {
        assert!(seat_count >= 2 && seat_count <= 10);
        let mut seats = vec![];
        for i in 0..seat_count {
            seats.push(Seat::new(SeatNo(i)))
        }
        let events = vec![TableEvent::TableOpened {
            table_id: id,
            seat_count
        }];
        Self { id, seats, hand: None, events }
    }

    pub fn seat_player(&mut self, player_info: PlayerInfo) {
        let seat = self.seats.iter_mut().find(|seat| seat.is_free()).unwrap();
        seat.take(player_info.clone());
        self.events.push(TableEvent::PlayerSeated {
            table_id: self.id,
            seat_no: seat.seat_no(),
            player_info
        });
    }
}


#[derive(Debug, Clone)]
pub enum TableEvent {
    TableOpened {
        table_id: TableId,
        seat_count: u8,
    },
    PlayerSeated {
        table_id: TableId,
        seat_no: SeatNo,
        player_info: PlayerInfo,
    },
}


#[derive(Debug, Clone)]
pub struct PlayerInfo {
    player_id: PlayerId,
    nickname: String,
    stack: Chips,
}



#[derive(Debug, Clone)]
struct Seat {
    seat_no: SeatNo,
    player_info: Option<PlayerInfo>,
}

impl Seat {
    pub fn new(seat_no: SeatNo) -> Self {
        Self { seat_no, player_info: None }
    }

    pub fn seat_no(&self) -> SeatNo {
        self.seat_no
    }

    pub fn is_free(&self) -> bool {
        self.player_info.is_none()
    }

    pub fn take(&mut self, player_info: PlayerInfo) {
        assert!(self.is_free());
        self.player_info = Some(player_info);
    }
}
