use super::super::parse::*;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq)]
pub enum JsToken {
    Document,
    VarKeyword,
    VariableName,
    Number,
    String,
    OperatorAdd,
    OperatorMultiply,
    OperatorEquals,
    OpenParen,
    CloseParen,
    Comma,
    Semicolon,
    Terminator,
}

const EXPRESSION_START: &[JsToken] = &[
    JsToken::VariableName,
    JsToken::Number,
    JsToken::String,
    JsToken::OperatorAdd,
];

impl Token for JsToken {
    fn regex(&self) -> &str {
        match self {
            Self::Document => "",
            Self::VarKeyword => r"\s*(var\s)\s*",
            Self::VariableName => r"\s*((?!(var))[a-zA-Z_][\w\d]*)\s*",
            Self::Number => r"\s*(-?\d[\d_]*(\.\d[\d_]*)?)\s*",
            Self::String => r#"\s*(("[^"]*")|('[^']*'))\s*"#,
            Self::OperatorAdd => r"\s*(\+)\s*",
            Self::OperatorMultiply => r"\s*(\*)\s*",
            Self::OperatorEquals => r"\s*(=)\s*",
            Self::OpenParen => r"\s*(\()\s*",
            Self::CloseParen => r"\s*(\))\s*",
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
                    Self::VarKeyword,
                    Self::Semicolon,
                    Self::Terminator,
                ],
                EXPRESSION_START
            ].concat(),
            Self::VarKeyword => vec![
                Self::VariableName,
            ],
            Self::VariableName => vec![
                Self::OperatorEquals,
                Self::OperatorAdd,
                Self::OperatorMultiply,
                Self::OpenParen,
                Self::CloseParen,
                Self::Comma,
                Self::Semicolon,
                Self::Terminator,
            ],
            Self::Number => vec![
                Self::OperatorAdd,
                Self::OperatorMultiply,
                Self::CloseParen,
                Self::Comma,
                Self::Semicolon,
                Self::Terminator,
            ],
            Self::String => vec![
                Self::OperatorAdd,
                Self::OperatorMultiply,
                Self::CloseParen,
                Self::Comma,
                Self::Semicolon,
                Self::Terminator,
            ],
            Self::OperatorAdd => vec![
                Self::VariableName,
                Self::Number,
                Self::String,
                Self::OperatorAdd
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
                Self::OperatorMultiply,
                Self::OperatorAdd,
                Self::Comma,
                Self::Terminator,
            ],
            Self::Comma => [
                &[
                    Self::CloseParen,
                ],
                EXPRESSION_START,
            ].concat(),
            Self::Semicolon => [
                &[
                    Self::VarKeyword,
                    Self::Semicolon,
                    Self::Terminator,
                ],
                EXPRESSION_START,
            ].concat(),
            Self::Terminator => vec![],
        }
    }

    fn is_terminator(&self) -> bool {
        matches!(self, Self::Terminator)
    }
}
