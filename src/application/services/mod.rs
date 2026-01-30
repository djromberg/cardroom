mod create_tournament;
mod join_tournament;
mod observe_table;

pub use create_tournament::*;
pub use join_tournament::*;


pub trait ProvideServices: CreateTournament + JoinTournament {}
impl<T: CreateTournament + JoinTournament> ProvideServices for T {}
