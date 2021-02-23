use crate::common::error::MyError;
use crate::common::validation::VoValidation;
use anyhow::{bail, Result};
use regex::Regex;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserId {
    value: usize,
}

impl UserId {
    pub fn new(value: usize) -> Result<Self> {
        if Self::validate(&*value.to_string()) {
            Ok(Self { value })
        } else {
            bail!(MyError::ValidationError("Invalid Format".to_string()))
        }
    }
}

impl VoValidation for UserId {
    fn regex() -> Regex {
        Regex::new(r#"^[0-9]+$"#).unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::user::vo::userid::UserId;

    pub fn userid1() -> UserId {
        UserId::new(1).unwrap()
    }

    pub fn userid2() -> UserId {
        UserId::new(2).unwrap()
    }
}
