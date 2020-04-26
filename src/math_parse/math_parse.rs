// mod crate::parse;
use super::super::parse::*; //{Rule, RuleType, Token, RuleClone, TokenClone};
use std::convert::TryFrom;
use std::any::Any;
use std::borrow::Borrow;

#[derive(Clone, Debug)]
pub enum MathRule {
    Document, // Root isn't referenced
    DocumentBody,
    Statement,
    Expression,
    BinaryExpression,
    // Parens, // Not yet
    BinaryOperator,
    // Add, // Not needed?
    Semicolon,
    Number,
    Terminator,
}

trait T {
    fn foo(&self) -> &'static str;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

struct A {}

impl T for A {
    fn foo(&self) -> &'static str {
        "a"
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}


// fn foo<'a>(rule: &Box<dyn Rule>) -> Result<MathRule, &'static str> {
//     // let b: &Box<dyn Rule> = rule;
//     let b = (*rule).b() as Box<Any>;
//     // let b = *b as Box<dyn Any>;
//     let result = b.downcast::<MathRule>();
//     match result {
//         Ok(r) => Ok(*r),
//         Err(_) => Err("bad")
//     }
//     // (rule as &dyn Any);
// }

// impl MathRule {
//     pub fn try_from(rule: Box<dyn Rule>) -> Result<Self, &'static str> {
//         // let result = (&Box::new(*rule) as &Box<dyn Any>).downcast::<MathRule>();
//         let result =  (rule).into_any().downcast::<MathRule>();
//         // let result = foo(&*rule);
//         // result
//         match result {
//             Ok(rule) => Ok(*rule),
//             Err(_) => Err("ub oh")
//         }
//     }
// }

impl MathRule {
    // pub fn try_from(rule: Box<dyn Rule>) -> Result<Self, &'static str> {
    //     // let result = (&Box::new(*rule) as &Box<dyn Any>).downcast::<MathRule>();
    //     let result =  (rule).into_any().downcast::<MathRule>();
    //     // let result = foo(&*rule);
    //     // result
    //     match result {
    //         Ok(rule) => Ok(*rule),
    //         Err(_) => Err("ub oh")
    //     }
    // }
}
 
impl Rule for MathRule {
    fn children(&self) -> Vec<RuleType<MathRule>> {
        match self {
            MathRule::Document => vec!(RuleType::Sequence(vec!(MathRule::DocumentBody.b(), MathRule::Terminator.b()))),
            MathRule::DocumentBody => vec!(RuleType::RepeatableRule(MathRule::Statement.b())),
            MathRule::Statement => vec!(RuleType::Sequence(vec!(MathRule::Expression.b(), MathRule::Semicolon.b()))),
            
            MathRule::Expression => vec!(RuleType::Rule(MathRule::BinaryExpression.b()), RuleType::Rule(MathRule::Number.b())),
            MathRule::BinaryExpression => vec!(RuleType::Sequence(vec!(MathRule::Number.b(), MathRule::BinaryOperator.b(), MathRule::Expression.b()))), // should be expression, operator, expression
            MathRule::BinaryOperator => vec!(RuleType::Token(MathToken::Plus.b())),
            // MathRule::Add => vec!(RuleType::Tokens(vec!(MathToken::Number.b(), MathToken::Plus.b(), MathToken::Number.b()))),
            MathRule::Semicolon => vec!(RuleType::Token(MathToken::Semicolon.b())),
            MathRule::Number => vec!(RuleType::Token(MathToken::Number.b())),
            MathRule::Terminator => vec!(RuleType::Token(MathToken::Terminator.b())),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MathToken {
    Document,
    Number,
    Plus,
    Whitespace,
    Semicolon,
    Terminator
}

impl Token for MathToken {
    fn regex(&self) -> &str {
        match self {
            MathToken::Document => "",
            MathToken::Number => r"\s*\d+(\.\d+)?",
            MathToken::Plus => r"\s*\+",
            MathToken::Whitespace => r"\s+",
            MathToken::Semicolon => r";",
            MathToken::Terminator => r"^$",
        }
    }

    fn next_tokens(&self) -> Vec<Box<dyn Token>> {
        match self {
            MathToken::Document => vec!(Box::new(MathToken::Whitespace), Box::new(MathToken::Number), Box::new(MathToken::Terminator)),
            MathToken::Number => vec!(Box::new(MathToken::Plus), Box::new(MathToken::Semicolon), Box::new(MathToken::Whitespace), Box::new(MathToken::Terminator)),
            MathToken::Plus => vec!(Box::new(MathToken::Number)),
            MathToken::Whitespace => vec!(Box::new(MathToken::Whitespace), Box::new(MathToken::Terminator)),
            MathToken::Semicolon => vec!(Box::new(MathToken::Whitespace), Box::new(MathToken::Terminator)), // eh
            MathToken::Terminator => vec!(),
        }
    }

    fn is_terminator(&self) -> bool {
        match self {
            MathToken::Terminator => true,
            _ => false
        }
    }
}