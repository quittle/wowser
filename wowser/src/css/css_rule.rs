use super::super::parse::*;
use super::css_token::CssToken;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq, Eq, Hash)]
pub enum CssRule {
    Document,
    TopLevelEntries,
    Blocks,
    Block,
    SelectorList,
    Selector,
    SelectorItem,
    SelectorSeparator,
    AtRule,
    AtKeyword,
    AtKeywordSymbols,
    AtKeywordSymbol,
    BlockBody,
    BlockBodyOpen,
    BlockBodyClose,
    PropertyList,
    StrictPropertyList,
    TrailingProperty,
    Property,
    PropertyKey,
    PropertySeparator,
    PropertyValue,
    PropertyTerminator,
    Terminator,
}

impl CssRule {}

impl Rule for CssRule {
    type Token = CssToken;

    #[rustfmt::skip]
    fn children(&self) -> Vec<RuleType<CssRule>> {
        match self {
            Self::Document => vec![
                RuleType::Sequence(vec![Self::TopLevelEntries, Self::Terminator]),
                RuleType::Rule(Self::Terminator),
            ],
            Self::TopLevelEntries => vec![
                RuleType::Sequence(vec![Self::Block, Self::TopLevelEntries]),
                RuleType::Sequence(vec![Self::AtRule, Self::TopLevelEntries]),
                RuleType::Sequence(vec![]),
            ],
            Self::Blocks => vec![
                RuleType::RepeatableRule(Self::Block),
            ],
            Self::Block => vec![
                RuleType::Sequence(vec![Self::SelectorList, Self::BlockBody]),
            ],
            Self::SelectorList => vec![
                RuleType::Sequence(vec![Self::Selector, Self::SelectorSeparator, Self::SelectorList]),
                RuleType::Rule(Self::Selector),
            ],
            Self::Selector => vec![
                RuleType::RepeatableRule(Self::SelectorItem)
            ],
            Self::SelectorItem => vec![
                RuleType::Token(CssToken::Selector)
            ],
            Self::SelectorSeparator => vec![
                RuleType::Token(CssToken::SelectorSeparator),
            ],
            Self::AtRule => vec![
                RuleType::Sequence(vec![Self::AtKeyword, Self::AtKeywordSymbols, Self::Blocks]),
            ],
            Self::AtKeywordSymbols => vec![
                RuleType::RepeatableRule(Self::AtKeywordSymbol),
            ],
            Self::AtKeyword => vec![
                RuleType::Token(CssToken::AtKeyword),
            ],
            Self::AtKeywordSymbol => vec![
                RuleType::Token(CssToken::AtKeywordSymbol),
            ],
            Self::BlockBody => vec![
                RuleType::Sequence(vec![Self::BlockBodyOpen, Self::PropertyList, Self::BlockBodyClose])
            ],
            Self::BlockBodyOpen => vec![
                RuleType::Token(CssToken::OpenBrace)
            ],
            Self::BlockBodyClose => vec![
                RuleType::Token(CssToken::CloseBrace)
            ],
            Self::PropertyList => vec![
                RuleType::Sequence(vec![Self::StrictPropertyList, Self::TrailingProperty]),
                RuleType::Rule(Self::StrictPropertyList),
            ],
            Self::StrictPropertyList => vec![
                RuleType::RepeatableRule(Self::Property)
            ],
            Self::TrailingProperty => vec![
                RuleType::Sequence(vec![
                    Self::PropertyKey,
                    Self::PropertySeparator,
                    Self::PropertyValue,
                    // No PropertyTerminator
                ])
            ],
            Self::Property => vec![
                RuleType::Sequence(vec![
                    Self::PropertyKey,
                    Self::PropertySeparator,
                    Self::PropertyValue,
                    Self::PropertyTerminator,
                ])
            ],
            Self::PropertyKey => vec![
                RuleType::Token(CssToken::PropertyKey)
            ],
            Self::PropertySeparator => vec![
                RuleType::Token(CssToken::PropertySeparator)
            ],
            Self::PropertyValue => vec![
                RuleType::Token(CssToken::PropertyValue),
            ],
            Self::PropertyTerminator => vec![
                RuleType::Token(CssToken::PropertyTerminator)
            ],
            Self::Terminator => vec![RuleType::Token(CssToken::Terminator)],
        }
    }
}
