use super::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum RuleType<R: Rule> {
    /// Single, unrepeatable rule
    Rule(R),
    /// Rule can repeat 0+ times in a row, greedily consuming
    RepeatableRule(R),
    /// A sequence of rules that need to be matched
    Sequence(Vec<R>),
    /// Single Token
    Token(R::Token),
}

pub trait Rule:
    fmt::Debug + fmt::Display + PartialEq + std::marker::Sized + Copy + Eq + std::hash::Hash
{
    type Token: Token;

    /// One of these children must match for the rule to match
    fn children(&self) -> Vec<RuleType<Self>>;

    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}
