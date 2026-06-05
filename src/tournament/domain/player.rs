use super::error::DomainError;

use uuid::Uuid;


#[derive(Debug, Clone, PartialEq)]
pub struct PlayerSpec {
    player_id: PlayerId,
    nickname: String,
}

impl PlayerSpec {
    pub fn new(player_id: PlayerId, nickname: String) -> Result<Self, DomainError> {
        if !nickname.len() >= 1 && nickname.len() <= 12 {
            Ok(Self { player_id, nickname })
        } else {
            Err(DomainError::InvalidNickname)
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn nickname(&self) -> String {
        self.nickname.clone()
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerId(Uuid);

impl PlayerId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}


#[derive(Debug, Clone)]
pub struct TournamentPlayer {
    spec: PlayerSpec,
    stack: u32,
}


impl TournamentPlayer {
    pub fn new(spec: &PlayerSpec, stack: u32) -> Self {
        assert!(stack > 0);
        Self { spec: spec.clone(), stack }
    }
}
