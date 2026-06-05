use thiserror::Error;


#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid table specification")]
    InvalidTableSpecification,
    #[error("Invalid table specification")]
    InvalidTournamentSpecification,
    #[error("Invalid nickname")]
    InvalidNickname,
    #[error("Tournament already started")]
    TournamentAlreadyStarted,
}
