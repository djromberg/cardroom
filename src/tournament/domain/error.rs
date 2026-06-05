use thiserror::Error;


#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid nickname")]
    InvalidNickname,
    #[error("Tournament already started")]
    TournamentAlreadyStarted,
}
