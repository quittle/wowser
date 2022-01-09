use super::{super::parse::*, js_token::JsToken};
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq, Eq, Hash)]
pub enum JsRule {
    Document,
    Statements,
    Statement,
    VarDeclaration,
    VarKeyword,
    VariableName,
    VariableAssignment,
    Expression,
    ExpressionAdd,
    ExpressionSubAdd,
    ExpressionMultiply,
    ExpressionSubMultiply,
    OperatorAdd,
    OperatorMultiply,
    OperatorEquals,
    LiteralValue,
    Number,
    String,
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
                RuleType::Sequence(vec![Self::Statements, Self::VarDeclaration, Self::Terminator]),
                RuleType::Sequence(vec![Self::Statements, Self::Expression, Self::Terminator]),
                RuleType::Sequence(vec![Self::Statements, Self::VariableAssignment, Self::Terminator]),
                RuleType::Sequence(vec![Self::Statements, Self::Terminator]),
                RuleType::Rule(Self::Terminator),
            ],
            Self::Statements => vec![
                RuleType::RepeatableRule(Self::Statement),
            ],
            Self::Statement => vec![
                RuleType::Sequence(vec![Self::VarDeclaration, Self::Semicolon]),
                RuleType::Sequence(vec![Self::Expression, Self::Semicolon]),
                RuleType::Sequence(vec![Self::VariableAssignment, Self::Semicolon]),
                RuleType::Sequence(vec![Self::Semicolon]),
            ],
            Self::VarDeclaration => vec![
                RuleType::Sequence(vec![Self::VarKeyword, Self::VariableName, Self::OperatorEquals, Self::Expression]),
                RuleType::Sequence(vec![Self::VarKeyword, Self::VariableName]),
            ],
            Self::VarKeyword => vec![
                RuleType::Token(JsToken::VarKeyword),
            ],
            Self::VariableName => vec![
                RuleType::Token(JsToken::VariableName),
            ],
            Self::VariableAssignment => vec![
                RuleType::Sequence(vec![Self::VariableName, Self::OperatorEquals, Self::Expression]),
            ],
            Self::Expression => vec![
                RuleType::Rule(Self::ExpressionAdd),
                RuleType::Rule(Self::ExpressionMultiply),
                RuleType::Rule(Self::VariableName),
                RuleType::Rule(Self::LiteralValue),
            ],
            Self::ExpressionSubAdd => vec![
                RuleType::Rule(Self::ExpressionMultiply),
                RuleType::Rule(Self::VariableName),
                RuleType::Rule(Self::ExpressionAdd),
                RuleType::Rule(Self::LiteralValue),
            ],
            Self::ExpressionAdd => vec![
                RuleType::Sequence(vec![Self::ExpressionMultiply, Self::OperatorAdd, Self::ExpressionSubAdd]),
                RuleType::Sequence(vec![Self::VariableName, Self::OperatorAdd, Self::ExpressionSubAdd]),
                RuleType::Sequence(vec![Self::LiteralValue, Self::OperatorAdd, Self::ExpressionSubAdd]),
                RuleType::Sequence(vec![Self::OperatorAdd, Self::LiteralValue]),
            ],
            Self::ExpressionMultiply => vec![
                RuleType::Sequence(vec![Self::VariableName, Self::OperatorMultiply, Self::ExpressionSubMultiply]),
                RuleType::Sequence(vec![Self::LiteralValue, Self::OperatorMultiply, Self::ExpressionSubMultiply]),
            ],
            Self::ExpressionSubMultiply => vec![
                RuleType::Rule(Self::ExpressionMultiply),
                RuleType::Rule(Self::VariableName),
                RuleType::Rule(Self::LiteralValue),
            ],
            Self::LiteralValue => vec![
                RuleType::Rule(Self::Number),
                RuleType::Rule(Self::String),
            ],
            Self::OperatorAdd => vec![
                RuleType::Token(JsToken::OperatorAdd),
            ],
            Self::OperatorMultiply => vec![
                RuleType::Token(JsToken::OperatorMultiply),
            ],
            Self::OperatorEquals => vec![
                RuleType::Token(JsToken::OperatorEquals),
            ],
            Self::Number => vec![
                RuleType::Token(JsToken::Number),
            ],
            Self::String => vec![
                RuleType::Token(JsToken::String),
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
