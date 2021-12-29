use super::super::parse::*;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq)]
pub enum MathToken {
    Document,
    Number,
    Plus,
    Whitespace,
    Semicolon,
    Terminator,
}

impl Token for MathToken {
    fn regex(&self) -> &str {
        match self {
            MathToken::Document => "",
            MathToken::Number => r"\s*(-?\d+(\.\d+)?)",
            MathToken::Plus => r"\s*(\+)",
            MathToken::Whitespace => r"\s+",
            MathToken::Semicolon => r"\s*;\s*",
            MathToken::Terminator => r"^$",
        }
    }

    #[rustfmt::skip]
    fn next_tokens(&self) -> Vec<MathToken> {
        match self {
            MathToken::Document => vec![
                MathToken::Whitespace,
                MathToken::Number,
                MathToken::Terminator,
            ],
            MathToken::Number => vec![
                MathToken::Plus,
                MathToken::Semicolon,
                MathToken::Whitespace,
            ],
            MathToken::Plus => vec![
                MathToken::Number
            ],
            MathToken::Whitespace => vec![
                MathToken::Whitespace,
                MathToken::Terminator,
            ],
            MathToken::Semicolon => vec![
                MathToken::Number,
                MathToken::Whitespace,
                MathToken::Semicolon,
                MathToken::Terminator,
            ],
            MathToken::Terminator => vec![],
        }
    }

    fn is_terminator(&self) -> bool {
        matches!(self, MathToken::Terminator)
    }
}
