use super::json_token::JsonToken;
use crate::parse::*;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq, Eq, Hash)]
pub(super) enum JsonRule {
    Document,
    Literal,
    StringToken,
    Array,
    ArrayEntries,
    Object,
    ObjectEntries,
    ObjectEntry,
    Value,
    OpenCurlyBraceToken,
    CloseCurlyBraceToken,
    OpenSquareBraceToken,
    CloseSquareBraceToken,
    ColonToken,
    CommaToken,
    TerminatorToken,
}

impl Rule for JsonRule {
    type Token = JsonToken;

    #[rustfmt::skip]
    fn children(&self) -> Vec<RuleType<Self>> {
        match self {
            Self::Document => vec![
                RuleType::Sequence(vec![Self::Literal, Self::TerminatorToken]),
                RuleType::Sequence(vec![Self::Array, Self::TerminatorToken]),
                RuleType::Sequence(vec![Self::Object, Self::TerminatorToken]),
            ],
            Self::Literal => vec![
                RuleType::Token(Self::Token::String),
                RuleType::Token(Self::Token::Number),
                RuleType::Token(Self::Token::Boolean),
                RuleType::Token(Self::Token::Null),
            ],
            Self::StringToken => vec![
                RuleType::Token(Self::Token::String),
            ],
            Self::Array => vec![
                RuleType::Sequence(vec![Self::OpenSquareBraceToken, Self::ArrayEntries, Self::CloseSquareBraceToken]),
            ],
            Self::ArrayEntries => vec![
                RuleType::Sequence(vec![Self::Value, Self::CommaToken, Self::ArrayEntries]),
                RuleType::Sequence(vec![Self::Value]),
                RuleType::Sequence(vec![]),
            ],
            Self::Object => vec![
                RuleType::Sequence(vec![Self::OpenCurlyBraceToken, Self::ObjectEntries, Self::CloseCurlyBraceToken]),
            ],
            Self::ObjectEntries => vec![
                RuleType::Sequence(vec![Self::ObjectEntry, Self::CommaToken, Self::ObjectEntries]),
                RuleType::Sequence(vec![Self::ObjectEntry]),
                RuleType::Sequence(vec![]),
            ],
            Self::ObjectEntry => vec![
                RuleType::Sequence(vec![Self::StringToken, Self::ColonToken, Self::Value]),
            ],
            Self::Value => vec![
                RuleType::Rule(Self::Literal),
                RuleType::Rule(Self::Array),
                RuleType::Rule(Self::Object),
            ],
            Self::OpenCurlyBraceToken => vec![
                RuleType::Token(Self::Token::OpenCurlyBrace),
            ],
            Self::CloseCurlyBraceToken => vec![
                RuleType::Token(Self::Token::CloseCurlyBrace),
            ],
            Self::OpenSquareBraceToken => vec![
                RuleType::Token(Self::Token::OpenSquareBrace),
            ],
            Self::CloseSquareBraceToken => vec![
                RuleType::Token(Self::Token::CloseSquareBrace),
            ],
            Self::ColonToken => vec![
                RuleType::Token(Self::Token::Colon),
            ],
            Self::CommaToken => vec![
                RuleType::Token(Self::Token::Comma),
            ],
            Self::TerminatorToken => vec![RuleType::Token(Self::Token::Terminator)],
        }
    }
}
