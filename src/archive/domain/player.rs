use crate::domain::Nickname;

use uuid::Uuid;

pub type PlayerId = Uuid;


#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInfo {
    pub player_id: PlayerId,
    pub nickname: Nickname,
    pub stack: u32,
}


#[derive(Debug, Clone)]
pub struct Player {
    id: PlayerId,
    nickname: Nickname,
    stack: u32,
}

impl Player {
    pub fn new(info: &PlayerInfo) -> Self {
        assert!(info.stack > 0);
        Self {
            id: info.player_id,
            nickname: info.nickname.clone(),
            stack: info.stack,
        }
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }
}
