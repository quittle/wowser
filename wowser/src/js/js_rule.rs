use super::{super::parse::*, js_token::JsToken};
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq, Eq, Hash)]
pub enum JsRule {
    Document,
    Statements,
    Statement,
    VarDeclaration,
    VarKeyword,
    FunctionDeclaration,
    FunctionKeyword,
    FunctionParams,
    VariableName,
    VariableAssignment,
    Expression,
    ExpressionAdd,
    ExpressionSubAdd,
    ExpressionMultiply,
    ExpressionSubMultiply,
    FunctionInvoke,
    FunctionArguments,
    OperatorAdd,
    OperatorMultiply,
    OperatorEquals,
    OpenParen,
    CloseParen,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Comma,
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
                RuleType::Rule(Self::FunctionDeclaration),
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
            Self::FunctionDeclaration => vec![
                RuleType::Sequence(vec![
                    Self::FunctionKeyword,
                    Self::VariableName,
                    Self::OpenParen,
                    Self::FunctionParams,
                    Self::CloseParen,
                    Self::OpenCurlyBrace,
                    Self::Statements,
                    Self::CloseCurlyBrace
                ]),
            ],
            Self::FunctionKeyword => vec![
                RuleType::Token(JsToken::FunctionKeyword),
            ],
            Self::FunctionParams => vec![
                RuleType::Sequence(vec![Self::VariableName, Self::Comma, Self::FunctionParams]),
                RuleType::Sequence(vec![Self::VariableName, Self::Comma]),
                RuleType::Rule(Self::VariableName),
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
                RuleType::Rule(Self::FunctionInvoke),
                RuleType::Rule(Self::VariableName),
                RuleType::Rule(Self::LiteralValue),
            ],
            Self::ExpressionAdd => vec![
                RuleType::Sequence(vec![Self::FunctionInvoke, Self::OperatorAdd, Self::ExpressionSubAdd]),
                RuleType::Sequence(vec![Self::ExpressionMultiply, Self::OperatorAdd, Self::ExpressionSubAdd]),
                RuleType::Sequence(vec![Self::VariableName, Self::OperatorAdd, Self::ExpressionSubAdd]),
                RuleType::Sequence(vec![Self::LiteralValue, Self::OperatorAdd, Self::ExpressionSubAdd]),
                RuleType::Sequence(vec![Self::OperatorAdd, Self::LiteralValue]),
            ],
            Self::ExpressionSubAdd => vec![
                RuleType::Rule(Self::FunctionInvoke),
                RuleType::Rule(Self::ExpressionMultiply),
                RuleType::Rule(Self::VariableName),
                RuleType::Rule(Self::ExpressionAdd),
                RuleType::Rule(Self::LiteralValue),
            ],
            Self::ExpressionMultiply => vec![
                RuleType::Sequence(vec![Self::FunctionInvoke, Self::OperatorMultiply, Self::ExpressionSubMultiply]),
                RuleType::Sequence(vec![Self::VariableName, Self::OperatorMultiply, Self::ExpressionSubMultiply]),
                RuleType::Sequence(vec![Self::LiteralValue, Self::OperatorMultiply, Self::ExpressionSubMultiply]),
            ],
            Self::ExpressionSubMultiply => vec![
                RuleType::Rule(Self::FunctionInvoke),
                RuleType::Rule(Self::ExpressionMultiply),
                RuleType::Rule(Self::VariableName),
                RuleType::Rule(Self::LiteralValue),
            ],
            Self::FunctionInvoke => vec![
                RuleType::Sequence(vec![Self::VariableName, Self::OpenParen, Self::FunctionArguments, Self::CloseParen]),
            ],
            Self::FunctionArguments => vec![
                RuleType::Sequence(vec![Self::Expression, Self::Comma, Self::FunctionArguments]),
                RuleType::Sequence(vec![Self::Expression, Self::Comma]),
                RuleType::Sequence(vec![Self::Expression]),
                RuleType::Sequence(vec![]),
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
            Self::OpenParen => vec![
                RuleType::Token(JsToken::OpenParen),
            ],
            Self::CloseParen => vec![
                RuleType::Token(JsToken::CloseParen),
            ],
            Self::OpenCurlyBrace => vec![
                RuleType::Token(JsToken::OpenCurlyBrace),
            ],
            Self::CloseCurlyBrace => vec![
                RuleType::Token(JsToken::CloseCurlyBrace),
            ],
            Self::Comma => vec![
                RuleType::Token(JsToken::Comma),
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
