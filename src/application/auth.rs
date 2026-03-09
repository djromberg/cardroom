use thiserror::Error;
use uuid::Uuid;


#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Permission denied: {required:?} role required")]
    PermissionDenied { required: AuthRole },
    #[error("Invalid account id")]
    InvalidAccountId,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthRole {
    Observer,
    Participant,
    Organizer,
    Unknown,
}

impl From<String> for AuthRole {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "observer" => AuthRole::Observer,
            "participant" => AuthRole::Participant,
            "organizer" => AuthRole::Organizer,
            _ => AuthRole::Unknown,
        }
    }
}

impl std::fmt::Display for AuthRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthRole::Observer => f.write_str("observer"),
            AuthRole::Participant => f.write_str("participant"),
            AuthRole::Organizer => f.write_str("organizer"),
            AuthRole::Unknown => f.write_str("unknown role"),
        }
    }
}


#[derive(Debug)]
pub struct AuthInfo {
    account_id: Uuid,
    roles: Vec<AuthRole>,
}

impl AuthInfo {
    pub fn new(account_id: Uuid, roles: Vec<AuthRole>) -> Self {
        Self { account_id, roles }
    }

    pub fn expect_role(&self, role: AuthRole) -> Result<Uuid, AuthError> {
        if self.roles.contains(&role) {
            Ok(self.account_id)
        } else {
            Err(AuthError::PermissionDenied { required: role })
        }
    }
}


#[cfg(test)]
mod tests {
}
