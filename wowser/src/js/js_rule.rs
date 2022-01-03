use super::{super::parse::*, js_token::JsToken};
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq, Eq, Hash)]
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
    type Token = JsToken;

    #[rustfmt::skip]
    fn children(&self) -> Vec<RuleType<Self>> {
        match self {
            Self::Document => vec![
                RuleType::Sequence(vec![Self::Statements, Self::Expression, Self::Terminator]),
                RuleType::Sequence(vec![Self::Statements, Self::Terminator]),
                RuleType::Rule(Self::Terminator),
            ],
            Self::Statements => vec![
                RuleType::RepeatableRule(Self::Statement),
            ],
            Self::Statement => vec![
                RuleType::Sequence(vec![Self::Expression, Self::Semicolon]),
                RuleType::Sequence(vec![Self::Semicolon]),
            ],
            Self::Expression => vec![
                RuleType::Rule(Self::ExpressionAdd),
                RuleType::Rule(Self::Number),
            ],
            Self::ExpressionAdd => vec![
                RuleType::Sequence(vec![Self::Number, Self::OperationAdd, Self::Expression]),
                RuleType::Sequence(vec![Self::OperationAdd, Self::Number]),
            ],
            Self::OperationAdd => vec![
                RuleType::Token(JsToken::OperatorAdd),
            ],
            Self::Number => vec![
                RuleType::Token(JsToken::Number),
            ],
            Self::Semicolon => vec![
                RuleType::Token(JsToken::Semicolon),
            ],
            Self::Terminator => vec![
                RuleType::Token(JsToken::Terminator)
            ],
        }
    }
}
