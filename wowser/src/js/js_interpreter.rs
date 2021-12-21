use super::{JsDocument, JsExpression, JsRule, JsStatement};
use crate::parse::{ASTNode, Interpreter};

pub struct JsInterpreter {}

fn on_statements(statements: &ASTNode<JsRule>) -> Vec<JsStatement> {
    let ASTNode { rule, children, .. } = statements;
    assert_eq!(
        **rule,
        JsRule::Statements,
        "Unexpected child type: {:?}",
        rule
    );

    children.iter().map(on_statement).collect()
}

fn on_statement(statement: &ASTNode<JsRule>) -> JsStatement {
    let ASTNode { rule, children, .. } = statement;
    assert_eq!(
        **rule,
        JsRule::Statement,
        "Unexpected child type: {:?}",
        rule
    );

    let first_child = &children[0];
    if JsRule::Semicolon == *first_child.rule {
        JsStatement { expression: None }
    } else {
        assert_eq!(
            children.len(),
            2,
            "Unexpected number of children for JsStatement"
        );

        JsStatement {
            expression: Some(on_expression(first_child)),
        }
    }
}

fn on_expression(expression: &ASTNode<JsRule>) -> JsExpression {
    let ASTNode { rule, children, .. } = expression;
    assert_eq!(
        **rule,
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

    match *child.rule {
        JsRule::Number => on_number(child),
        JsRule::ExpressionAdd => on_expression_add(child),
        _ => panic!("Unexpected rule: {}", rule),
    }
}

fn on_number(node: &ASTNode<JsRule>) -> JsExpression {
    let ASTNode { rule, token, .. } = node;

    assert_eq!(**rule, JsRule::Number, "Unexpected child type: {:?}", rule);

    let number = token.unwrap().literal;
    let normalized_number = number.replace("_", "");
    let number_value = normalized_number.parse::<f64>().unwrap();

    JsExpression::Number(number_value)
}

fn on_expression_add(node: &ASTNode<JsRule>) -> JsExpression {
    let ASTNode { rule, children, .. } = node;

    assert_eq!(
        **rule,
        JsRule::ExpressionAdd,
        "Unexpected child type: {:?}",
        rule
    );

    let first_child = &children[0];

    match *first_child.rule {
        JsRule::OperationAdd => on_number(&children[1]),
        JsRule::Number => {
            let a = on_number(&children[0]);
            let b = on_expression(&children[2]);
            JsExpression::Add(Box::new(a), Box::new(b))
        }
        _ => panic!("Invalid first type type"),
    }
}

impl Interpreter<'_> for JsInterpreter {
    type RuleType = JsRule;
    type Result = JsDocument;

    fn on_node(&self, document: &ASTNode<JsRule>) -> Option<JsDocument> {
        let ASTNode { rule, children, .. } = document;
        assert_eq!(
            **rule,
            JsRule::Document,
            "Unexpected child type: {:?}",
            rule
        );

        if children.len() == 1 && *children[0].rule == JsRule::Terminator {
            Some(JsDocument {
                statements: vec![],
                expression_results: vec![],
            })
        } else {
            let mut statements = on_statements(&children[0]);
            let second_child = &children[1];
            if JsRule::Expression == *second_child.rule {
                statements.push(JsStatement {
                    expression: Some(on_expression(second_child)),
                })
            }
            Some(JsDocument {
                statements,
                expression_results: vec![],
            })
        }
    }
}
