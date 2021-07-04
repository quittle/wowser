use super::super::parse::*;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Debug, DisplayFromDebug, PartialEq)]
pub enum CssToken {
    Document,
    SelectorSeparator,
    Selector,
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
    fn next_tokens(&self) -> Vec<Box<dyn Token>> {
        match self {
            Self::Document => vec![
                Box::new(Self::Selector),
                Box::new(Self::Terminator),
            ],
            Self::Selector => vec![
                Box::new(Self::Selector),
                Box::new(Self::SelectorSeparator),
                Box::new(Self::OpenBrace),
            ],
            Self::SelectorSeparator => vec![
                Box::new(Self::Selector),
            ],
            Self::OpenBrace => vec![
                Box::new(Self::PropertyKey),
                Box::new(Self::CloseBrace),
            ],
            Self::PropertyKey => vec![
                Box::new(Self::PropertySeparator),
            ],
            Self::PropertySeparator => vec![
                Box::new(Self::PropertyValue),
            ],
            Self::PropertyValue => vec![
                Box::new(Self::PropertyTerminator),
                Box::new(Self::CloseBrace),
            ],
            Self::PropertyTerminator => vec![
                Box::new(Self::PropertyKey),
                Box::new(Self::CloseBrace),
            ],
            Self::CloseBrace => vec![
                Box::new(Self::Selector),
                Box::new(Self::Terminator),
            ],
            Self::Terminator => vec![],
        }
    }

    fn is_terminator(&self) -> bool {
        matches!(self, Self::Terminator)
    }
}
