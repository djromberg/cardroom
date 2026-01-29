use crate::application::AuthError;
use crate::application::AuthInfo;

use crate::domain::SaveTournament;
use crate::domain::SaveTournamentError;
use crate::domain::Tournament;
use crate::domain::TournamentSpecification;
use crate::domain::TournamentSpecificationError;

use thiserror::Error;
use uuid::Uuid;


#[derive(Debug, Error)]
pub enum CreateTournamentError {
    #[error(transparent)]
    TournamentSpecificationError(#[from] TournamentSpecificationError),
    #[error(transparent)]
    SaveTournamentError(#[from] SaveTournamentError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
}


#[derive(Debug)]
pub struct CreateTournamentRequest {
    pub table_count: u8,
    pub table_seat_count: u8,
}


#[derive(Debug)]
pub struct CreateTournamentResponse {
    pub tournament_id: Uuid,
}


pub trait CreateTournament {
    fn create_tournament(&mut self, request: CreateTournamentRequest, auth_info: &AuthInfo) -> Result<CreateTournamentResponse, CreateTournamentError>;
}


pub(in crate::application) fn create_tournament<Repository: SaveTournament>(request: CreateTournamentRequest, auth_info: &AuthInfo, repository: &mut Repository) -> Result<CreateTournamentResponse, CreateTournamentError> {
    auth_info.ensure_authenticated()?;
    let tournament_spec = TournamentSpecification::new(request.table_count, request.table_seat_count)?;
    let tournament = Tournament::new(&tournament_spec);
    let response = CreateTournamentResponse { tournament_id: tournament.id() };
    repository.save_tournament(tournament)?;
    Ok(response)
}


#[cfg(test)]
mod tests {
    use crate::application::auth::AuthRole;

    use super::*;

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
    fn create_tournament_without_being_authenticated() {
        let mut repository = DummyRepository::new_with_successful_save();
        let request = CreateTournamentRequest { table_count: 1, table_seat_count: 5 };
        let auth_info = AuthInfo::Unauthenticated;
        let result = create_tournament(request, &auth_info, &mut repository);
        assert!(matches!(result, Err(CreateTournamentError::AuthError(AuthError::AuthenticationRequired))));
        assert_eq!(repository.tournament(), None);
    }

    #[test]
    fn create_tournament_with_invalid_parameters() {
        let mut repository = DummyRepository::new_with_successful_save();
        let request = CreateTournamentRequest { table_count: 0, table_seat_count: 5 };
        let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: AuthRole::Member };
        let result = create_tournament(request, &auth_info, &mut repository);
        assert!(matches!(result, Err(CreateTournamentError::TournamentSpecificationError(_))));
        assert_eq!(repository.tournament(), None);
    }

    #[test]
    fn create_tournament_with_repository_error() {
        let mut repository = DummyRepository::new_with_error_on_save(SaveTournamentError::DatabaseWritingError);
        let request = CreateTournamentRequest { table_count: 50, table_seat_count: 5 };
        let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: AuthRole::Member };
        let result = create_tournament(request, &auth_info, &mut repository);
        assert!(matches!(result, Err(CreateTournamentError::SaveTournamentError(SaveTournamentError::DatabaseWritingError))));
    }

    #[test]
    fn create_tournament_without_any_error() {
        let mut repository = DummyRepository::new_with_successful_save();
        let request = CreateTournamentRequest { table_count: 50, table_seat_count: 5 };
        let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: AuthRole::Member };
        let result = create_tournament(request, &auth_info, &mut repository);
        let tournament = repository.tournament().unwrap();
        assert!(result.is_ok_and(|response| response.tournament_id == tournament.id()));
    }
}
