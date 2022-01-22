use std::rc::Rc;

use super::{JsDocument, JsExpression, JsFunction, JsRule, JsStatement, JsValue};
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
        JsRule::VarDeclaration => on_var_declaration(first_child),
        JsRule::VariableAssignment => on_variable_assignment(first_child),
        JsRule::FunctionDeclaration => on_function_declaration(first_child),
        JsRule::ReturnKeyword => JsStatement::Return(on_expression(&children[1])),
        rule => panic!("Unexpected child of Statement: {}", rule),
    }
}

fn on_var_declaration(var_declaration: &JsASTNode) -> JsStatement {
    let children = extract_interpreter_children(var_declaration, JsRule::VarDeclaration);

    let reference = JsReference {
        name: on_variable_name(&children[1]),
        value: JsValue::undefined_rc(),
    };
    if children.len() == 4 {
        JsStatement::VariableAssignment(reference, on_expression(&children[3]))
    } else {
        JsStatement::VarDeclaration(reference)
    }
}

fn on_variable_name_reference(variable_name: &JsASTNode) -> JsExpression {
    JsExpression::Reference(on_variable_name(variable_name))
}

fn on_variable_name(variable_name: &JsASTNode) -> String {
    extract_interpreter_token(variable_name, JsRule::VariableName)
}

fn on_variable_assignment(variable_assignment: &JsASTNode) -> JsStatement {
    let children =
        extract_interpreter_n_children(variable_assignment, JsRule::VariableAssignment, 3);

    JsStatement::VariableAssignment(
        JsReference {
            name: on_variable_name(&children[0]),
            value: JsValue::undefined_rc(),
        },
        on_expression(&children[2]),
    )
}

fn on_function_declaration(node: &JsASTNode) -> JsStatement {
    let children = extract_interpreter_n_children(node, JsRule::FunctionDeclaration, 8);
    let function_name = on_variable_name(&children[1]);
    let params = on_function_params(&children[3]);
    let statements = on_statements(&children[6]);
    JsStatement::FunctionDeclaration(JsReference {
        name: function_name.clone(),
        value: Rc::new(JsValue::Function(JsFunction::UserDefined(
            function_name,
            params,
            statements,
        ))),
    })
}

fn on_function_params(node: &JsASTNode) -> Vec<String> {
    let children = extract_interpreter_children(node, JsRule::FunctionParams);
    let variable_name = on_variable_name(&children[0]);
    let mut params = if children.len() == 3 {
        on_function_params(&children[2])
    } else {
        vec![]
    };
    params.insert(0, variable_name);
    params
}

fn on_expression(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::Expression, 1);

    let child = &children[0];

    match child.rule {
        JsRule::FunctionInvoke => on_function_invoke(child),
        JsRule::VariableName => on_variable_name_reference(child),
        JsRule::LiteralValue => on_literal_value(child),
        JsRule::ExpressionAdd => on_expression_add(child),
        JsRule::ExpressionMultiply => on_expression_multiply(child),
        rule => panic!("Unexpected rule: {}", rule),
    }
}

fn on_function_invoke(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::FunctionInvoke, 4);

    let reference_to_invoke = on_variable_name_reference(&children[0]);
    let arguments = on_function_arguments(&children[2]);

    JsExpression::InvokeFunction(Box::new(reference_to_invoke), arguments)
}

fn on_function_arguments(node: &JsASTNode) -> Vec<JsExpression> {
    let children = extract_interpreter_children(node, JsRule::FunctionArguments);

    let mut ret = vec![];

    if !children.is_empty() {
        ret.push(on_expression(&children[0]));
    }

    if children.len() == 3 {
        ret.extend(on_function_arguments(&children[2]))
    }

    ret
}

fn on_literal_value(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::LiteralValue, 1);

    let child = &children[0];

    match child.rule {
        JsRule::Number => on_number(child),
        JsRule::String => on_string(child),
        JsRule::Undefined => JsExpression::Undefined,
        rule => panic!("Unexpected rule: {}", rule),
    }
}

