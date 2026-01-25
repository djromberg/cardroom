mod create_tournament;

pub use create_tournament::*;


pub trait ProvideServices: CreateTournament {}
impl<T: CreateTournament> ProvideServices for T {}
