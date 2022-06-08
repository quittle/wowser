use fancy_regex::Regex;
use std::fmt;

pub trait Token: fmt::Debug + fmt::Display + Copy + PartialEq {
    fn built_regex(&self) -> Regex {
        let regex = self.regex();
        Regex::new(format!("^{regex}").as_str())
            .unwrap_or_else(|_| panic!("invalid regex: {regex}"))
    }

    fn regex(&self) -> &str;
    fn next_tokens(&self) -> Vec<Self>;
    fn is_terminator(&self) -> bool;
}
