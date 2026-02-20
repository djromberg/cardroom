use crate::domain::PublishTournamentMessages;
use crate::domain::SaveTournament;
use crate::domain::SaveTournamentError;
use crate::domain::Tournament;


pub fn save_tournament_and_publish_messages<Repository: SaveTournament, Publisher: PublishTournamentMessages>(
    mut tournament: Tournament,
    repository: &mut Repository,
    publisher: &Publisher,
) -> Result<(), SaveTournamentError> {
    let tournament_messages = tournament.collect_messages();
    repository.save_tournament(tournament)?;
    publisher.publish_tournament_messages(tournament_messages);
    Ok(())
}
