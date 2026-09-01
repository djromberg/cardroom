use super::shared::{Chips, PlayerId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    player_id: PlayerId,
    nickname: String,
    stack: StackLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerInfo {
    player_id: PlayerId,
    nickname: String,
    stack: Chips,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StackLocation {
    AtTable(Chips),
    InHand,
}

impl Player {
    pub fn new(info: PlayerInfo) -> Self {
        Self {
            player_id: info.player_id,
            nickname: info.nickname,
            stack: StackLocation::AtTable(info.stack),
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn nickname(&self) -> &str {
        &self.nickname
    }

    pub fn stack(&self) -> Option<Chips> {
        match self.stack {
            StackLocation::AtTable(stack) => Some(stack),
            StackLocation::InHand => None,
        }
    }

    pub fn take_stack(&mut self) -> Chips {
        let previous = std::mem::replace(&mut self.stack, StackLocation::InHand);
        match previous {
            StackLocation::AtTable(stack) => stack,
            StackLocation::InHand => panic!("player's stack is already in a hand"),
        }
    }

    pub fn return_stack(&mut self, stack: Chips) {
        assert_eq!(self.stack, StackLocation::InHand);
        self.stack = StackLocation::AtTable(stack);
    }
}

impl PlayerInfo {
    pub fn new(player_id: PlayerId, nickname: String, stack: Chips) -> Self {
        Self {
            player_id,
            nickname,
            stack,
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn nickname(&self) -> &str {
        &self.nickname
    }

    pub fn stack(&self) -> Chips {
        self.stack
    }
}
