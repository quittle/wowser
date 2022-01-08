use super::{super::parse::*, js_token::JsToken};
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq, Eq, Hash)]
pub enum JsRule {
    Document,
    Statements,
    Statement,
    Expression,
    ExpressionAdd,
    ExpressionMultiply,
    ExpressionSubMultiply,
    OperatorAdd,
    OperatorMultiply,
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
                RuleType::Rule(Self::ExpressionMultiply),
                RuleType::Rule(Self::Number),
            ],
            Self::ExpressionAdd => vec![
                RuleType::Sequence(vec![Self::ExpressionMultiply, Self::OperatorAdd, Self::Expression]),
                RuleType::Sequence(vec![Self::Number, Self::OperatorAdd, Self::Expression]),
                RuleType::Sequence(vec![Self::OperatorAdd, Self::Number]),
            ],
            Self::ExpressionMultiply => vec![
                RuleType::Sequence(vec![Self::Number, Self::OperatorMultiply, Self::ExpressionSubMultiply]),
            ],
            Self::ExpressionSubMultiply => vec![
                RuleType::Rule(Self::ExpressionMultiply),
                RuleType::Rule(Self::Number),
            ],
            Self::OperatorAdd => vec![
                RuleType::Token(JsToken::OperatorAdd),
            ],
            Self::OperatorMultiply => vec![
                RuleType::Token(JsToken::OperatorMultiply),
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
