use super::super::parse::*;

#[derive(Clone, Debug, PartialEq)]
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
            MathToken::Number => r"\s*-?\d+(\.\d+)?",
            MathToken::Plus => r"\s*\+",
            MathToken::Whitespace => r"\s+",
            MathToken::Semicolon => r"\s*;\s*",
            MathToken::Terminator => r"^$",
        }
    }

    fn next_tokens(&self) -> Vec<Box<dyn Token>> {
        match self {
            MathToken::Document => vec![
                Box::new(MathToken::Whitespace),
                Box::new(MathToken::Number),
                Box::new(MathToken::Terminator),
            ],
            MathToken::Number => vec![
                Box::new(MathToken::Plus),
                Box::new(MathToken::Semicolon),
                Box::new(MathToken::Whitespace),
            ],
            MathToken::Plus => vec![Box::new(MathToken::Number)],
            MathToken::Whitespace => vec![
                Box::new(MathToken::Whitespace),
                Box::new(MathToken::Terminator),
            ],
            MathToken::Semicolon => vec![
                Box::new(MathToken::Number),
                Box::new(MathToken::Whitespace),
                Box::new(MathToken::Semicolon),
                Box::new(MathToken::Terminator),
            ],
            MathToken::Terminator => vec![],
        }
    }

    fn is_terminator(&self) -> bool {
        match self {
            MathToken::Terminator => true,
            _ => false,
        }
    }
}
