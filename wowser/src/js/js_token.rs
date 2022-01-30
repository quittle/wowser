use super::super::parse::*;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq)]
pub enum JsToken {
    Document,
    IfKeyword,
    VarKeyword,
    FunctionKeyword,
    ReturnKeyword,
    TrueKeyword,
    FalseKeyword,
    NullKeyword,
    NaNKeyword,
    VariableName,
    Number,
    String,
    Undefined,
    OperatorAdd,
    OperatorMultiply,
    OperatorEquals,
    OperatorEquality,
    OpenParen,
    CloseParen,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Comma,
    Colon,
    Semicolon,
    Terminator,
}

const STATEMENT_START: &[JsToken] = &[
    JsToken::IfKeyword,
    JsToken::VarKeyword,
    JsToken::FunctionKeyword,
    JsToken::ReturnKeyword,
    JsToken::Semicolon,
];

const EXPRESSION_START: &[JsToken] = &[
    JsToken::TrueKeyword,
    JsToken::FalseKeyword,
    JsToken::NullKeyword,
    JsToken::Number,
    JsToken::String,
    JsToken::Undefined,
    JsToken::NaNKeyword,
    JsToken::OperatorAdd,
    JsToken::VariableName,
    JsToken::OpenCurlyBrace,
];

const POST_EXPRESSION: &[JsToken] = &[
    JsToken::OperatorAdd,
    JsToken::OperatorMultiply,
    JsToken::OperatorEquality,
    JsToken::CloseParen,
    JsToken::CloseCurlyBrace,
    JsToken::Comma,
    JsToken::Semicolon,
    JsToken::Terminator,
];

impl Token for JsToken {
    fn regex(&self) -> &str {
        match self {
            Self::Document => "",
            Self::IfKeyword => r"\s*(if)\s*",
            Self::VarKeyword => r"\s*(var\s)\s*",
            Self::FunctionKeyword => r"\s*(function\s)\s*",
            Self::ReturnKeyword => r"\s*(return\s)\s*",
            Self::TrueKeyword => r"\s*(true)\s*",
            Self::FalseKeyword => r"\s*(false)\s*",
            Self::NullKeyword => r"\s*(null)\s*",
            Self::VariableName => {
                r"\s*((?!((var|function|return|undefined|true|false|null|if|NaN)[^a-zA-Z_$]))[a-zA-Z_][\w\d]*)\s*"
            }
            Self::Number => r"\s*(-?\d[\d_]*(\.\d[\d_]*)?)\s*",
            Self::String => r#"\s*(("[^"]*")|('[^']*'))\s*"#,
            Self::Undefined => r"\s*(undefined)\s*",
            Self::NaNKeyword => r"\s*(NaN)\s*",
            Self::OperatorAdd => r"\s*(\+)\s*",
            Self::OperatorMultiply => r"\s*(\*)\s*",
            Self::OperatorEquals => r"\s*(=)\s*",
            Self::OperatorEquality => r"\s*(!==|!=|===|==)\s*",
            Self::OpenParen => r"\s*(\()\s*",
            Self::CloseParen => r"\s*(\))\s*",
            Self::OpenCurlyBrace => r"\s*({)\s*",
            Self::CloseCurlyBrace => r"\s*(})\s*",
            Self::Comma => r"\s*(,)\s*",
            Self::Colon => r"\s*(:)\s*",
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
            Self::IfKeyword => vec![
                Self::OpenParen,
            ],
            Self::VarKeyword => vec![
                Self::VariableName,
            ],
            Self::FunctionKeyword => vec![
                Self::VariableName,
            ],
            Self::ReturnKeyword => Vec::from(EXPRESSION_START),
            Self::TrueKeyword => Vec::from(POST_EXPRESSION),
            Self::FalseKeyword => Vec::from(POST_EXPRESSION),
            Self::NullKeyword => Vec::from(POST_EXPRESSION),
            Self::VariableName => [
                &[
                    Self::OperatorEquals,
                    Self::OpenParen,
                ],
                POST_EXPRESSION,
            ].concat(),
            Self::Number => Vec::from(POST_EXPRESSION),
            Self::String => [
                &[
                    Self::Colon,
                ],
                POST_EXPRESSION,
            ].concat(),
            Self::Undefined => Vec::from(POST_EXPRESSION),
            Self::NaNKeyword => Vec::from(POST_EXPRESSION),
            Self::OperatorAdd => Vec::from(EXPRESSION_START),
            Self::OperatorMultiply => Vec::from(EXPRESSION_START),
            Self::OperatorEquals => Vec::from(EXPRESSION_START),
            Self::OperatorEquality => Vec::from(EXPRESSION_START),
            Self::OpenParen => [
                &[
                    Self::CloseParen,
                ],
                EXPRESSION_START,
            ].concat(),
            Self::CloseParen => [
                &[
                    Self::CloseParen,
                    Self::OpenCurlyBrace,
                    Self::OperatorMultiply,
                    Self::OperatorAdd,
                    Self::Comma,
                Self::Terminator,
                ],
                EXPRESSION_START,
                STATEMENT_START,
            ].concat(),
            Self::OpenCurlyBrace => [
                &[
                    Self::CloseCurlyBrace,
                ],
                EXPRESSION_START,
                STATEMENT_START,
            ].concat(),
            Self::CloseCurlyBrace => [
                &[
                    Self::CloseCurlyBrace,
                    Self::Terminator,
                ],
                EXPRESSION_START,
                STATEMENT_START,
                POST_EXPRESSION,
            ].concat(),
            Self::Comma => [
                &[
                    Self::CloseParen,
                    Self::CloseCurlyBrace,
                ],
                EXPRESSION_START,
            ].concat(),
            Self::Colon => Vec::from(EXPRESSION_START),
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
