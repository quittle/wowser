use super::super::parse::*;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq)]
pub enum JsToken {
    Document,
    Number,
    OperatorAdd,
    OperatorMultiply,
    Semicolon,
    Terminator,
}

impl Token for JsToken {
    fn regex(&self) -> &str {
        match self {
            Self::Document => "",
            Self::Number => r"\s*([\d_]+)\s*",
            Self::OperatorAdd => r"\s*(\+)\s*",
            Self::OperatorMultiply => r"\s*(\*)\s*",
            Self::Semicolon => r"\s*(;)\s*",
            Self::Terminator => r"\s*$",
        }
    }

    #[rustfmt::skip]
    fn next_tokens(&self) -> Vec<Self> {
        match self {
            Self::Document => vec![
                Self::Number,
                Self::OperatorAdd,
                Self::Semicolon,
                Self::Terminator,
            ],
            Self::Number => vec![
                Self::OperatorAdd,
                Self::OperatorMultiply,
                Self::Semicolon,
                Self::Terminator,
            ],
            Self::OperatorAdd => vec![
                Self::Number,
                Self::OperatorAdd
            ],
            Self::OperatorMultiply => vec![
                Self::Number,
                Self::OperatorAdd
            ],
            Self::Semicolon => vec![
                Self::Number,
                Self::OperatorAdd,
                Self::Semicolon,
                Self::Terminator,
            ],
            Self::Terminator => vec![],
        }
    }

    fn is_terminator(&self) -> bool {
        matches!(self, Self::Terminator)
    }
}
