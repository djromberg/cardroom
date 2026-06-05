use super::error::DomainError;
use super::player::TournamentPlayer;

use uuid::Uuid;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TableSpec {
    seat_count: u8,
}

impl TableSpec {
    pub fn new(seat_count: u8) -> Result<Self, DomainError> {
        if seat_count >= 2 && seat_count <= 10 {
            Ok(Self { seat_count })
        } else {
            Err(DomainError::InvalidTableSpecification)
        }
    }

    pub fn seat_count(&self) -> u8 {
        self.seat_count
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TableId(Uuid);

impl TableId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}


#[derive(Debug, Clone)]
pub struct TournamentTable {
    id: TableId,
    spec: TableSpec,
    players: Vec<TournamentPlayer>,
}

impl TournamentTable {
    pub fn new(spec: &TableSpec) -> Self {
        Self { id: TableId::new(), spec: spec.clone(), players: vec![] }
    }

    pub fn id(&self) -> TableId {
        self.id
    }

    pub fn seat_count(&self) -> u8 {
        self.spec.seat_count
    }

    pub fn player_count(&self) -> u8 {
        self.players.len() as u8
    }

    pub fn all_seats_taken(&self) -> bool {
        self.player_count() == self.seat_count()
    }

    pub fn seat_player(&mut self, player: TournamentPlayer) {
        assert!(!self.all_seats_taken());
        self.players.push(player);
    }
}
