use super::super::parse::*;
use std::any::Any;

#[derive(Clone, Debug)]
pub enum MathRule {
    Document, // Root isn't referenced
    DocumentBody,
    Statement,
    Expression,
    BinaryExpression,
    BinaryOperator,
    Semicolon,
    Number,
    Terminator,
}

trait T {
    fn foo(&self) -> &'static str;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

struct A {}

impl T for A {
    fn foo(&self) -> &'static str {
        "a"
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl MathRule {}
 
impl Rule for MathRule {
    fn children(&self) -> Vec<RuleType<MathRule>> {
        match self {
            MathRule::Document => vec!(RuleType::Sequence(vec!(MathRule::DocumentBody.b(), MathRule::Terminator.b()))),
            MathRule::DocumentBody => vec!(RuleType::RepeatableRule(MathRule::Statement.b())),
            MathRule::Statement => vec!(RuleType::Sequence(vec!(MathRule::Expression.b(), MathRule::Semicolon.b()))),
            MathRule::Expression => vec!(RuleType::Rule(MathRule::BinaryExpression.b()), RuleType::Rule(MathRule::Number.b())),
            MathRule::BinaryExpression => vec!(RuleType::Sequence(vec!(MathRule::Number.b(), MathRule::BinaryOperator.b(), MathRule::Expression.b()))),
            MathRule::BinaryOperator => vec!(RuleType::Token(MathToken::Plus.b())),
            MathRule::Semicolon => vec!(RuleType::Token(MathToken::Semicolon.b())),
            MathRule::Number => vec!(RuleType::Token(MathToken::Number.b())),
            MathRule::Terminator => vec!(RuleType::Token(MathToken::Terminator.b())),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MathToken {
    Document,
    Number,
    Plus,
    Whitespace,
    Semicolon,
    Terminator
}

impl Token for MathToken {
    fn regex(&self) -> &str {
        match self {
            MathToken::Document => "",
            MathToken::Number => r"\s*-?\d+(\.\d+)?",
            MathToken::Plus => r"\s*\+",
            MathToken::Whitespace => r"\s+",
            MathToken::Semicolon => r";",
            MathToken::Terminator => r"^$",
        }
    }

    fn next_tokens(&self) -> Vec<Box<dyn Token>> {
        match self {
            MathToken::Document => vec!(Box::new(MathToken::Whitespace), Box::new(MathToken::Number), Box::new(MathToken::Terminator)),
            MathToken::Number => vec!(Box::new(MathToken::Plus), Box::new(MathToken::Semicolon), Box::new(MathToken::Whitespace)),
            MathToken::Plus => vec!(Box::new(MathToken::Number)),
            MathToken::Whitespace => vec!(Box::new(MathToken::Whitespace), Box::new(MathToken::Terminator)),
            MathToken::Semicolon => vec!(Box::new(MathToken::Number), Box::new(MathToken::Whitespace), Box::new(MathToken::Semicolon), Box::new(MathToken::Terminator)),
            MathToken::Terminator => vec!(),
        }
    }

    fn is_terminator(&self) -> bool {
        match self {
            MathToken::Terminator => true,
            _ => false
        }
    }
}