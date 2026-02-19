mod create_tournament;
mod find_tournaments;
mod join_tournament;
mod observe_table;

pub use create_tournament::*;
pub use find_tournaments::*;
pub use join_tournament::*;
pub use observe_table::*;


pub trait ProvideServices: FindTournaments + CreateTournament + JoinTournament + ObserveTable {}
impl<T: FindTournaments + CreateTournament + JoinTournament + ObserveTable> ProvideServices for T {}
