use super::super::parse::*;
use super::css_token::CssToken;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Debug, DisplayFromDebug, PartialEq)]
pub enum CssRule {
    Document,
    Blocks,
    Block,
    SelectorList,
    Selector,
    SelectorItem,
    SelectorSeparator,
    BlockBody,
    BlockBodyOpen,
    BlockBodyClose,
    PropertyList,
    Property,
    PropertyKey,
    PropertySeparator,
    PropertyValue,
    PropertyTerminator,
    Terminator,
}

impl CssRule {}

impl Rule for CssRule {
    #[rustfmt::skip]
    fn children(&self) -> Vec<RuleType<Self>> {
        match self {
            Self::Document => vec![
                RuleType::Sequence(vec![Self::Blocks.b(), Self::Terminator.b()]),
                RuleType::Rule(Self::Terminator.b()),
            ],
            Self::Blocks => vec![
                RuleType::RepeatableRule(Self::Block.b()),
            ],
            Self::Block => vec![
                RuleType::Sequence(vec![Self::SelectorList.b(), Self::BlockBody.b()]),
            ],
            Self::SelectorList => vec![
                RuleType::Sequence(vec![Self::Selector.b(), Self::SelectorSeparator.b(), Self::SelectorList.b()]),
                RuleType::Rule(Self::Selector.b()),
            ],
            Self::Selector => vec![
                RuleType::RepeatableRule(Self::SelectorItem.b())
            ],
            Self::SelectorItem => vec![
                RuleType::Token(CssToken::Selector.b())
            ],
            Self::SelectorSeparator => vec![
                RuleType::Token(CssToken::SelectorSeparator.b()),
            ],
            Self::BlockBody => vec![
                RuleType::Sequence(vec![Self::BlockBodyOpen.b(), Self::PropertyList.b(), Self::BlockBodyClose.b()])
            ],
            Self::BlockBodyOpen => vec![
                RuleType::Token(CssToken::OpenBrace.b())
            ],
            Self::BlockBodyClose => vec![
                RuleType::Token(CssToken::CloseBrace.b())
            ],
            Self::PropertyList => vec![
                RuleType::RepeatableRule(Self::Property.b())
            ],
            Self::Property => vec![
                RuleType::Sequence(vec![
                    Self::PropertyKey.b(),
                    Self::PropertySeparator.b(),
                    Self::PropertyValue.b(),
                    Self::PropertyTerminator.b(),
                ])
            ],
            Self::PropertyKey => vec![
                RuleType::Token(CssToken::PropertyKey.b())
            ],
            Self::PropertySeparator => vec![
                RuleType::Token(CssToken::PropertySeparator.b())
            ],
            Self::PropertyValue => vec![
                RuleType::Token(CssToken::PropertyValue.b())
            ],
            Self::PropertyTerminator => vec![
                RuleType::Token(CssToken::PropertyTerminator.b())
            ],
            Self::Terminator => vec![RuleType::Token(CssToken::Terminator.b())],
        }
    }
}
