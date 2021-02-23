use crate::common::error::MyError;
use anyhow::{bail, Result};
use regex::Regex;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Name {
    value: String,
}

impl Name {
    pub fn new(value: &str) -> Result<Self> {
        let regex = Regex::new(r#"^[0-9a-zA-Z]+$"#).unwrap();
        if regex.is_match(value) {
            Ok(Self {
                value: value.to_string(),
            })
        } else {
            bail!(MyError::ValidationError("Invalid Format".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::user::vo::name::Name;

    pub fn name1() -> Name {
        Name::new("name1").unwrap()
    }

    pub fn name2() -> Name {
        Name::new("name2").unwrap()
    }

    #[test]
    fn name_equals() {
        //値によって同一と見なされること
        let name1 = self::name1();
        let name2 = self::name1();
        assert_eq!(name1, name2);

        //値によって同一と見なされないこと
        let name3 = self::name2();
        assert_ne!(name1, name3);
    }

    #[test]
    fn name_validation() {
        //生成できること
        assert!(Name::new("InvalidName01").is_ok());

        //バリデーションエラーとなること
        assert!(Name::new("InvalidName01!!!").is_err());
    }
}
