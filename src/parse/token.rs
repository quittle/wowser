use std::fmt;
use regex::Regex;

pub trait TokenClone {
    fn clone_box(&self) -> Box<dyn Token>;
    fn b(self) -> Box<dyn Token>;
}

impl<T: 'static + Token + Clone> TokenClone for T {
    fn clone_box(&self) -> Box<dyn Token> {
        Box::new(self.clone())
    }

    fn b(self) -> Box<dyn Token> {
        Box::new(self)
    }
}

pub trait Token: TokenClone + fmt::Debug {
    fn built_regex(&self) -> Regex {
        Regex::new(format!("^{}", self.regex()).as_str()).expect("valid regex")
    }

    fn eq(&self, other: &dyn Token) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }

    fn regex(&self) -> &str;
    fn next_tokens(&self) -> Vec<Box<dyn Token>>;
    fn is_terminator(&self) -> bool;
}