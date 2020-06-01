use super::super::parse::*;
use super::html_token::HtmlToken;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Debug, DisplayFromDebug)]
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
    #[rustfmt::skip]
    fn children(&self) -> Vec<RuleType<HtmlRule>> {
        match self {
            HtmlRule::Document => vec![
                RuleType::Sequence(vec![
                    HtmlRule::Doctype.b(),
                    HtmlRule::TagContents.b(),
                    HtmlRule::Terminator.b(),
                ]),
                RuleType::Sequence(vec![HtmlRule::Doctype.b(), HtmlRule::Terminator.b()]),
                RuleType::Sequence(vec![HtmlRule::TagContents.b(), HtmlRule::Terminator.b()]),
                RuleType::Rule(HtmlRule::Terminator.b()),
            ],
            HtmlRule::Doctype => vec![RuleType::Sequence(vec![
                HtmlRule::DoctypeStart.b(),
                HtmlRule::DoctypeContents.b(),
                HtmlRule::TagEnd.b(),
            ])],
            HtmlRule::DoctypeStart => vec![RuleType::Token(HtmlToken::DoctypeOpen.b())],
            HtmlRule::DoctypeContents => vec![RuleType::RepeatableRule(
                HtmlRule::DoctypeContentsString.b(),
            )],
            HtmlRule::DoctypeContentsString => vec![
                RuleType::Token(HtmlToken::DoctypeUnquotedString.b()),
                RuleType::Token(HtmlToken::DoctypeQuotedString.b()),
            ],
            HtmlRule::TagEnd => vec![RuleType::Token(HtmlToken::TagEnd.b())],
            HtmlRule::Tag => vec![
                RuleType::Rule(HtmlRule::SelfClosingTag.b()),
                RuleType::Rule(HtmlRule::NonSelfClosingTag.b()),
            ],
            HtmlRule::SelfClosingTag => vec![RuleType::Sequence(vec![
                HtmlRule::OpeningTagPrelude.b(),
                HtmlRule::SelfClosingTagEnding.b(),
            ])],
            HtmlRule::SelfClosingTagEnding => {
                vec![RuleType::Token(HtmlToken::TagSelfClosingEnd.b())]
            }
            HtmlRule::NonSelfClosingTag => vec![RuleType::Sequence(vec![
                HtmlRule::OpeningTag.b(),
                HtmlRule::TagContents.b(),
                HtmlRule::ClosingTag.b(),
            ])],
            HtmlRule::OpeningTag => vec![RuleType::Sequence(vec![
                HtmlRule::OpeningTagPrelude.b(),
                HtmlRule::TagEnd.b(),
            ])],
            HtmlRule::OpeningTagPrelude => vec![RuleType::Sequence(vec![
                HtmlRule::OpeningTagName.b(),
                HtmlRule::OpeningTagAttributes.b(),
            ])],
            HtmlRule::OpeningTagName => vec![RuleType::Token(HtmlToken::TagStart.b())],
            HtmlRule::OpeningTagAttributes => {
                vec![RuleType::RepeatableRule(HtmlRule::TagAttribute.b())]
            }
            HtmlRule::TagAttribute => vec![
                RuleType::Sequence(vec![
                    HtmlRule::AttributeName.b(),
                    HtmlRule::AttributeEquals.b(),
                    HtmlRule::AttributeValue.b(),
                ]),
                RuleType::Sequence(vec![
                    HtmlRule::AttributeName.b(),
                    HtmlRule::AttributeEquals.b(),
                ]),
                RuleType::Rule(HtmlRule::AttributeName.b()),
            ],
            HtmlRule::AttributeName => vec![RuleType::Token(HtmlToken::AttributeName.b())],
            HtmlRule::AttributeEquals => vec![RuleType::Token(HtmlToken::Equals.b())],
            HtmlRule::AttributeValue => vec![
                RuleType::Token(HtmlToken::QuotedString.b()),
                RuleType::Token(HtmlToken::UnquotedString.b()),
            ],
            HtmlRule::TagContents => vec![
                RuleType::Sequence(vec![HtmlRule::Text.b(), HtmlRule::TagsAndText.b()]),
                RuleType::Rule(HtmlRule::TagsAndText.b()),
                RuleType::Rule(HtmlRule::Text.b()),
            ],
            HtmlRule::TagsAndText => vec![RuleType::RepeatableRule(HtmlRule::TagAndText.b())],
            HtmlRule::TagAndText => vec![
                RuleType::Sequence(vec![HtmlRule::Tag.b(), HtmlRule::Text.b()]),
                RuleType::Rule(HtmlRule::Tag.b()),
            ],
            HtmlRule::Text => vec![RuleType::Token(HtmlToken::Text.b())],
            HtmlRule::ClosingTag => vec![RuleType::Sequence(vec![
                HtmlRule::ClosingTagStart.b(),
                HtmlRule::TagEnd.b(),
            ])],
            HtmlRule::ClosingTagStart => vec![RuleType::Token(HtmlToken::ClosingTagStart.b())],
            HtmlRule::Terminator => vec![RuleType::Token(HtmlToken::Terminator.b())],
        }
    }
}
