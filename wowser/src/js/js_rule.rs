use super::{super::parse::*, js_token::JsToken};
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Debug, DisplayFromDebug, PartialEq)]
pub enum JsRule {
    Document,
    Statements,
    Statement,
    Expression,
    ExpressionAdd,
    OperationAdd,
    Number,
    Semicolon,
    Terminator,
}

impl JsRule {}

impl Rule for JsRule {
    #[rustfmt::skip]
    fn children(&self) -> Vec<RuleType<Self>> {
        match self {
            Self::Document => vec![
                RuleType::Sequence(vec![Self::Statements.b(), Self::Expression.b(), Self::Terminator.b()]),
                RuleType::Sequence(vec![Self::Statements.b(), Self::Terminator.b()]),
                RuleType::Rule(Self::Terminator.b()),
            ],
            Self::Statements => vec![
                RuleType::RepeatableRule(Self::Statement.b()),
            ],
            Self::Statement => vec![
                RuleType::Sequence(vec![Self::Expression.b(), Self::Semicolon.b()]),
                RuleType::Sequence(vec![Self::Semicolon.b()]),
            ],
            Self::Expression => vec![
                RuleType::Rule(Self::ExpressionAdd.b()),
                RuleType::Rule(Self::Number.b()),
            ],
            Self::ExpressionAdd => vec![
                RuleType::Sequence(vec![Self::Number.b(), Self::OperationAdd.b(), Self::Expression.b()]),
                RuleType::Sequence(vec![Self::OperationAdd.b(), Self::Number.b()]),
            ],
            Self::OperationAdd => vec![
                RuleType::Token(JsToken::OperatorAdd.b()),
            ],
            Self::Number => vec![
                RuleType::Token(JsToken::Number.b()),
            ],
            Self::Semicolon => vec![
                RuleType::Token(JsToken::Semicolon.b()),
            ],
            Self::Terminator => vec![
                RuleType::Token(JsToken::Terminator.b())
            ],
        }
    }
}
