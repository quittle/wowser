use super::super::parse::*;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Debug, DisplayFromDebug, PartialEq)]
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
            HtmlToken::DoctypeUnquotedString => r#"\s*([^"\s]+)"#,
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

    fn next_tokens(&self) -> Vec<Box<dyn Token>> {
        match self {
            HtmlToken::Document => vec![
                Box::new(HtmlToken::DoctypeOpen),
                Box::new(HtmlToken::TagStart),
                Box::new(HtmlToken::Text),
                Box::new(HtmlToken::Terminator),
            ],
            HtmlToken::DoctypeOpen => vec![
                Box::new(HtmlToken::TagEnd),
                Box::new(HtmlToken::DoctypeUnquotedString),
                Box::new(HtmlToken::DoctypeQuotedString),
                Box::new(HtmlToken::Terminator),
            ],
            HtmlToken::DoctypeUnquotedString => vec![
                Box::new(HtmlToken::TagEnd),
                Box::new(HtmlToken::DoctypeUnquotedString),
                Box::new(HtmlToken::DoctypeQuotedString),
                Box::new(HtmlToken::Terminator),
            ],
            HtmlToken::DoctypeQuotedString => vec![
                Box::new(HtmlToken::TagEnd),
                Box::new(HtmlToken::DoctypeUnquotedString),
                Box::new(HtmlToken::DoctypeQuotedString),
                Box::new(HtmlToken::Terminator),
            ],
            HtmlToken::TagStart => vec![
                Box::new(HtmlToken::AttributeName),
                Box::new(HtmlToken::TagEnd),
                Box::new(HtmlToken::TagSelfClosingEnd),
            ],
            HtmlToken::AttributeName => vec![
                Box::new(HtmlToken::Equals),
                Box::new(HtmlToken::AttributeName),
                Box::new(HtmlToken::TagEnd),
                Box::new(HtmlToken::TagSelfClosingEnd),
                Box::new(HtmlToken::Terminator),
            ],
            HtmlToken::Equals => vec![
                Box::new(HtmlToken::QuotedString),
                Box::new(HtmlToken::UnquotedString),
                Box::new(HtmlToken::Terminator),
            ],
            HtmlToken::QuotedString => vec![
                Box::new(HtmlToken::AttributeName),
                Box::new(HtmlToken::TagEnd),
                Box::new(HtmlToken::TagSelfClosingEnd),
                Box::new(HtmlToken::Terminator),
            ],
            HtmlToken::UnquotedString => vec![
                Box::new(HtmlToken::AttributeName),
                Box::new(HtmlToken::TagEnd),
                Box::new(HtmlToken::TagSelfClosingEnd),
                Box::new(HtmlToken::Terminator),
            ],
            HtmlToken::TagSelfClosingEnd => vec![
                Box::new(HtmlToken::TagStart),
                Box::new(HtmlToken::ClosingTagStart),
                Box::new(HtmlToken::Text),
                Box::new(HtmlToken::Terminator),
            ],
            HtmlToken::TagEnd => vec![
                Box::new(HtmlToken::TagStart),
                Box::new(HtmlToken::ClosingTagStart),
                Box::new(HtmlToken::Text),
                Box::new(HtmlToken::Terminator),
            ],
            HtmlToken::Text => vec![
                Box::new(HtmlToken::TagStart),
                Box::new(HtmlToken::ClosingTagStart),
                Box::new(HtmlToken::Terminator),
            ],
            HtmlToken::ClosingTagStart => vec![
                Box::new(HtmlToken::TagEnd),
                Box::new(HtmlToken::Text),
                Box::new(HtmlToken::Terminator),
            ],
            HtmlToken::Terminator => vec![],
        }
    }

    fn is_terminator(&self) -> bool {
        match self {
            HtmlToken::Terminator => true,
            _ => false,
        }
    }
}
