use thiserror::Error;


#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid tournament specification")]
    InvalidTournamentSpecification,
    #[error("Invalid table specification")]
    InvalidTableSpecification,
    #[error("Invalid nickname")]
    InvalidNickname,
    #[error("Tournament already started")]
    TournamentAlreadyStarted,
    #[error("Player already joined")]
    PlayerAlreadyJoined,
}
