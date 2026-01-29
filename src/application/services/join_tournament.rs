use crate::application::AuthError;
use crate::application::AuthInfo;

use crate::domain::LoadTournament;
use crate::domain::LoadTournamentError;
use crate::domain::Nickname;
use crate::domain::NicknameError;
use crate::domain::SaveTournament;
use crate::domain::SaveTournamentError;
use crate::domain::TournamentError;

use thiserror::Error;
use uuid::Uuid;


#[derive(Debug, Error)]
pub enum JoinTournamentError {
    #[error(transparent)]
    NicknameError(#[from] NicknameError),
    #[error(transparent)]
    LoadTournamentError(#[from] LoadTournamentError),
    #[error(transparent)]
    SaveTournamentError(#[from] SaveTournamentError),
    #[error(transparent)]
    TournamentError(#[from] TournamentError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
}


#[derive(Debug)]
pub struct JoinTournamentRequest {
    pub tournament_id: Uuid,
    pub nickname: String,
}


#[derive(Debug)]
pub struct JoinTournamentResponse {
    pub table_id: Uuid,
}


pub trait JoinTournament {
    fn join_tournament(&mut self, request: JoinTournamentRequest, auth_info: &AuthInfo) -> Result<JoinTournamentResponse, JoinTournamentError>;
}


pub(in crate::application) fn join_tournament<Repository: LoadTournament + SaveTournament>(request: JoinTournamentRequest, auth_info: &AuthInfo, repository: &mut Repository) -> Result<JoinTournamentResponse, JoinTournamentError> {
    let account_id = auth_info.ensure_authenticated()?;
    let nickname = Nickname::new(request.nickname)?;
    let mut tournament = repository.load_tournament(request.tournament_id)?;
    let table_id = tournament.join(account_id, nickname)?;
    repository.save_tournament(tournament)?;
    let response = JoinTournamentResponse { table_id };
    Ok(response)
}


#[cfg(test)]
mod tests {
    use crate::application::AuthRole;
    use crate::domain::Tournament;
    use crate::domain::TournamentSpecification;

    use super::*;

    struct DummyRepository {
        load_error: Option<LoadTournamentError>,
        save_error: Option<SaveTournamentError>,
        tournament: Option<Tournament>,
    }

    impl DummyRepository {
        fn new_with_error_on_load(load_error: LoadTournamentError) -> Self {
            Self { load_error: Some(load_error), save_error: None, tournament: None }
        }

        fn new_with_error_on_save(save_error: SaveTournamentError, tournament: Tournament) -> Self {
            Self { load_error: None, save_error: Some(save_error), tournament: Some(tournament) }
        }

        fn new_without_tournament() -> Self {
            Self { load_error: None, save_error: None, tournament: None }
        }

        fn new_with_tournament(tournament: Tournament) -> Self {
            Self { load_error: None, save_error: None, tournament: Some(tournament) }
        }

        fn tournament(&self) -> Option<&Tournament> {
            return self.tournament.as_ref()
        }
    }

    impl LoadTournament for DummyRepository {
        fn load_tournament(&self, tournament_id: Uuid) -> Result<Tournament, LoadTournamentError> {
            if let Some(error) = self.load_error {
                Err(error)
            } else if let Some(tournament) = &self.tournament {
                if tournament.id() == tournament_id {
                    Ok(tournament.clone())
                } else {
                    Err(LoadTournamentError::TournamentNotFound)
                }
            } else {
                Err(LoadTournamentError::TournamentNotFound)
            }
        }
    }

    impl SaveTournament for DummyRepository {
        fn save_tournament(&mut self, tournament: Tournament) -> Result<(), SaveTournamentError> {
            if let Some(error) = self.save_error {
                Err(error)
            } else {
                self.tournament = Some(tournament);
                Ok(())
            }
        }
    }


    #[test]
    fn join_tournament_without_being_authenticated() {
        let mut repository = DummyRepository::new_without_tournament();
        let request = JoinTournamentRequest { tournament_id: Uuid::new_v4(), nickname: "Daniel".into() };
        let auth_info = AuthInfo::Unauthenticated;
        let result = join_tournament(request, &auth_info, &mut repository);
        assert!(matches!(result, Err(JoinTournamentError::AuthError(AuthError::AuthenticationRequired))));
        assert_eq!(repository.tournament(), None);
    }

    #[test]
    fn join_tournament_with_invalid_parameters() {
        let mut repository = DummyRepository::new_without_tournament();
        let request = JoinTournamentRequest { tournament_id: Uuid::new_v4(), nickname: "".into() };
        let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: AuthRole::Member };
        let result = join_tournament(request, &auth_info, &mut repository);
        assert!(matches!(result, Err(JoinTournamentError::NicknameError(_))));
        assert_eq!(repository.tournament(), None);
    }

    #[test]
    fn join_tournament_with_repository_error_on_load() {
        let mut repository = DummyRepository::new_with_error_on_load(LoadTournamentError::DatabaseReadingError);
        let request = JoinTournamentRequest { tournament_id: Uuid::new_v4(), nickname: "Daniel".into() };
        let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: AuthRole::Member };
        let result = join_tournament(request, &auth_info, &mut repository);
        assert!(matches!(result, Err(JoinTournamentError::LoadTournamentError(LoadTournamentError::DatabaseReadingError))));
    }

    #[test]
    fn join_tournament_with_repository_error_on_save() {
        let spec = TournamentSpecification::new(1, 2).unwrap();
        let tournament = Tournament::new(&spec);
        let tournament_id = tournament.id();
        let mut repository = DummyRepository::new_with_error_on_save(SaveTournamentError::DatabaseWritingError, tournament);
        let request = JoinTournamentRequest { tournament_id, nickname: "Daniel".into() };
        let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: AuthRole::Member };
        let result = join_tournament(request, &auth_info, &mut repository);
        assert!(matches!(result, Err(JoinTournamentError::SaveTournamentError(SaveTournamentError::DatabaseWritingError))));
    }

    #[test]
    fn join_tournament_with_tournament_error() {
        let spec = TournamentSpecification::new(1, 2).unwrap();
        let mut tournament = Tournament::new(&spec);
        _ = tournament.join(Uuid::new_v4(), Nickname::new("James").unwrap());
        _ = tournament.join(Uuid::new_v4(), Nickname::new("Patricia").unwrap());
        let tournament_id = tournament.id();
        let mut repository = DummyRepository::new_with_tournament(tournament);
        let request = JoinTournamentRequest { tournament_id, nickname: "Daniel".into() };
        let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: AuthRole::Member };
        let result = join_tournament(request, &auth_info, &mut repository);
        assert!(matches!(result, Err(JoinTournamentError::TournamentError(_))));
    }

    #[test]
    fn join_tournament_without_any_error() {
        let spec = TournamentSpecification::new(1, 2).unwrap();
        let tournament = Tournament::new(&spec);
        let tournament_id = tournament.id();
        let table_ids = tournament.table_ids();
        let mut repository = DummyRepository::new_with_tournament(tournament);
        let request = JoinTournamentRequest { tournament_id, nickname: "Daniel".into() };
        let auth_info = AuthInfo::Authenticated { account_id: Uuid::new_v4(), role: AuthRole::Member };
        let result = join_tournament(request, &auth_info, &mut repository);
        assert!(result.is_ok_and(|response| table_ids.contains(&response.table_id)));
    }
}
