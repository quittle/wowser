use super::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum RuleType<R: Rule<T>, T: Token> {
    /// Single, unrepeatable rule
    Rule(R),
    /// Rule can repeat 0+ times in a row, greedily consuming
    RepeatableRule(R),
    /// A sequence of rules that need to be matched
    Sequence(Vec<R>),
    /// Single Token
    Token(T),
}

pub trait Rule<T: Token>:
    fmt::Debug + fmt::Display + PartialEq + std::marker::Sized + Copy + Eq + std::hash::Hash
{
    /// One of these children must match for the rule to match
    fn children(&self) -> Vec<RuleType<Self, T>>;

    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}
