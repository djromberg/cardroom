use super::nickname::Nickname;

use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct Player {
    id: Uuid,
    nickname: Nickname,
    stack: u32,
}

impl Player {
    pub fn new(id: Uuid, nickname: Nickname, stack: u32) -> Self {
        assert!(stack > 0);
        // Self { id, nickname, stack }#
        Self { id, nickname, stack }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}
