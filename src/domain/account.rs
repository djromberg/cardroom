use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccountId(Uuid);

impl AccountId {
    pub const fn new(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<Uuid> for AccountId {
    fn from(value: Uuid) -> Self {
        Self::new(value)
    }
}

impl From<AccountId> for Uuid {
    fn from(account_id: AccountId) -> Self {
        account_id.0
    }
}
