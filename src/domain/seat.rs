use super::player::Player;
use super::player::PlayerData;


#[derive(Debug, Clone)]
pub struct Seat {
    position: u8,
    player: Option<Player>,
}

impl Seat {
    pub fn new(position: u8) -> Self {
        Self { position, player: None }
    }

    pub fn position(&self) -> u8 {
        self.position
    }

    pub fn is_available(&self) -> bool {
        self.player.is_none()
    }

    pub fn is_taken(&self) -> bool {
        self.player.is_some()
    }

    pub fn take(&mut self, data: PlayerData) {
        let player = Player::new(self.position, data);
        self.player = Some(player);
    }

    pub fn player(&mut self) -> &mut Player {
        self.player.as_mut().unwrap()
    }
}
