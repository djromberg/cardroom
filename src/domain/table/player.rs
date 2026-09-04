use super::super::account::AccountId;
use super::super::chips::Chips;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Player {
    account_id: AccountId,
    nickname: String,
    stack: StackLocation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerInfo {
    account_id: AccountId,
    nickname: String,
    stack: Chips,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StackLocation {
    AtTable(Chips),
    InHand,
}

impl Player {
    pub(super) fn new(info: PlayerInfo) -> Self {
        Self {
            account_id: info.account_id,
            nickname: info.nickname,
            stack: StackLocation::AtTable(info.stack),
        }
    }

    pub(super) fn account_id(&self) -> AccountId {
        self.account_id
    }

    pub(super) fn nickname(&self) -> &str {
        &self.nickname
    }

    pub(super) fn stack(&self) -> Option<Chips> {
        match self.stack {
            StackLocation::AtTable(stack) => Some(stack),
            StackLocation::InHand => None,
        }
    }

    pub(super) fn take_stack(&mut self) -> Chips {
        let previous = std::mem::replace(&mut self.stack, StackLocation::InHand);
        match previous {
            StackLocation::AtTable(stack) => stack,
            StackLocation::InHand => panic!("player's stack is already in a hand"),
        }
    }

    pub(super) fn return_stack(&mut self, stack: Chips) {
        assert_eq!(self.stack, StackLocation::InHand);
        self.stack = StackLocation::AtTable(stack);
    }
}

impl PlayerInfo {
    pub fn new(account_id: AccountId, nickname: String, stack: Chips) -> Self {
        Self {
            account_id,
            nickname,
            stack,
        }
    }

    pub fn account_id(&self) -> AccountId {
        self.account_id
    }

    pub fn nickname(&self) -> &str {
        &self.nickname
    }

    pub fn stack(&self) -> Chips {
        self.stack
    }
}
