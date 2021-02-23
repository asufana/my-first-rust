use regex::Regex;

pub trait VoValidation {
    //バリデーション定義
    fn regex() -> Regex;

    //バリデーション実施
    fn validate(value: &str) -> bool {
        Self::regex().is_match(value)
    }
}
