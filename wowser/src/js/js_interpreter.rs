use super::{JsDocument, JsExpression, JsRule, JsStatement};
use crate::{
    js::JsReference,
    parse::{
        extract_interpreter_children, extract_interpreter_n_children, extract_interpreter_token,
        ASTNode, Interpreter,
    },
};

type JsASTNode<'a> = ASTNode<'a, JsRule>;

pub struct JsInterpreter {}

fn on_statements(statements: &JsASTNode) -> Vec<JsStatement> {
    let children = extract_interpreter_children(statements, JsRule::Statements);

    children.iter().map(on_statement).collect()
}

fn on_statement(statement: &JsASTNode) -> JsStatement {
    let children = extract_interpreter_children(statement, JsRule::Statement);

    let first_child = &children[0];
    match first_child.rule {
        JsRule::Semicolon => JsStatement::Empty,
        JsRule::Expression => JsStatement::Expression(on_expression(first_child)),
        JsRule::VarDeclaration => JsStatement::VarDeclaration(on_var_declaration(first_child)),
        rule => panic!("Unexpected child of Statement: {}", rule),
    }
}

fn on_var_declaration(var_declaration: &JsASTNode) -> JsReference {
    let children = extract_interpreter_n_children(var_declaration, JsRule::VarDeclaration, 2);

    JsReference {
        name: on_variable_name(&children[1]),
    }
}

fn on_variable_name(variable_name: &JsASTNode) -> String {
    extract_interpreter_token(variable_name, JsRule::VariableName)
}

fn on_expression(expression: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(expression, JsRule::Expression, 1);

    let child = &children[0];

    match child.rule {
        JsRule::Number => on_number(child),
        JsRule::ExpressionAdd => on_expression_add(child),
        JsRule::ExpressionMultiply => on_expression_multiply(child),
        rule => panic!("Unexpected rule: {}", rule),
    }
}

fn on_number(node: &JsASTNode) -> JsExpression {
    let token = extract_interpreter_token(node, JsRule::Number);
    let normalized_number = token.replace("_", "");
    let number_value = normalized_number.parse::<f64>().unwrap();
    JsExpression::Number(number_value)
}

fn on_expression_add(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_children(node, JsRule::ExpressionAdd);

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
    let children = extract_interpreter_n_children(node, JsRule::ExpressionMultiply, 3);

    let a = on_number(&children[0]);
    let b = on_expression_sub_multiply(&children[2]);
    JsExpression::Multiply(Box::new(a), Box::new(b))
}

fn on_expression_sub_multiply(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::ExpressionSubMultiply, 1);

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
        let children = extract_interpreter_children(document, JsRule::Document);

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
