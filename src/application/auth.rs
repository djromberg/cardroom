use thiserror::Error;
use uuid::Uuid;


#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Authentication required: Please authenticate yourself")]
    AuthenticationRequired,
    #[error("Permission denied: {required:?} rights required, but only {found:?} rights given")]
    PermissionDenied { required: AuthRole, found: AuthRole },
}


#[derive(Debug, Clone, Copy)]
pub enum AuthRole {
    Member,
    Moderator,
    Administrator,
}


#[derive(Debug)]
pub enum AuthInfo {
    Unauthenticated,
    Authenticated {
        account_id: Uuid,
        role: AuthRole,
    }
}

impl AuthInfo {
    pub fn ensure_authenticated(&self) -> Result<Uuid, AuthError> {
        match self {
            Self::Unauthenticated => Err(AuthError::AuthenticationRequired),
            Self::Authenticated { account_id, .. } => Ok(*account_id),
        }
    }

    pub fn ensure_moderator(&self) -> Result<Uuid, AuthError> {
        match self {
            Self::Unauthenticated => Err(AuthError::AuthenticationRequired),
            Self::Authenticated { account_id, role } => {
                match role {
                    AuthRole::Member => Err(AuthError::PermissionDenied { required: AuthRole::Moderator, found: *role }),
                    _ => Ok(*account_id),
                }
            },
        }
    }
    
    pub fn ensure_administrator(&self) -> Result<Uuid, AuthError> {
        match self {
            Self::Unauthenticated => Err(AuthError::AuthenticationRequired),
            Self::Authenticated { account_id, role } => {
                match role {
                    AuthRole::Administrator => Ok(*account_id),
                    _ => Err(AuthError::PermissionDenied { required: AuthRole::Administrator, found: *role }),
                }
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_authenticated() {
        let auth_info = AuthInfo::Unauthenticated;
        assert!(matches!(auth_info.ensure_authenticated(), Err(AuthError::AuthenticationRequired)));
        let account_id = Uuid::new_v4();
        let auth_info = AuthInfo::Authenticated { account_id, role: AuthRole::Member };
        assert!(matches!(auth_info.ensure_authenticated(), Ok(id) if id == account_id));
        let auth_info = AuthInfo::Authenticated { account_id, role: AuthRole::Moderator };
        assert!(matches!(auth_info.ensure_authenticated(), Ok(id) if id == account_id));
        let auth_info = AuthInfo::Authenticated { account_id, role: AuthRole::Administrator };
        assert!(matches!(auth_info.ensure_authenticated(), Ok(id) if id == account_id));
    }

    #[test]
    fn ensure_moderator() {
        let auth_info = AuthInfo::Unauthenticated;
        assert!(matches!(auth_info.ensure_moderator(), Err(AuthError::AuthenticationRequired)));
        let account_id = Uuid::new_v4();
        let auth_info = AuthInfo::Authenticated { account_id, role: AuthRole::Member };
        assert!(matches!(auth_info.ensure_moderator(), Err(AuthError::PermissionDenied { required: AuthRole::Moderator, found: AuthRole::Member })));
        let auth_info = AuthInfo::Authenticated { account_id, role: AuthRole::Moderator };
        assert!(matches!(auth_info.ensure_moderator(), Ok(id) if id == account_id));
        let auth_info = AuthInfo::Authenticated { account_id, role: AuthRole::Administrator };
        assert!(matches!(auth_info.ensure_moderator(), Ok(id) if id == account_id));
    }

    #[test]
    fn ensure_administrator() {
        let auth_info = AuthInfo::Unauthenticated;
        assert!(matches!(auth_info.ensure_administrator(), Err(AuthError::AuthenticationRequired)));
        let account_id = Uuid::new_v4();
        let auth_info = AuthInfo::Authenticated { account_id, role: AuthRole::Member };
        assert!(matches!(auth_info.ensure_administrator(), Err(AuthError::PermissionDenied { required: AuthRole::Administrator, found: AuthRole::Member })));
        let auth_info = AuthInfo::Authenticated { account_id, role: AuthRole::Moderator };
        assert!(matches!(auth_info.ensure_administrator(), Err(AuthError::PermissionDenied { required: AuthRole::Administrator, found: AuthRole::Moderator })));
        let auth_info = AuthInfo::Authenticated { account_id, role: AuthRole::Administrator };
        assert!(matches!(auth_info.ensure_administrator(), Ok(id) if id == account_id));
    }
}
