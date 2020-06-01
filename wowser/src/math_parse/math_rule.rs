use super::super::parse::*;
use super::math_token::MathToken;
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Debug, DisplayFromDebug)]
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

impl Rule for MathRule {
    #[rustfmt::skip]
    fn children(&self) -> Vec<RuleType<MathRule>> {
        match self {
            MathRule::Document => vec![RuleType::Sequence(vec![
                MathRule::DocumentBody.b(),
                MathRule::Terminator.b(),
            ])],
            MathRule::DocumentBody => vec![RuleType::RepeatableRule(MathRule::Statement.b())],
            MathRule::Statement => vec![RuleType::Sequence(vec![
                MathRule::Expression.b(),
                MathRule::Semicolon.b(),
            ])],
            MathRule::Expression => vec![
                RuleType::Rule(MathRule::BinaryExpression.b()),
                RuleType::Rule(MathRule::Number.b()),
            ],
            MathRule::BinaryExpression => vec![RuleType::Sequence(vec![
                MathRule::Number.b(),
                MathRule::BinaryOperator.b(),
                MathRule::Expression.b(),
            ])],
            MathRule::BinaryOperator => vec![RuleType::Token(MathToken::Plus.b())],
            MathRule::Semicolon => vec![RuleType::Token(MathToken::Semicolon.b())],
            MathRule::Number => vec![RuleType::Token(MathToken::Number.b())],
            MathRule::Terminator => vec![RuleType::Token(MathToken::Terminator.b())],
        }
    }
}
