use super::{JsDocument, JsExpression, JsRule, JsStatement};
use crate::{
    js::JsReference,
    parse::{ASTNode, Interpreter},
};

type JsASTNode<'a> = ASTNode<'a, JsRule>;

pub struct JsInterpreter {}

fn on_statements(statements: &JsASTNode) -> Vec<JsStatement> {
    let ASTNode { rule, children, .. } = statements;
    assert_eq!(
        *rule,
        JsRule::Statements,
        "Unexpected child type: {:?}",
        rule
    );

    children.iter().map(on_statement).collect()
}

fn on_statement(statement: &JsASTNode) -> JsStatement {
    let ASTNode { rule, children, .. } = statement;
    assert_eq!(
        *rule,
        JsRule::Statement,
        "Unexpected child type: {:?}",
        rule
    );

    let first_child = &children[0];
    match first_child.rule {
        JsRule::Semicolon => JsStatement::Empty,
        JsRule::Expression => JsStatement::Expression(on_expression(first_child)),
        JsRule::VarDeclaration => JsStatement::VarDeclaration(on_var_declaration(first_child)),
        rule => panic!("Unexpected child of Statement: {}", rule),
    }
}

fn on_var_declaration(expression: &JsASTNode) -> JsReference {
    let ASTNode { rule, children, .. } = expression;
    assert_eq!(
        *rule,
        JsRule::VarDeclaration,
        "Unexpected child type: {:?}",
        rule
    );

    assert_eq!(
        children.len(),
        2,
        "Unexpected number of children for VarDeclaration"
    );

    let variable_name = &children[1];

    JsReference {
        name: variable_name.token.unwrap().literal.into(),
    }
}

fn on_expression(expression: &JsASTNode) -> JsExpression {
    let ASTNode { rule, children, .. } = expression;
    assert_eq!(
        *rule,
        JsRule::Expression,
        "Unexpected child type: {:?}",
        rule
    );

    assert_eq!(
        children.len(),
        1,
        "Unexpected number of children for JsExpression"
    );

    let child = &children[0];

    match child.rule {
        JsRule::Number => on_number(child),
        JsRule::ExpressionAdd => on_expression_add(child),
        JsRule::ExpressionMultiply => on_expression_multiply(child),
        _ => panic!("Unexpected rule: {}", rule),
    }
}

fn on_number(node: &JsASTNode) -> JsExpression {
    let ASTNode { rule, token, .. } = node;

    assert_eq!(*rule, JsRule::Number, "Unexpected child type: {:?}", rule);

    let number = token.unwrap().literal;
    let normalized_number = number.replace("_", "");
    let number_value = normalized_number.parse::<f64>().unwrap();

    JsExpression::Number(number_value)
}

fn on_expression_add(node: &JsASTNode) -> JsExpression {
    let ASTNode { rule, children, .. } = node;

    assert_eq!(
        *rule,
        JsRule::ExpressionAdd,
        "Unexpected child type: {:?}",
        rule
    );

    let first_child = &children[0];

    match first_child.rule {
        JsRule::ExpressionMultiply => {
            let a = on_expression_multiply(&children[0]);
            let b = on_expression(&children[2]);
            JsExpression::Add(Box::new(a), Box::new(b))
        }
        JsRule::OperatorAdd => on_number(&children[1]),
        JsRule::Number => {
            let a = on_number(&children[0]);
            let b = on_expression(&children[2]);
            JsExpression::Add(Box::new(a), Box::new(b))
        }
        _ => panic!("Invalid first type type"),
    }
}

fn on_expression_multiply(node: &JsASTNode) -> JsExpression {
    let ASTNode { rule, children, .. } = node;

    assert_eq!(
        *rule,
        JsRule::ExpressionMultiply,
        "Unexpected child type: {:?}",
        rule
    );

    let a = on_number(&children[0]);
    let b = on_expression_sub_multiply(&children[2]);
    JsExpression::Multiply(Box::new(a), Box::new(b))
}

fn on_expression_sub_multiply(node: &JsASTNode) -> JsExpression {
    let ASTNode { rule, children, .. } = node;

    assert_eq!(
        *rule,
        JsRule::ExpressionSubMultiply,
        "Unexpected child type: {:?}",
        rule
    );

    let first_child = &children[0];
    match first_child.rule {
        JsRule::ExpressionMultiply => on_expression_multiply(first_child),
        JsRule::Number => on_number(first_child),
        rule => panic!("Invalid child type {}", rule),
    }
}

impl Interpreter<'_, JsRule> for JsInterpreter {
    type Result = JsDocument;

    fn on_node(&self, document: &JsASTNode) -> Option<JsDocument> {
        let ASTNode { rule, children, .. } = document;
        assert_eq!(*rule, JsRule::Document, "Unexpected child type: {:?}", rule);

        let first_child = &children[0];

        let statements = match first_child.rule {
            JsRule::Terminator => vec![],
            JsRule::Expression => vec![JsStatement::Expression(on_expression(first_child))],
            JsRule::VarDeclaration => {
                vec![JsStatement::VarDeclaration(on_var_declaration(first_child))]
            }
            JsRule::Statements => {
                let mut statements = on_statements(first_child);
                let second_child = &children[1];
                match second_child.rule {
                    JsRule::Expression => {
                        statements.push(JsStatement::Expression(on_expression(second_child)))
                    }
                    JsRule::VarDeclaration => statements.push(JsStatement::VarDeclaration(
                        on_var_declaration(second_child),
                    )),
                    _ => {}
                };
                statements
            }
            rule => panic!("Unspported first rule: {}", rule),
        };

        Some(JsDocument {
            statements,
            expression_results: vec![],
        })
    }
}
