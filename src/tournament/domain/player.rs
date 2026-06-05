use super::error::DomainError;

use uuid::Uuid;


#[derive(Debug, Clone)]
pub struct TournamentPlayer {
    id: Uuid,
    nickname: String,
    status: TournamentPlayerStatus,
}


impl TournamentPlayer {
    pub fn new(id: Uuid, nickname: String, stack: u32) -> Result<Self, DomainError> {
        assert!(stack > 0);
        if nickname.is_empty() || nickname.len() > 12 {
            Err(DomainError::InvalidNickname)
        } else {
            Ok(Self { id, nickname, status: TournamentPlayerStatus::Active(stack) })
        }
    }
}


#[derive(Debug, Clone)]
enum TournamentPlayerStatus {
    Active(u32),
    Eliminated(usize),
}
