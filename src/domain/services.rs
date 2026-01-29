use crate::domain::PublishTableEvents;
use crate::domain::SaveTournament;
use crate::domain::SaveTournamentError;
use crate::domain::Tournament;


pub fn save_tournament_and_publish_events<Repository: SaveTournament, Publisher: PublishTableEvents>(
    mut tournament: Tournament,
    repository: &mut Repository,
    publisher: &Publisher,
) -> Result<(), SaveTournamentError> {
    let table_events = tournament.collect_events();
    repository.save_tournament(tournament)?;
    publisher.publish_table_events(table_events);
    Ok(())
}
