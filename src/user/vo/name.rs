
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Name {
    value: String
}

impl Name {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::user::vo::name::Name;

    pub fn name1() -> Name {
        Name::new("name1")
    }

    #[test]
    fn name_equals() {
        let name1 = self::name1();
        let name2 = self::name1();

        //値による同一性が得られること
        assert_eq!(name1, name2);
    }
}


