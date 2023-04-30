use super::{super::parse::*, js_token::JsToken};
use wowser_macros::DisplayFromDebug;

#[derive(Clone, Copy, Debug, DisplayFromDebug, PartialEq, Eq, Hash)]
pub enum JsRule {
    Document,
    Statements,
    Statement,
    IfKeyword,
    IfStatement,
    ElseKeyword,
    ElseStatement,
    VarDeclaration,
    VarKeyword,
    ThisKeyword,
    FunctionDeclaration,
    FunctionKeyword,
    FunctionParams,
    ReturnKeyword,
    ThrowKeyword,
    ObjectLiteral,
    ObjectMembers,
    TrueKeyword,
    FalseKeyword,
    NullKeyword,
    RightHandSideVariable,
    VariableName,
    VariableAssignment,
    Expression,
    ExpressionAdd,
    ExpressionSubAdd,
    ExpressionMultiply,
    ExpressionSubMultiply,
    ExpressionEquality,
    ExpressionSubEquality,
    ExpressionFunctionInvoke,
    FunctionInvoke,
    FunctionArguments,
    ExpressionConditional,
    ExpressionSubConditional,
    OperatorAdd,
    OperatorMultiply,
    OperatorEquals,
    OperatorEquality,
    DotAccess,
    OpenParen,
    CloseParen,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Dot,
    Comma,
    LiteralValue,
    Number,
    String,
    Undefined,
    NaNKeyword,
    Colon,
    Semicolon,
    QuestionMark,
    Terminator,
}

impl JsRule {}

impl Rule for JsRule {
    type Token = JsToken;

