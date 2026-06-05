use crate::application::RepositoryError;
use crate::domain::Tournament;
use crate::domain::TournamentEvent;
use crate::domain::TournamentId;

use async_trait::async_trait;


#[async_trait]
pub trait TournamentRepository: Clone + Sync + Send + 'static {
    async fn load_tournament(&self, id: TournamentId) -> Result<Tournament, RepositoryError>;
    async fn save_tournament(&self, tournament: Tournament) -> Result<Vec<TournamentEvent>, RepositoryError>;
}
