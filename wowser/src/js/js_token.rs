use super::super::parse::*;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Debug, DisplayFromDebug, PartialEq)]
pub enum JsToken {
    Document,
    Number,
    OperatorAdd,
    Semicolon,
    Terminator,
}

impl Token for JsToken {
    fn regex(&self) -> &str {
        match self {
            Self::Document => "",
            Self::Number => r"\s*([\d_]+)\s*",
            Self::OperatorAdd => r"\s*(\+)\s*",
            Self::Semicolon => r"\s*(;)\s*",
            Self::Terminator => r"\s*$",
        }
    }

    #[rustfmt::skip]
    fn next_tokens(&self) -> Vec<Box<dyn Token>> {
        match self {
            Self::Document => vec![
                Box::new(Self::Number),
                Box::new(Self::OperatorAdd),
                Box::new(Self::Semicolon),
                Box::new(Self::Terminator),
            ],
            Self::Number => vec![
                Box::new(Self::OperatorAdd),
                Box::new(Self::Semicolon),
                Box::new(Self::Terminator),
            ],
            Self::OperatorAdd => vec![
                Box::new(Self::Number),
            ],
            Self::Semicolon => vec![
                Box::new(Self::Number),
                Box::new(Self::OperatorAdd),
                Box::new(Self::Semicolon),
                Box::new(Self::Terminator),
            ],
            Self::Terminator => vec![],
        }
    }

    fn is_terminator(&self) -> bool {
        matches!(self, Self::Terminator)
    }
}
