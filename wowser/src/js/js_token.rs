use super::super::parse::*;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq)]
pub enum JsToken {
    Document,
    VarKeyword,
    FunctionKeyword,
    ReturnKeyword,
    TrueKeyword,
    FalseKeyword,
    VariableName,
    Number,
    String,
    Undefined,
    OperatorAdd,
    OperatorMultiply,
    OperatorEquals,
    OpenParen,
    CloseParen,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Comma,
    Semicolon,
    Terminator,
}

const STATEMENT_START: &[JsToken] = &[
    JsToken::VarKeyword,
    JsToken::FunctionKeyword,
    JsToken::ReturnKeyword,
    JsToken::Semicolon,
];

const EXPRESSION_START: &[JsToken] = &[
    JsToken::TrueKeyword,
    JsToken::FalseKeyword,
    JsToken::VariableName,
    JsToken::Number,
    JsToken::String,
    JsToken::Undefined,
    JsToken::OperatorAdd,
];

const POST_EXPRESSION: &[JsToken] = &[
    JsToken::OperatorAdd,
    JsToken::OperatorMultiply,
    JsToken::CloseParen,
    JsToken::Comma,
    JsToken::Semicolon,
    JsToken::Terminator,
];

impl Token for JsToken {
    fn regex(&self) -> &str {
        match self {
            Self::Document => "",
            Self::VarKeyword => r"\s*(var\s)\s*",
            Self::FunctionKeyword => r"\s*(function\s)\s*",
            Self::ReturnKeyword => r"\s*(return\s)\s*",
            Self::TrueKeyword => r"\s*(true)\s*",
            Self::FalseKeyword => r"\s*(false)\s*",
            Self::VariableName => {
                r"\s*((?!((var|function|return|undefined|true|false)[^a-zA-Z_]))[a-zA-Z_][\w\d]*)\s*"
            }
            Self::Number => r"\s*(-?\d[\d_]*(\.\d[\d_]*)?)\s*",
            Self::String => r#"\s*(("[^"]*")|('[^']*'))\s*"#,
            Self::Undefined => r"\s*(undefined)\s*",
            Self::OperatorAdd => r"\s*(\+)\s*",
            Self::OperatorMultiply => r"\s*(\*)\s*",
            Self::OperatorEquals => r"\s*(=)\s*",
            Self::OpenParen => r"\s*(\()\s*",
            Self::CloseParen => r"\s*(\))\s*",
            Self::OpenCurlyBrace => r"\s*({)\s*",
            Self::CloseCurlyBrace => r"\s*(})\s*",
            Self::Comma => r"\s*(,)\s*",
            Self::Semicolon => r"\s*(;)\s*",
            Self::Terminator => r"\s*$",
        }
    }

    #[rustfmt::skip]
    fn next_tokens(&self) -> Vec<Self> {
        match self {
            Self::Document => [
                &[
                    Self::Terminator,
                ],
                EXPRESSION_START,
                STATEMENT_START,
            ].concat(),
            Self::VarKeyword => vec![
                Self::VariableName,
            ],
            Self::FunctionKeyword => vec![
                Self::VariableName,
            ],
            Self::ReturnKeyword => Vec::from(EXPRESSION_START),
            Self::TrueKeyword => Vec::from(POST_EXPRESSION),
            Self::FalseKeyword => Vec::from(POST_EXPRESSION),
            Self::VariableName => [
                &[
                    Self::OperatorEquals,
                    Self::OpenParen,
                ],
                POST_EXPRESSION,
            ].concat(),
            Self::Number => Vec::from(POST_EXPRESSION),
            Self::String => Vec::from(POST_EXPRESSION),
            Self::Undefined => Vec::from(POST_EXPRESSION),
            Self::OperatorAdd => vec![
                Self::TrueKeyword,
                Self::FalseKeyword,
                Self::VariableName,
                Self::Number,
                Self::String,
                Self::OperatorAdd,
                Self::Undefined,
            ],
            Self::OperatorMultiply => Vec::from(EXPRESSION_START),
            Self::OperatorEquals => Vec::from(EXPRESSION_START),
            Self::OpenParen => [
                &[
                    Self::VariableName,
                    Self::CloseParen,
                ],
                EXPRESSION_START,
            ].concat(),
            Self::CloseParen => vec![
                Self::CloseParen,
                Self::OpenCurlyBrace,
                Self::OperatorMultiply,
                Self::OperatorAdd,
                Self::Comma,
                Self::Terminator,
            ],
            Self::OpenCurlyBrace => [
                EXPRESSION_START,
                STATEMENT_START,
            ].concat(),
            Self::CloseCurlyBrace => [
                &[
                    Self::Terminator,
                ],
                EXPRESSION_START,
                STATEMENT_START,
            ].concat(),
            Self::Comma => [
                &[
                    Self::CloseParen,
                ],
                EXPRESSION_START,
            ].concat(),
            Self::Semicolon => [
                &[
                    Self::CloseCurlyBrace,
                    Self::Terminator,
                ],
                EXPRESSION_START,
                STATEMENT_START,
            ].concat(),
            Self::Terminator => vec![],
        }
    }

    fn is_terminator(&self) -> bool {
        matches!(self, Self::Terminator)
    }
}
