use super::super::parse::*;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq)]
pub enum JsToken {
    Document,
    IfKeyword,
    ElseKeyword,
    VarKeyword,
    FunctionKeyword,
    ThisKeyword,
    ReturnKeyword,
    ThrowKeyword,
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
    Dot,
    Comma,
    Colon,
    Semicolon,
    QuestionMark,
    InlineComment,
    MultilineComment,
    Terminator,
}

const STATEMENT_START: &[JsToken] = &[
    JsToken::IfKeyword,
    JsToken::VarKeyword,
    JsToken::FunctionKeyword,
    JsToken::ReturnKeyword,
    JsToken::ThrowKeyword,
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
    JsToken::ElseKeyword,
    JsToken::Colon,
    JsToken::QuestionMark,
    JsToken::Terminator,
];

impl Token for JsToken {
    fn regex(&self) -> &str {
        match self {
            Self::Document => "",
            Self::IfKeyword => r"\s*(if)\s*",
            Self::ElseKeyword => r"\s*(else)\s*",
            Self::VarKeyword => r"\s*(var\s)\s*",
            Self::FunctionKeyword => r"\s*(function\s)\s*",
            Self::ThisKeyword => r"\s*(this)\s*",
            Self::ReturnKeyword => r"\s*(return\s)\s*",
            Self::ThrowKeyword => r"\s*(throw\s)\s*",
            Self::TrueKeyword => r"\s*(true)\s*",
            Self::FalseKeyword => r"\s*(false)\s*",
            Self::NullKeyword => r"\s*(null)\s*",
            Self::VariableName => {
                r"\s*((?!((var|function|throw|return|undefined|true|false|null|if|NaN)[^a-zA-Z_$]))[a-zA-Z_][\w\d]*)\s*"
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
            Self::Dot => r"\s*(\.)\s*",
            Self::Comma => r"\s*(,)\s*",
            Self::Colon => r"\s*(:)\s*",
            Self::Semicolon => r"\s*(;)\s*",
            Self::QuestionMark => r"\s*(\?)\s*",
            Self::InlineComment => r"\s*(//.*)",
            Self::MultilineComment => r"\s*(/\*(.|\n)*?\*/)\s*",
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
            Self::ElseKeyword => [
                &[
                    Self::OpenParen,
                    Self::OperatorAdd,
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
            Self::ThisKeyword => [
                &[
                    Self::OperatorEquals,
                    Self::OpenParen,
                    Self::Dot,
                ],
                POST_EXPRESSION,
            ].concat(),
            Self::ReturnKeyword => Vec::from(EXPRESSION_START),
            Self::ThrowKeyword => Vec::from(EXPRESSION_START),
            Self::TrueKeyword => Vec::from(POST_EXPRESSION),
            Self::FalseKeyword => Vec::from(POST_EXPRESSION),
            Self::NullKeyword => Vec::from(POST_EXPRESSION),
            Self::VariableName => [
                &[
                    Self::OperatorEquals,
                    Self::OpenParen,
                    Self::Dot,
                ],
                POST_EXPRESSION,
            ].concat(),
            Self::Number => Vec::from(POST_EXPRESSION),
            Self::String => [
                &[
                    Self::Dot,
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
                EXPRESSION_START,
                STATEMENT_START,
                POST_EXPRESSION,
            ].concat(),
            Self::Dot => vec![
                Self::VariableName,
            ],
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
                    Self::ElseKeyword,
                    Self::Terminator,
                ],
                EXPRESSION_START,
                STATEMENT_START,
            ].concat(),
            Self::QuestionMark => Vec::from(EXPRESSION_START),
            Self::InlineComment => vec![],
            Self::MultilineComment => vec![],
            Self::Terminator => vec![],
        }
    }

    fn get_comment_tokens() -> &'static [Self] {
        &[Self::InlineComment, Self::MultilineComment]
    }

    fn is_terminator(&self) -> bool {
        matches!(self, Self::Terminator)
    }
}
