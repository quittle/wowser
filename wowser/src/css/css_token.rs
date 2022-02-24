use super::super::parse::*;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq)]
pub enum CssToken {
    Document,
    SelectorSeparator,
    Selector,
    AtKeyword,
    AtKeywordSymbol,
    OpenBrace,
    CloseBrace,
    PropertyKey,
    PropertySeparator,
    PropertyValue,
    PropertyTerminator,
    Terminator,
}

impl Token for CssToken {
    fn regex(&self) -> &str {
        match self {
            Self::Document => "",
            Self::Selector => r"\s*([#\-_\.\w\d\[\]]+)\s*",
            Self::SelectorSeparator => r"\s*(,)\s*",
            Self::AtKeyword => r"\s*(@\w[\w\-]*)\s",
            Self::AtKeywordSymbol => r"\s*(\w+)\s",
            Self::OpenBrace => r"\s*(\{)\s*",
            Self::CloseBrace => r"\s*(\})\s*",
            Self::PropertyKey => r"[\-\w\.\d]+",
            Self::PropertySeparator => r"\s*(:)\s*",
            Self::PropertyValue => r#"\s*([#'"\w\d\-\(\),]+)\s*"#,
            Self::PropertyTerminator => r"\s*(;)\s*",
            Self::Terminator => r"\s*$",
        }
    }

    #[rustfmt::skip]
    fn next_tokens(&self) -> Vec<CssToken> {
        match self {
            Self::Document => vec![
                Self::Selector,
                Self::Terminator,
            ],
            Self::Selector => vec![
                Self::Selector,
                Self::SelectorSeparator,
                Self::OpenBrace,
            ],
            Self::SelectorSeparator => vec![
                Self::Selector,
            ],
            Self::AtKeyword => vec![
                Self::AtKeywordSymbol,
            ],
            Self::AtKeywordSymbol =>vec![
                Self::AtKeywordSymbol,
                Self::OpenBrace,
            ],
            Self::OpenBrace => vec![
                Self::PropertyKey,
                Self::CloseBrace,
            ],
            Self::PropertyKey => vec![
                Self::PropertySeparator,
            ],
            Self::PropertySeparator => vec![
                Self::PropertyValue,
            ],
            Self::PropertyValue => vec![
                Self::PropertyTerminator,
                Self::CloseBrace,
            ],
            Self::PropertyTerminator => vec![
                Self::PropertyKey,
                Self::CloseBrace,
            ],
            Self::CloseBrace => vec![
                Self::Selector,
                Self::Terminator,
            ],
            Self::Terminator => vec![],
        }
    }

    fn is_terminator(&self) -> bool {
        matches!(self, Self::Terminator)
    }
}
