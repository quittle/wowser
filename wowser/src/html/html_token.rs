use super::super::parse::*;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq)]
pub enum HtmlToken {
    Document,
    DoctypeOpen,
    DoctypeQuotedString,
    DoctypeUnquotedString,
    TagStart,
    AttributeName,
    Equals,
    QuotedString,
    UnquotedString,
    TagSelfClosingEnd,
    TagEnd,
    Text,
    ClosingTagStart,
    Terminator,
}

impl Token for HtmlToken {
    fn regex(&self) -> &str {
        match self {
            HtmlToken::Document => "",
            HtmlToken::DoctypeOpen => r"\s*<!(?i)doctype(?-i)",
            HtmlToken::DoctypeUnquotedString => r#"\s*([^"\s>]+)"#,
            HtmlToken::DoctypeQuotedString => r#"\s*"([^"]*)""#,
            HtmlToken::TagStart => r"\s*<(\w+)",
            HtmlToken::AttributeName => r"\s*(\w[\w\d\-_]*)",
            HtmlToken::Equals => r"\s*=",
            HtmlToken::QuotedString => r#"\s*"([^"]*)""#,
            HtmlToken::UnquotedString => r"\S+",
            HtmlToken::TagSelfClosingEnd => r"\s*/>",
            HtmlToken::TagEnd => r"\s*>",
            HtmlToken::Text => r"[^<]*",
            HtmlToken::ClosingTagStart => r"</\w+",
            HtmlToken::Terminator => r"\s*$",
        }
    }

    #[rustfmt::skip]
    fn next_tokens(&self) -> Vec<HtmlToken> {
        match self {
            HtmlToken::Document => vec![
                HtmlToken::DoctypeOpen,
                HtmlToken::TagStart,
                HtmlToken::Text,
                HtmlToken::Terminator,
            ],
            HtmlToken::DoctypeOpen => vec![
                HtmlToken::TagEnd,
                HtmlToken::DoctypeUnquotedString,
                HtmlToken::DoctypeQuotedString,
                HtmlToken::Terminator,
            ],
            HtmlToken::DoctypeUnquotedString => vec![
                HtmlToken::TagEnd,
                HtmlToken::DoctypeUnquotedString,
                HtmlToken::DoctypeQuotedString,
                HtmlToken::Terminator,
            ],
            HtmlToken::DoctypeQuotedString => vec![
                HtmlToken::TagEnd,
                HtmlToken::DoctypeUnquotedString,
                HtmlToken::DoctypeQuotedString,
                HtmlToken::Terminator,
            ],
            HtmlToken::TagStart => vec![
                HtmlToken::AttributeName,
                HtmlToken::TagEnd,
                HtmlToken::TagSelfClosingEnd,
            ],
            HtmlToken::AttributeName => vec![
                HtmlToken::Equals,
                HtmlToken::AttributeName,
                HtmlToken::TagEnd,
                HtmlToken::TagSelfClosingEnd,
                HtmlToken::Terminator,
            ],
            HtmlToken::Equals => vec![
                HtmlToken::QuotedString,
                HtmlToken::UnquotedString,
                HtmlToken::Terminator,
            ],
            HtmlToken::QuotedString => vec![
                HtmlToken::AttributeName,
                HtmlToken::TagEnd,
                HtmlToken::TagSelfClosingEnd,
                HtmlToken::Terminator,
            ],
            HtmlToken::UnquotedString => vec![
                HtmlToken::AttributeName,
                HtmlToken::TagEnd,
                HtmlToken::TagSelfClosingEnd,
                HtmlToken::Terminator,
            ],
            HtmlToken::TagSelfClosingEnd => vec![
                HtmlToken::TagStart,
                HtmlToken::ClosingTagStart,
                HtmlToken::Text,
                HtmlToken::Terminator,
            ],
            HtmlToken::TagEnd => vec![
                HtmlToken::TagStart,
                HtmlToken::ClosingTagStart,
                HtmlToken::Text,
                HtmlToken::Terminator,
            ],
            HtmlToken::Text => vec![
                HtmlToken::TagStart,
                HtmlToken::ClosingTagStart,
                HtmlToken::Terminator,
            ],
            HtmlToken::ClosingTagStart => vec![
                HtmlToken::TagEnd,
                HtmlToken::Text,
                HtmlToken::Terminator,
            ],
            HtmlToken::Terminator => vec![],
        }
    }

    fn is_terminator(&self) -> bool {
        matches!(self, HtmlToken::Terminator)
    }
}
