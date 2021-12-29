use super::super::parse::*;
use super::html_token::HtmlToken;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq, Eq, Hash)]
pub enum HtmlRule {
    Document,
    Doctype,
    DoctypeStart,
    DoctypeContents,
    DoctypeContentsString,
    TagEnd,
    Tag,
    SelfClosingTag,
    NonSelfClosingTag,
    OpeningTagPrelude,
    OpeningTagName,
    OpeningTagAttributes,
    OpeningTag,
    SelfClosingTagEnding,
    TagAttribute,
    AttributeName,
    AttributeEquals,
    AttributeValue,
    TagContents,
    TagsAndText,
    TagAndText,
    Text,
    ClosingTag,
    ClosingTagStart,
    Terminator,
}

impl HtmlRule {}

impl Rule for HtmlRule {
    type Token = HtmlToken;

    #[rustfmt::skip]
    fn children(&self) -> Vec<RuleType<HtmlRule>> {
        match self {
            HtmlRule::Document => vec![
                RuleType::Sequence(vec![
                    HtmlRule::Doctype,
                    HtmlRule::TagContents,
                    HtmlRule::Terminator,
                ]),
                RuleType::Sequence(vec![HtmlRule::Doctype, HtmlRule::Terminator]),
                RuleType::Sequence(vec![HtmlRule::TagContents, HtmlRule::Terminator]),
                RuleType::Rule(HtmlRule::Terminator),
            ],
            HtmlRule::Doctype => vec![RuleType::Sequence(vec![
                HtmlRule::DoctypeStart,
                HtmlRule::DoctypeContents,
                HtmlRule::TagEnd,
            ])],
            HtmlRule::DoctypeStart => vec![RuleType::Token(HtmlToken::DoctypeOpen)],
            HtmlRule::DoctypeContents => vec![RuleType::RepeatableRule(
                HtmlRule::DoctypeContentsString,
            )],
            HtmlRule::DoctypeContentsString => vec![
                RuleType::Token(HtmlToken::DoctypeUnquotedString),
                RuleType::Token(HtmlToken::DoctypeQuotedString),
            ],
            HtmlRule::TagEnd => vec![RuleType::Token(HtmlToken::TagEnd)],
            HtmlRule::Tag => vec![
                RuleType::Rule(HtmlRule::SelfClosingTag),
                RuleType::Rule(HtmlRule::NonSelfClosingTag),
            ],
            HtmlRule::SelfClosingTag => vec![RuleType::Sequence(vec![
                HtmlRule::OpeningTagPrelude,
                HtmlRule::SelfClosingTagEnding,
            ])],
            HtmlRule::SelfClosingTagEnding => {
                vec![RuleType::Token(HtmlToken::TagSelfClosingEnd)]
            }
            HtmlRule::NonSelfClosingTag => vec![RuleType::Sequence(vec![
                HtmlRule::OpeningTag,
                HtmlRule::TagContents,
                HtmlRule::ClosingTag,
            ])],
            HtmlRule::OpeningTag => vec![RuleType::Sequence(vec![
                HtmlRule::OpeningTagPrelude,
                HtmlRule::TagEnd,
            ])],
            HtmlRule::OpeningTagPrelude => vec![RuleType::Sequence(vec![
                HtmlRule::OpeningTagName,
                HtmlRule::OpeningTagAttributes,
            ])],
            HtmlRule::OpeningTagName => vec![RuleType::Token(HtmlToken::TagStart)],
            HtmlRule::OpeningTagAttributes => {
                vec![RuleType::RepeatableRule(HtmlRule::TagAttribute)]
            }
            HtmlRule::TagAttribute => vec![
                RuleType::Sequence(vec![
                    HtmlRule::AttributeName,
                    HtmlRule::AttributeEquals,
                    HtmlRule::AttributeValue,
                ]),
                RuleType::Sequence(vec![
                    HtmlRule::AttributeName,
                    HtmlRule::AttributeEquals,
                ]),
                RuleType::Rule(HtmlRule::AttributeName),
            ],
            HtmlRule::AttributeName => vec![RuleType::Token(HtmlToken::AttributeName)],
            HtmlRule::AttributeEquals => vec![RuleType::Token(HtmlToken::Equals)],
            HtmlRule::AttributeValue => vec![
                RuleType::Token(HtmlToken::QuotedString),
                RuleType::Token(HtmlToken::UnquotedString),
            ],
            HtmlRule::TagContents => vec![
                RuleType::Sequence(vec![HtmlRule::Text, HtmlRule::TagsAndText]),
                RuleType::Rule(HtmlRule::TagsAndText),
                RuleType::Rule(HtmlRule::Text),
            ],
            HtmlRule::TagsAndText => vec![RuleType::RepeatableRule(HtmlRule::TagAndText)],
            HtmlRule::TagAndText => vec![
                RuleType::Sequence(vec![HtmlRule::Tag, HtmlRule::Text]),
                RuleType::Rule(HtmlRule::Tag),
            ],
            HtmlRule::Text => vec![RuleType::Token(HtmlToken::Text)],
            HtmlRule::ClosingTag => vec![RuleType::Sequence(vec![
                HtmlRule::ClosingTagStart,
                HtmlRule::TagEnd,
            ])],
            HtmlRule::ClosingTagStart => vec![RuleType::Token(HtmlToken::ClosingTagStart)],
            HtmlRule::Terminator => vec![RuleType::Token(HtmlToken::Terminator)],
        }
    }
}
