use thiserror::Error;

use std::fmt::Display;


#[derive(Debug, Error)]
pub enum NicknameError {
    #[error("Nicknames must have at least one character")]
    NicknameTooShort,
    #[error("Nicknames must not have more than 12 characters")]
    NicknameTooLong,
}


#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Nickname {
    value: String,
}

impl Nickname {
    pub fn new(value: impl Into<String>) -> Result<Self, NicknameError> {
        let value = value.into();
        if value.is_empty() {
            Err(NicknameError::NicknameTooShort)
        } else if value.len() > 12 {
            Err(NicknameError::NicknameTooLong)
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
        assert!(matches!(result, Err(NicknameError::NicknameTooShort)));
        let result = Nickname::new("a".repeat(13));
        assert!(matches!(result, Err(NicknameError::NicknameTooLong)));
    }

    #[test]
    fn new_with_valid_value() {
        let result = Nickname::new("denyo");
        assert!(result.is_ok());
        let nickname = result.unwrap();
        assert_eq!(format!("{}", nickname), "denyo");
    }
}
