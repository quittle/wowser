use super::token::Token;
use std::fmt;

pub trait RuleClone {
    fn clone_box(&self) -> Box<dyn Rule>;

    fn b(self) -> Box<dyn Rule>;
}

impl<T: 'static + Rule + Clone> RuleClone for T {
    fn clone_box(&self) -> Box<dyn Rule> {
        Box::new(self.clone())
    }

    fn b(self) -> Box<dyn Rule> {
        Box::new(self)
    }
}

pub enum RuleType {
    /// Single, unrepeatable rule
    Rule(Box<dyn Rule>),
    /// Rule can repeat 0+ times in a row, greedily consuming
    RepeatableRule(Box<dyn Rule>),
    /// A sequence of rules that need to be matched
    Sequence(Vec<Box<dyn Rule>>),
    /// Single Token
    Token(Box<dyn Token>),
}

pub trait Rule: RuleClone + fmt::Debug {
    /// One of these children must match for the rule to match
    fn children(&self) -> Vec<RuleType>;
}