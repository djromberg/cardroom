use crate::application::AuthError;
use crate::application::AuthInfo;
use crate::application::AuthRole;
use crate::application::TournamentSummary;
use crate::application::get_tournament_summary;

use crate::domain::SaveTournament;
use crate::domain::SaveTournamentError;
use crate::domain::Tournament;
use crate::domain::TournamentSpecification;
use crate::domain::TournamentSpecificationError;

use serde::Deserialize;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum CreateTournamentError {
    #[error(transparent)]
    TournamentSpecificationError(#[from] TournamentSpecificationError),
    #[error(transparent)]
    SaveTournamentError(#[from] SaveTournamentError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTournamentRequest {
    pub table_count: u8,
    pub table_seat_count: u8,
}


pub type CreateTournamentResponse = TournamentSummary;


pub trait CreateTournament {
    fn create_tournament(&mut self, request: CreateTournamentRequest, auth_info: &AuthInfo) -> Result<CreateTournamentResponse, CreateTournamentError>;
}


pub(in crate::application) fn create_tournament<Repository: SaveTournament>(request: CreateTournamentRequest, auth_info: &AuthInfo, repository: &mut Repository) -> Result<CreateTournamentResponse, CreateTournamentError> {
    let account_id = auth_info.expect_role(AuthRole::Organizer)?;
    let tournament_spec = TournamentSpecification::new(request.table_count, request.table_seat_count)?;
    let tournament = Tournament::new(&tournament_spec);
    let response = get_tournament_summary(&tournament, account_id);
    repository.save_tournament(tournament)?;
    Ok(response)
}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::application::auth::AuthRole;

    use uuid::Uuid;

    struct DummyRepository {
        save_error: Option<SaveTournamentError>,
        tournament: Option<Tournament>,
    }

    impl DummyRepository {
        fn new_with_successful_save() -> Self {
            Self { save_error: None, tournament: None }
        }

        fn new_with_error_on_save(error: SaveTournamentError) -> Self {
            Self { save_error: Some(error), tournament: None }
        }

        fn tournament(&self) -> Option<&Tournament> {
            return self.tournament.as_ref()
        }
    }

    impl SaveTournament for DummyRepository {
        fn save_tournament(&mut self, tournament: Tournament) -> Result<(), SaveTournamentError> {
            if matches!(self.save_error, Some(SaveTournamentError::DatabaseWritingError)) {
                Err(SaveTournamentError::DatabaseWritingError)
            } else if matches!(self.save_error, Some(SaveTournamentError::TournamentOutdated)) {
                Err(SaveTournamentError::TournamentOutdated)
            } else {
                self.tournament = Some(tournament);
                Ok(())
            }
        }
    }


    #[test]
    fn create_tournament_with_invalid_parameters() {
        let mut repository = DummyRepository::new_with_successful_save();
        let request = CreateTournamentRequest { table_count: 0, table_seat_count: 5 };
        let auth_info = AuthInfo::new(Uuid::new_v4(), vec![AuthRole::Organizer]);
        let result = create_tournament(request, &auth_info, &mut repository);
        assert!(matches!(result, Err(CreateTournamentError::TournamentSpecificationError(_))));
        assert_eq!(repository.tournament(), None);
    }

    #[test]
    fn create_tournament_with_repository_error() {
        let mut repository = DummyRepository::new_with_error_on_save(SaveTournamentError::DatabaseWritingError);
        let request = CreateTournamentRequest { table_count: 50, table_seat_count: 5 };
        let auth_info = AuthInfo::new(Uuid::new_v4(), vec![AuthRole::Organizer]);
        let result = create_tournament(request, &auth_info, &mut repository);
        assert!(matches!(result, Err(CreateTournamentError::SaveTournamentError(SaveTournamentError::DatabaseWritingError))));
    }

    #[test]
    fn create_tournament_without_required_role() {
        let mut repository = DummyRepository::new_with_successful_save();
        let request = CreateTournamentRequest { table_count: 50, table_seat_count: 5 };
        let auth_info = AuthInfo::new(Uuid::new_v4(), vec![AuthRole::Participant]);
        let result = create_tournament(request, &auth_info, &mut repository);
        assert!(matches!(result, Err(CreateTournamentError::AuthError(AuthError::PermissionDenied { required: AuthRole::Organizer }))));
    }

    #[test]
    fn create_tournament_without_any_error() {
        let mut repository = DummyRepository::new_with_successful_save();
        let request = CreateTournamentRequest { table_count: 50, table_seat_count: 5 };
        let auth_info = AuthInfo::new(Uuid::new_v4(), vec![AuthRole::Organizer]);
        let result = create_tournament(request, &auth_info, &mut repository);
        let tournament = repository.tournament().unwrap();
        assert!(result.is_ok_and(|response| response.tournament_id == tournament.id().to_string()));
    }
}
