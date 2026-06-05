use super::player::TournamentPlayer;

use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct TournamentTable {
    id: Uuid,
    seat_count: u8,
    players: Vec<TournamentPlayer>,
}

impl TournamentTable {
    pub fn new(seat_count: u8) -> Self {
        Self { id: Uuid::new_v4(), seat_count, players: vec![] }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn seat_count(&self) -> u8 {
        self.seat_count
    }

    pub fn player_count(&self) -> u8 {
        self.players.len() as u8
    }

    pub fn has_free_seat(&self) -> bool {
        self.players.len() < self.seat_count as usize
    }

    pub fn seat_player(&mut self, player: TournamentPlayer) {
        assert!(self.has_free_seat());
        self.players.push(player);
    }
}
