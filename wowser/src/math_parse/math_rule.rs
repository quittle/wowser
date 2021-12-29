use super::super::parse::*;
use super::math_token::MathToken;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq, Eq, Hash)]
pub enum MathRule {
    Document,
    DocumentBody,
    Statement,
    Expression,
    BinaryExpression,
    BinaryOperator,
    Semicolon,
    Number,
    Terminator,
}

impl MathRule {}

impl Rule<MathToken> for MathRule {
    #[rustfmt::skip]
    fn children(&self) -> Vec<RuleType<MathRule, MathToken>> {
        match self {
            MathRule::Document => vec![RuleType::Sequence(vec![
                MathRule::DocumentBody,
                MathRule::Terminator,
            ])],
            MathRule::DocumentBody => vec![RuleType::RepeatableRule(MathRule::Statement)],
            MathRule::Statement => vec![RuleType::Sequence(vec![
                MathRule::Expression,
                MathRule::Semicolon,
            ])],
            MathRule::Expression => vec![
                RuleType::Rule(MathRule::BinaryExpression),
                RuleType::Rule(MathRule::Number),
            ],
            MathRule::BinaryExpression => vec![RuleType::Sequence(vec![
                MathRule::Number,
                MathRule::BinaryOperator,
                MathRule::Expression,
            ])],
            MathRule::BinaryOperator => vec![RuleType::Token(MathToken::Plus)],
            MathRule::Semicolon => vec![RuleType::Token(MathToken::Semicolon)],
            MathRule::Number => vec![RuleType::Token(MathToken::Number)],
            MathRule::Terminator => vec![RuleType::Token(MathToken::Terminator)],
        }
    }
}
