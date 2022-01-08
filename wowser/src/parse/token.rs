use fancy_regex::Regex;
use std::fmt;

pub trait Token: fmt::Debug + fmt::Display + Copy + PartialEq {
    fn built_regex(&self) -> Regex {
        Regex::new(format!("^{}", self.regex()).as_str()).expect("valid regex")
    }

    fn regex(&self) -> &str;
    fn next_tokens(&self) -> Vec<Self>;
    fn is_terminator(&self) -> bool;
}
