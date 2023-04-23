use fancy_regex::Regex;
use std::fmt;

pub trait Token: fmt::Debug + fmt::Display + Copy + PartialEq + 'static {
    fn built_regex(&self) -> Regex {
        let regex = self.regex();
        Regex::new(format!("^{regex}").as_str())
            .unwrap_or_else(|_| panic!("invalid regex: {regex}"))
    }

    fn regex(&self) -> &str;
    fn next_tokens(&self) -> Vec<Self>;
    fn is_terminator(&self) -> bool;

    /// Indicates that the token is a comment and unneeded to pass on to the parser.
    /// Comments can be found anywhere in the source code and preempt any token matches.
    fn get_comment_tokens() -> &'static [Self] {
        &[]
    }

    /// Indicates that the token represents a comment.
    fn is_comment(&self) -> bool {
        Self::get_comment_tokens().contains(self)
    }
}
