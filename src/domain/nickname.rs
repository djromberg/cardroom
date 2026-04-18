use super::DomainError;

use std::fmt::Display;


#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Nickname {
    value: String,
}

impl Nickname {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.is_empty() || value.len() > 12 {
            Err(DomainError::InvalidNickname)
        } else {
            Ok(Self { value })
        }
    }
}

impl Display for Nickname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_invalid_values() {
        let result = Nickname::new("");
        assert!(matches!(result, Err(DomainError::InvalidNickname)));
        let result = Nickname::new("a".repeat(13));
        assert!(matches!(result, Err(DomainError::InvalidNickname)));
    }

    #[test]
    fn new_with_valid_value() {
        let result = Nickname::new("denyo");
        assert!(result.is_ok());
        let nickname = result.unwrap();
        assert_eq!(format!("{}", nickname), "denyo");
    }
}
