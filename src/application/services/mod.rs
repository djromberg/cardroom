mod create_tournament;
mod join_tournament;
mod observe_table;

pub use create_tournament::*;
pub use join_tournament::*;
pub use observe_table::*;


pub trait ProvideServices: CreateTournament + JoinTournament + ObserveTable {}
impl<T: CreateTournament + JoinTournament + ObserveTable> ProvideServices for T {}
