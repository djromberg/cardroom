use super::nickname::Nickname;

use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct Player {
    account_id: Uuid,
    nickname: Nickname,
    stack: u32,
}

impl Player {
    pub fn new(account_id: Uuid, nickname: Nickname, stack: u32) -> Self {
        assert!(stack > 0);
        Self { account_id, nickname, stack }
    }

    pub fn account_id(&self) -> Uuid {
        self.account_id
    }
}
