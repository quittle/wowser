use wowser_macros::DisplayFromDebug;

use crate::parse::*;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq)]
pub(super) enum JsonToken {
    Document,
    String,
    Number,
    Boolean,
    Null,
    OpenCurlyBrace,
    CloseCurlyBrace,
    OpenSquareBrace,
    CloseSquareBrace,
    Colon,
    Comma,
    Terminator,
}

const POST_VALUES: &[JsonToken] = &[
    JsonToken::Comma,
    JsonToken::CloseCurlyBrace,
    JsonToken::CloseSquareBrace,
    JsonToken::Terminator,
];

const VALUES: &[JsonToken] = &[
    JsonToken::String,
    JsonToken::Number,
    JsonToken::Boolean,
    JsonToken::Null,
    JsonToken::OpenCurlyBrace,
    JsonToken::OpenSquareBrace,
];

impl Token for JsonToken {
    fn regex(&self) -> &str {
        match self {
            Self::Document => "",
            Self::String => r#"\s*("(?:[^"\\]|\\.)*")\s*"#,
            Self::Number => r"\s*(-?(0([1-9]|(\.\d+)?$)|[1-9])\d*(\.\d+)?)\s*",
            Self::Boolean => r"\s*(true|false)\s*",
            Self::Null => r"\s*(null)\s*",
            Self::OpenCurlyBrace => r"\s*(\{)\s*",
            Self::CloseCurlyBrace => r"\s*(\})\s*",
            Self::OpenSquareBrace => r"\s*(\[)\s*",
            Self::CloseSquareBrace => r"\s*(\])\s*",
            Self::Colon => r"\s*(:)\s*",
            Self::Comma => r"\s*(,)\s*",
            Self::Terminator => r"\s*$",
        }
    }

    fn next_tokens(&self) -> Vec<JsonToken> {
        match self {
            Self::Document => Vec::from(VALUES),
            Self::String => ([&[Self::Colon], POST_VALUES]).concat(),
            Self::Number => Vec::from(POST_VALUES),
            Self::Boolean => Vec::from(POST_VALUES),
            Self::Null => Vec::from(POST_VALUES),
            Self::OpenCurlyBrace => vec![Self::String, Self::CloseCurlyBrace],
            Self::CloseCurlyBrace => Vec::from(POST_VALUES),
            Self::OpenSquareBrace => ([&[Self::CloseSquareBrace], VALUES]).concat(),
            Self::CloseSquareBrace => Vec::from(POST_VALUES),
            Self::Colon => Vec::from(VALUES),
            Self::Comma => Vec::from(VALUES),
            Self::Terminator => vec![],
        }
    }

    fn is_terminator(&self) -> bool {
        matches!(self, Self::Terminator)
    }
}
