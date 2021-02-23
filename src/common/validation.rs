use regex::Regex;

pub trait VoValidation {
    fn regex() -> Regex;
    fn validation(value: &str) -> bool {
        Self::regex().is_match(value)
    }
}
