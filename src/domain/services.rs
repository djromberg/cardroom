use crate::domain::PublishTournamentEvents;
use crate::domain::SaveTournament;
use crate::domain::SaveTournamentError;
use crate::domain::Tournament;


pub fn save_tournament_and_publish_events<Repository: SaveTournament, Publisher: PublishTournamentEvents>(
    mut tournament: Tournament,
    repository: &mut Repository,
    publisher: &Publisher,
) -> Result<(), SaveTournamentError> {
    let table_events = tournament.collect_events();
    repository.save_tournament(tournament)?;
    publisher.publish_tournament_events(table_events);
    Ok(())
}