fn on_number(node: &JsASTNode) -> JsExpression {
    let token = extract_interpreter_token(node, JsRule::Number);
    let normalized_number = token.replace("_", "");
    let number_value = normalized_number.parse::<f64>().unwrap();
    JsExpression::Number(number_value)
}

fn on_string(node: &JsASTNode) -> JsExpression {
    let token = extract_interpreter_token(node, JsRule::String);
    let quote_stripped = token[1..token.len() - 1].to_string();
    JsExpression::String(quote_stripped)
}

fn on_expression_add(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_children(node, JsRule::ExpressionAdd);

    let first_child = &children[0];

    match first_child.rule {
        JsRule::FunctionInvoke => {
            let a = on_function_invoke(&children[0]);
            let b = on_expression_sub_add(&children[2]);
            JsExpression::Add(Box::new(a), Box::new(b))
        }
        JsRule::ExpressionMultiply => {
            let a = on_expression_multiply(&children[0]);
            let b = on_expression_sub_add(&children[2]);
            JsExpression::Add(Box::new(a), Box::new(b))
        }
        JsRule::VariableName => {
            let a = on_variable_name_reference(&children[0]);
            let b = on_expression_sub_add(&children[2]);
            JsExpression::Add(Box::new(a), Box::new(b))
        }
        JsRule::LiteralValue => {
            let a = on_literal_value(&children[0]);
            let b = on_expression_sub_add(&children[2]);
            JsExpression::Add(Box::new(a), Box::new(b))
        }
        JsRule::OperatorAdd => {
            let literal_value_expression = on_literal_value(&children[1]);
            JsExpression::CastToNumber(Box::new(literal_value_expression))
        }
        _ => panic!("Invalid first type type"),
    }
}

fn on_expression_sub_add(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::ExpressionSubAdd, 1);

    let first_child = &children[0];

    match first_child.rule {
        JsRule::ExpressionMultiply => on_expression_multiply(first_child),
        JsRule::VariableName => on_variable_name_reference(first_child),
        JsRule::LiteralValue => on_literal_value(first_child),
        JsRule::ExpressionAdd => on_expression_add(first_child),
        rule => panic!("Invalid first child rule: {}", rule),
    }
}

fn on_expression_multiply(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::ExpressionMultiply, 3);

    let first_child = &children[0];
    let a = match first_child.rule {
        JsRule::VariableName => on_variable_name_reference(first_child),
        JsRule::LiteralValue => on_literal_value(first_child),
        rule => panic!("Invalid first child rule: {}", rule),
    };
    let b = on_expression_sub_multiply(&children[2]);
    JsExpression::Multiply(Box::new(a), Box::new(b))
}

fn on_expression_sub_multiply(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::ExpressionSubMultiply, 1);

    let first_child = &children[0];
    match first_child.rule {
        JsRule::ExpressionMultiply => on_expression_multiply(first_child),
        JsRule::LiteralValue => on_literal_value(first_child),
        JsRule::VariableName => on_variable_name_reference(first_child),
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
                vec![on_var_declaration(first_child)]
            }
            JsRule::VariableAssignment => vec![on_variable_assignment(first_child)],
            JsRule::Statements => {
                let mut statements = on_statements(first_child);
                let second_child = &children[1];
                match second_child.rule {
                    JsRule::Expression => {
                        statements.push(JsStatement::Expression(on_expression(second_child)))
                    }
                    JsRule::VarDeclaration => statements.push(on_var_declaration(second_child)),
                    JsRule::VariableAssignment => {
                        statements.push(on_variable_assignment(second_child))
                    }
                    _ => {}
                };
                statements
            }
            rule => panic!("Unspported first rule: {}", rule),
        };

        Some(JsDocument::new(statements))
    }
}