    #[rustfmt::skip]
    fn children(&self) -> Vec<RuleType<Self>> {
        match self {
            Self::Document => vec![
                RuleType::Sequence(vec![Self::Statements, Self::VarDeclaration, Self::Terminator]),
                RuleType::Sequence(vec![Self::Statements, Self::Expression, Self::Terminator]),
                RuleType::Sequence(vec![Self::Statements, Self::VariableAssignment, Self::Terminator]),
                RuleType::Sequence(vec![Self::Statements, Self::Terminator]),
                RuleType::Rule(Self::Terminator),
            ],
            Self::Statements => vec![
                RuleType::RepeatableRule(Self::Statement),
            ],
            Self::Statement => vec![
                RuleType::Rule(Self::FunctionDeclaration),
                RuleType::Rule(Self::IfStatement),
                RuleType::Sequence(vec![Self::ReturnKeyword, Self::Expression, Self::Semicolon]),
                RuleType::Sequence(vec![Self::ThrowKeyword, Self::Expression, Self::Semicolon]),
                RuleType::Sequence(vec![Self::VarDeclaration, Self::Semicolon]),
                RuleType::Sequence(vec![Self::Expression, Self::Semicolon]),
                RuleType::Sequence(vec![Self::VariableAssignment, Self::Semicolon]),
                RuleType::Sequence(vec![Self::Semicolon]),
            ],
            Self::VarDeclaration => vec![
                RuleType::Sequence(vec![Self::VarKeyword, Self::VariableName, Self::OperatorEquals, Self::Expression]),
                RuleType::Sequence(vec![Self::VarKeyword, Self::VariableName]),
            ],
            Self::VarKeyword => vec![
                RuleType::Token(JsToken::VarKeyword),
            ],
            Self::ThisKeyword => vec![
                RuleType::Token(JsToken::ThisKeyword),
            ],
            Self::FunctionDeclaration => vec![
                RuleType::Sequence(vec![
                    Self::FunctionKeyword,
                    Self::VariableName,
                    Self::OpenParen,
                    Self::FunctionParams,
                    Self::CloseParen,
                    Self::OpenCurlyBrace,
                    Self::Statements,
                    Self::CloseCurlyBrace
                ]),
            ],
            Self::FunctionKeyword => vec![
                RuleType::Token(JsToken::FunctionKeyword),
            ],
            Self::FunctionParams => vec![
                RuleType::Sequence(vec![Self::VariableName, Self::Comma, Self::FunctionParams]),
                RuleType::Sequence(vec![Self::VariableName, Self::Comma]),
                RuleType::Rule(Self::VariableName),
                RuleType::Sequence(vec![]),
            ],
            Self::ReturnKeyword => vec![
                RuleType::Token(JsToken::ReturnKeyword),
            ],
            Self::ThrowKeyword => vec![
                RuleType::Token(JsToken::ThrowKeyword),
            ],
            Self::ObjectLiteral => vec![
                RuleType::Sequence(vec![Self::OpenCurlyBrace, Self::ObjectMembers, Self::CloseCurlyBrace]),
            ],
            Self::ObjectMembers => vec![
                RuleType::Sequence(vec![Self::String, Self::Colon, Self::Expression, Self::Comma, Self::ObjectMembers]),
                RuleType::Sequence(vec![Self::String, Self::Colon, Self::Expression, Self::Comma]),
                RuleType::Sequence(vec![Self::String, Self::Colon, Self::Expression]),
                RuleType::Sequence(vec![]),
            ],
            Self::IfStatement => vec![
                RuleType::Sequence(vec![
                    Self::IfKeyword,
                    Self::OpenParen,
                    Self::Expression,
                    Self::CloseParen,
                    Self::OpenCurlyBrace,
                    Self::Statements,
                    Self::CloseCurlyBrace,
                    Self::ElseStatement,
                ]),
                RuleType::Sequence(vec![
                    Self::IfKeyword,
                    Self::OpenParen,
                    Self::Expression,
                    Self::CloseParen,
                    Self::Statement,
                    Self::ElseStatement,
                ]),
                RuleType::Sequence(vec![
                    Self::IfKeyword,
                    Self::OpenParen,
                    Self::Expression,
                    Self::CloseParen,
                    Self::Expression,
                    Self::ElseStatement,
                ]),
            ],
            Self::ElseStatement => vec![
                RuleType::Sequence(vec![
                    Self::ElseKeyword,
                    Self::OpenCurlyBrace,
                    Self::Statements,
                    Self::CloseCurlyBrace,
                ]),
                RuleType::Sequence(vec![
                    Self::ElseKeyword,
                    Self::Statement
                ]),
                RuleType::Sequence(vec![
                    Self::ElseKeyword,
                    Self::Expression,
                ]),
                RuleType::Sequence(vec![]),
            ],
            Self::IfKeyword => vec![
                RuleType::Token(JsToken::IfKeyword),
            ],
            Self::ElseKeyword => vec![
                RuleType::Token(JsToken::ElseKeyword),
            ],
            Self::TrueKeyword => vec![
                RuleType::Token(JsToken::TrueKeyword),
            ],
            Self::FalseKeyword => vec![
                RuleType::Token(JsToken::FalseKeyword),
            ],
            Self::NullKeyword => vec![
                RuleType::Token(JsToken::NullKeyword),
            ],
            Self::RightHandSideVariable => vec![
                RuleType::Rule(Self::ThisKeyword),
                RuleType::Rule(Self::VariableName),
            ],
            Self::VariableName => vec![
                RuleType::Token(JsToken::VariableName),
            ],
            Self::VariableAssignment => vec![
                RuleType::Sequence(vec![Self::VariableName, Self::OperatorEquals, Self::Expression]),
            ],
            Self::Expression => vec![
                RuleType::Rule(Self::ExpressionConditional),
                RuleType::Rule(Self::ExpressionEquality),
                RuleType::Rule(Self::ExpressionAdd),
                RuleType::Rule(Self::ExpressionMultiply),
                RuleType::Rule(Self::ExpressionFunctionInvoke),
                RuleType::Rule(Self::DotAccess),
                RuleType::Rule(Self::RightHandSideVariable),
                RuleType::Rule(Self::LiteralValue),
            ],
            Self::ExpressionConditional => vec![
                RuleType::Sequence(vec![
                    JsRule::ExpressionSubConditional,
                    JsRule::QuestionMark,
                    JsRule::Expression,
                    JsRule::Colon,
                    JsRule::ExpressionSubConditional,
                ]),
            ],
            Self::ExpressionSubConditional => vec![
                RuleType::Rule(Self::ExpressionFunctionInvoke),
                RuleType::Rule(Self::ExpressionEquality),
                RuleType::Rule(Self::ExpressionAdd),
                RuleType::Rule(Self::ExpressionMultiply),
                RuleType::Rule(Self::RightHandSideVariable),
                RuleType::Rule(Self::LiteralValue),
            ],
            Self::ExpressionEquality => vec![
                RuleType::Sequence(vec![Self::ExpressionFunctionInvoke, Self::OperatorEquality, Self::ExpressionSubEquality]),
                RuleType::Sequence(vec![Self::ExpressionAdd, Self::OperatorEquality, Self::ExpressionSubEquality]),
                RuleType::Sequence(vec![Self::ExpressionMultiply, Self::OperatorEquality, Self::ExpressionSubEquality]),
                RuleType::Sequence(vec![Self::RightHandSideVariable, Self::OperatorEquality, Self::ExpressionSubEquality]),
                RuleType::Sequence(vec![Self::LiteralValue, Self::OperatorEquality, Self::ExpressionSubEquality]),
            ],
            Self::ExpressionSubEquality => vec![
                RuleType::Rule(Self::ExpressionFunctionInvoke),
                RuleType::Rule(Self::ExpressionEquality),
                RuleType::Rule(Self::ExpressionAdd),
                RuleType::Rule(Self::ExpressionMultiply),
                RuleType::Rule(Self::RightHandSideVariable),
                RuleType::Rule(Self::LiteralValue),
            ],
            Self::ExpressionAdd => vec![
                RuleType::Sequence(vec![Self::ExpressionFunctionInvoke, Self::OperatorAdd, Self::ExpressionSubAdd]),
                RuleType::Sequence(vec![Self::ExpressionMultiply, Self::OperatorAdd, Self::ExpressionSubAdd]),
                RuleType::Sequence(vec![Self::RightHandSideVariable, Self::OperatorAdd, Self::ExpressionSubAdd]),
                RuleType::Sequence(vec![Self::LiteralValue, Self::OperatorAdd, Self::ExpressionSubAdd]),
                RuleType::Sequence(vec![Self::OperatorAdd, Self::LiteralValue]),
            ],
            Self::ExpressionSubAdd => vec![
                RuleType::Rule(Self::ExpressionFunctionInvoke),
                RuleType::Rule(Self::ExpressionAdd),
                RuleType::Rule(Self::ExpressionMultiply),
                RuleType::Rule(Self::RightHandSideVariable),
                RuleType::Rule(Self::ExpressionAdd),
                RuleType::Rule(Self::LiteralValue),
            ],
            Self::ExpressionMultiply => vec![
                RuleType::Sequence(vec![Self::ExpressionFunctionInvoke, Self::OperatorMultiply, Self::ExpressionSubMultiply]),
                RuleType::Sequence(vec![Self::RightHandSideVariable, Self::OperatorMultiply, Self::ExpressionSubMultiply]),
                RuleType::Sequence(vec![Self::LiteralValue, Self::OperatorMultiply, Self::ExpressionSubMultiply]),
            ],
            Self::ExpressionSubMultiply => vec![
                RuleType::Rule(Self::ExpressionFunctionInvoke),
                RuleType::Rule(Self::ExpressionMultiply),
                RuleType::Rule(Self::RightHandSideVariable),
                RuleType::Rule(Self::LiteralValue),
            ],
            Self::ExpressionFunctionInvoke => vec![
                RuleType::Sequence(vec![Self::DotAccess, Self::FunctionInvoke]),
                RuleType::Sequence(vec![Self::RightHandSideVariable, Self::FunctionInvoke]),
                RuleType::Sequence(vec![Self::LiteralValue, Self::FunctionInvoke]),
            ],
            Self::FunctionInvoke => vec![
                RuleType::Sequence(vec![Self::OpenParen, Self::FunctionArguments, Self::CloseParen]),
            ],
            Self::FunctionArguments => vec![
                RuleType::Sequence(vec![Self::Expression, Self::Comma, Self::FunctionArguments]),
                RuleType::Sequence(vec![Self::Expression, Self::Comma]),
                RuleType::Sequence(vec![Self::Expression]),
                RuleType::Sequence(vec![]),
            ],
            Self::LiteralValue => vec![
                RuleType::Rule(Self::TrueKeyword),
                RuleType::Rule(Self::FalseKeyword),
                RuleType::Rule(Self::Number),
                RuleType::Rule(Self::String),
                RuleType::Rule(Self::Undefined),
                RuleType::Rule(Self::NullKeyword),
                RuleType::Rule(Self::NaNKeyword),
                RuleType::Rule(Self::ObjectLiteral),
            ],
            Self::OperatorAdd => vec![
                RuleType::Token(JsToken::OperatorAdd),
            ],
            Self::OperatorMultiply => vec![
                RuleType::Token(JsToken::OperatorMultiply),
            ],
            Self::OperatorEquals => vec![
                RuleType::Token(JsToken::OperatorEquals),
            ],
            Self::OperatorEquality => vec![
                RuleType::Token(JsToken::OperatorEquality),
            ],
            Self::DotAccess => vec![
                RuleType::Sequence(vec![JsRule::RightHandSideVariable, JsRule::Dot, JsRule::DotAccess]),
                RuleType::Sequence(vec![JsRule::RightHandSideVariable, JsRule::Dot, JsRule::VariableName]),
                RuleType::Sequence(vec![JsRule::LiteralValue, JsRule::Dot, JsRule::DotAccess]),
                RuleType::Sequence(vec![JsRule::LiteralValue, JsRule::Dot, JsRule::VariableName]),
            ],
            Self::OpenParen => vec! [
                RuleType::Token(JsToken::OpenParen),
            ],
            Self::CloseParen => vec![
                RuleType::Token(JsToken::CloseParen),
            ],
            Self::OpenCurlyBrace => vec![
                RuleType::Token(JsToken::OpenCurlyBrace),
            ],
            Self::CloseCurlyBrace => vec![
                RuleType::Token(JsToken::CloseCurlyBrace),
            ],
            Self::Dot => vec![
                RuleType::Token(JsToken::Dot),
            ],
            Self::Comma => vec![
                RuleType::Token(JsToken::Comma),
            ],
            Self::Number => vec![
                RuleType::Token(JsToken::Number),
            ],
            Self::String => vec![
                RuleType::Token(JsToken::String),
            ],
            Self::Undefined => vec![
                RuleType::Token(JsToken::Undefined),
            ],
            Self::NaNKeyword => vec![
                RuleType::Token(JsToken::NaNKeyword),
            ],
            Self::Colon => vec![
                RuleType::Token(JsToken::Colon),
            ],
            Self::Semicolon => vec![
                RuleType::Token(JsToken::Semicolon),
            ],
            Self::QuestionMark => vec![
                RuleType::Token(JsToken::QuestionMark),
            ],
            Self::Terminator => vec![
                RuleType::Token(JsToken::Terminator)
            ],
        }
    }
}
