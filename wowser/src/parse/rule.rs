use super::token::Token;

use std::any::Any;
use std::fmt;

pub trait RuleClone {
    fn clone_box(&self) -> Box<Self>;

    fn b(self) -> Box<Self>;

    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: 'static + Rule + Clone> RuleClone for T {
    fn clone_box(&self) -> Box<Self> {
        Box::new(self.clone())
    }

    fn b(self) -> Box<Self> {
        Box::new(self)
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

#[derive(fmt::Debug)]
pub enum RuleType<T: Rule> {
    /// Single, unrepeatable rule
    Rule(Box<T>),
    /// Rule can repeat 0+ times in a row, greedily consuming
    RepeatableRule(Box<T>),
    /// A sequence of rules that need to be matched
    Sequence(Vec<Box<T>>),
    /// Single Token
    Token(Box<dyn Token>),
}

pub trait Rule: RuleClone + fmt::Debug + fmt::Display
where
    Self: std::marker::Sized,
{
    /// One of these children must match for the rule to match
    fn children(&self) -> Vec<RuleType<Self>>;

    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}
