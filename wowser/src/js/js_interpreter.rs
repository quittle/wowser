use super::{JsDocument, JsExpression, JsFunction, JsRule, JsStatement, JsValue, JsValueGraph};
use crate::{
    garbage_collector::GcNodeGraph,
    js::JsReference,
    parse::{
        extract_interpreter_children, extract_interpreter_n_children, extract_interpreter_token,
        ASTNode, Interpreter,
    },
};

type JsASTNode<'a> = ASTNode<'a, JsRule>;

pub struct JsInterpreter {}

fn on_statements(node_graph: &JsValueGraph, statements: &JsASTNode) -> Vec<JsStatement> {
    let children = extract_interpreter_children(statements, JsRule::Statements);

    children
        .iter()
        .map(|statement| on_statement(node_graph, statement))
        .collect()
}

fn on_statement(node_graph: &JsValueGraph, statement: &JsASTNode) -> JsStatement {
    let children = extract_interpreter_children(statement, JsRule::Statement);

    let first_child = &children[0];
    match first_child.rule {
        JsRule::Semicolon => JsStatement::Empty,
        JsRule::Expression => JsStatement::Expression(on_expression(first_child)),
        JsRule::VarDeclaration => on_var_declaration(node_graph, first_child),
        JsRule::VariableAssignment => on_variable_assignment(node_graph, first_child),
        JsRule::FunctionDeclaration => on_function_declaration(node_graph, first_child),
        JsRule::ReturnKeyword => JsStatement::Return(on_expression(&children[1])),
        JsRule::IfStatement => on_if_statement(node_graph, first_child),
        rule => panic!("Unexpected child of Statement: {rule}"),
    }
}

fn on_var_declaration(node_graph: &JsValueGraph, var_declaration: &JsASTNode) -> JsStatement {
    let children = extract_interpreter_children(var_declaration, JsRule::VarDeclaration);

    let reference = JsReference {
        name: on_variable_name(&children[1]),
        value: JsValue::undefined_rc(node_graph),
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

fn on_variable_assignment(
    node_graph: &JsValueGraph,
    variable_assignment: &JsASTNode,
) -> JsStatement {
    let children =
        extract_interpreter_n_children(variable_assignment, JsRule::VariableAssignment, 3);

    JsStatement::VariableAssignment(
        JsReference {
            name: on_variable_name(&children[0]),
            value: JsValue::undefined_rc(node_graph),
        },
        on_expression(&children[2]),
    )
}

fn on_function_declaration(node_graph: &JsValueGraph, node: &JsASTNode) -> JsStatement {
    let children = extract_interpreter_n_children(node, JsRule::FunctionDeclaration, 8);
    let function_name = on_variable_name(&children[1]);
    let params = on_function_params(&children[3]);
    let statements = on_statements(node_graph, &children[6]);
    let raw_text = node.rebuild_full_text().trim().to_string();
    JsStatement::FunctionDeclaration(JsReference {
        name: function_name.clone(),
        value: GcNodeGraph::create_node(
            node_graph,
            JsValue::Function(JsFunction::UserDefined(
                raw_text,
                function_name,
                params,
                statements,
            )),
        ),
    })
}

fn on_function_params(node: &JsASTNode) -> Vec<String> {
    let children = extract_interpreter_children(node, JsRule::FunctionParams);

    if children.is_empty() {
        return vec![];
    }

    let variable_name = on_variable_name(&children[0]);
    let mut params = if children.len() == 3 {
        on_function_params(&children[2])
    } else {
        vec![]
    };
    params.insert(0, variable_name);
    params
}

fn on_if_statement(node_graph: &JsValueGraph, node: &JsASTNode) -> JsStatement {
    let children = extract_interpreter_children(node, JsRule::IfStatement);

    let conditional_expression = on_expression(&children[2]);

    let execution_statements = if children.len() == 5 {
        let execution_node = &children[4];
        match execution_node.rule {
            JsRule::Expression => vec![JsStatement::Expression(on_expression(execution_node))],
            JsRule::Statement => vec![on_statement(node_graph, execution_node)],
            _ => panic!(
                "Unexpected if statement execution node: {}",
                execution_node.rule
            ),
        }
    } else {
        on_statements(node_graph, &children[5])
    };

    JsStatement::If(conditional_expression, execution_statements)
}

fn on_expression(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::Expression, 1);

    let child = &children[0];

    match child.rule {
        JsRule::ExpressionFunctionInvoke => on_expression_function_invoke(child),
        JsRule::ExpressionEquality => on_expression_equality(child),
        JsRule::ExpressionAdd => on_expression_add(child),
        JsRule::ExpressionMultiply => on_expression_multiply(child),
        JsRule::ExpressionConditional => on_expression_conditional(child),
        JsRule::VariableName => on_variable_name_reference(child),
        JsRule::LiteralValue => on_literal_value(child),
        JsRule::DotAccess => on_dot_access(child),
        rule => panic!("Unexpected rule: {rule}"),
    }
}

fn on_expression_function_invoke(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::ExpressionFunctionInvoke, 2);

    let first_child = &children[0];
    let reference_to_invoke = match first_child.rule {
        JsRule::DotAccess => on_dot_access(first_child),
        JsRule::VariableName => on_variable_name_reference(first_child),
        JsRule::LiteralValue => on_literal_value(first_child),
        rule => panic!("Unexpected rule: {rule}"),
    };

    let arguments = on_function_invoke(&children[1]);

    JsExpression::InvokeFunction(Box::new(reference_to_invoke), arguments)
}

fn on_function_invoke(node: &JsASTNode) -> Vec<JsExpression> {
    let children = extract_interpreter_n_children(node, JsRule::FunctionInvoke, 3);

    on_function_arguments(&children[1])
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

fn on_expression_conditional(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::ExpressionConditional, 5);

    let case_sub_conditional = &children[0];
    let true_condition_expression = &children[2];
    let false_condition_expression_sub_conditional = &children[4];

    JsExpression::Condition(
        Box::new(on_expression_sub_conditional(case_sub_conditional)),
        Box::new(on_expression(true_condition_expression)),
        Box::new(on_expression_sub_conditional(
            false_condition_expression_sub_conditional,
        )),
    )
}

fn on_expression_sub_conditional(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::ExpressionSubConditional, 1);

    let first_child = &children[0];

    match first_child.rule {
        JsRule::ExpressionFunctionInvoke => on_expression_function_invoke(first_child),
        JsRule::ExpressionEquality => on_expression_equality(first_child),
        JsRule::ExpressionAdd => on_expression_add(first_child),
        JsRule::ExpressionMultiply => on_expression_multiply(first_child),
        JsRule::VariableName => on_variable_name_reference(first_child),
        JsRule::LiteralValue => on_literal_value(first_child),
        rule => panic!("Unexpected rule: {rule}"),
    }
}

fn on_literal_value(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::LiteralValue, 1);

    let child = &children[0];

    match child.rule {
        JsRule::TrueKeyword => JsExpression::Boolean(true),
        JsRule::FalseKeyword => JsExpression::Boolean(false),
        JsRule::NullKeyword => JsExpression::Null,
        JsRule::Number => on_number(child),
        JsRule::String => on_string(child),
        JsRule::Undefined => JsExpression::Undefined,
        JsRule::NaNKeyword => JsExpression::Number(f64::NAN),
        JsRule::ObjectLiteral => on_object_literal(child),
        rule => panic!("Unexpected rule: {rule}"),
    }
}

fn on_number(node: &JsASTNode) -> JsExpression {
    let token = extract_interpreter_token(node, JsRule::Number);
    let normalized_number = token.replace('_', "");
    let number_value = normalized_number.parse::<f64>().unwrap();
    JsExpression::Number(number_value)
}

fn on_string(node: &JsASTNode) -> JsExpression {
    JsExpression::String(on_string_literal(node))
}

fn on_string_literal(node: &JsASTNode) -> String {
    let token = extract_interpreter_token(node, JsRule::String);
    token[1..token.len() - 1].to_string()
}

fn on_object_literal(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::ObjectLiteral, 3);
    let object_members = on_object_members(&children[1]);
    JsExpression::Object(object_members)
}

fn on_object_members(node: &JsASTNode) -> Vec<(String, JsExpression)> {
    let children = extract_interpreter_children(node, JsRule::ObjectMembers);
    if children.is_empty() {
        return vec![];
    }

    let key = on_string_literal(&children[0]);
    let value = on_expression(&children[2]);

    let mut ret = vec![(key, value)];

    if children.len() == 5 {
        ret.extend(on_object_members(&children[4]));
    }

    ret
}

fn on_expression_add(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_children(node, JsRule::ExpressionAdd);

    let first_child = &children[0];

    match first_child.rule {
        JsRule::ExpressionFunctionInvoke => {
            let a = on_expression_function_invoke(&children[0]);
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
        JsRule::ExpressionFunctionInvoke => on_expression_function_invoke(first_child),
        JsRule::ExpressionMultiply => on_expression_multiply(first_child),
        JsRule::VariableName => on_variable_name_reference(first_child),
        JsRule::LiteralValue => on_literal_value(first_child),
        JsRule::ExpressionAdd => on_expression_add(first_child),
        rule => panic!("Invalid first child rule: {rule}"),
    }
}

fn on_expression_multiply(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::ExpressionMultiply, 3);

    let first_child = &children[0];
    let a = match first_child.rule {
        JsRule::ExpressionFunctionInvoke => on_expression_function_invoke(first_child),
        JsRule::VariableName => on_variable_name_reference(first_child),
        JsRule::LiteralValue => on_literal_value(first_child),
        rule => panic!("Invalid first child rule: {rule}"),
    };
    let b = on_expression_sub_multiply(&children[2]);
    JsExpression::Multiply(Box::new(a), Box::new(b))
}

fn on_expression_sub_multiply(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::ExpressionSubMultiply, 1);

    let first_child = &children[0];
    match first_child.rule {
        JsRule::ExpressionFunctionInvoke => on_expression_function_invoke(first_child),
        JsRule::ExpressionMultiply => on_expression_multiply(first_child),
        JsRule::LiteralValue => on_literal_value(first_child),
        JsRule::VariableName => on_variable_name_reference(first_child),
        rule => panic!("Invalid child type {rule}"),
    }
}

fn on_expression_equality(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::ExpressionEquality, 3);

    let first_child = &children[0];
    let a = Box::new(match first_child.rule {
        JsRule::ExpressionFunctionInvoke => on_expression_function_invoke(first_child),
        JsRule::ExpressionAdd => on_expression_add(first_child),
        JsRule::ExpressionMultiply => on_expression_multiply(first_child),
        JsRule::VariableName => on_variable_name_reference(first_child),
        JsRule::LiteralValue => on_literal_value(first_child),
        rule => panic!("Invalid first child rule: {rule}"),
    });
    let b = Box::new(on_expression_sub_equality(&children[2]));

    match on_equality_operator(&children[1]).as_str() {
        "==" => JsExpression::DoubleEquals(true, a, b),
        "!=" => JsExpression::DoubleEquals(false, a, b),
        "===" => JsExpression::TripleEquals(true, a, b),
        "!==" => JsExpression::TripleEquals(false, a, b),
        operator => panic!("Invalid equality operator found: {operator}"),
    }
}

fn on_expression_sub_equality(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::ExpressionSubEquality, 1);

    let first_child = &children[0];
    match first_child.rule {
        JsRule::ExpressionFunctionInvoke => on_expression_function_invoke(first_child),
        JsRule::ExpressionEquality => on_expression_equality(first_child),
        JsRule::ExpressionAdd => on_expression_add(first_child),
        JsRule::ExpressionMultiply => on_expression_multiply(first_child),
        JsRule::VariableName => on_variable_name_reference(first_child),
        JsRule::LiteralValue => on_literal_value(first_child),
        rule => panic!("Invalid child type {rule}"),
    }
}

fn on_equality_operator(node: &JsASTNode) -> String {
    extract_interpreter_token(node, JsRule::OperatorEquality)
}

fn on_dot_access(node: &JsASTNode) -> JsExpression {
    let children = extract_interpreter_n_children(node, JsRule::DotAccess, 3);

    let first_child = &children[0];
    let base_value = match first_child.rule {
        JsRule::VariableName => on_variable_name_reference(first_child),
        JsRule::LiteralValue => on_literal_value(first_child),
        rule => panic!("Invalid child type {rule}"),
    };

    let access_child = &children[2];
    match access_child.rule {
        JsRule::VariableName => {
            JsExpression::AccessMember(Box::new(base_value), on_variable_name(access_child))
        }
        JsRule::DotAccess => {
            let reversed_access_chain = recurse_dot_access(node);

            let mut prev = base_value;
            for link in reversed_access_chain.iter().rev() {
                prev = JsExpression::AccessMember(Box::new(prev), link.to_string());
            }
            prev
        }
        rule => panic!("Invalid child type {rule}"),
    }
}

fn recurse_dot_access(node: &JsASTNode) -> Vec<String> {
    let children = extract_interpreter_n_children(node, JsRule::DotAccess, 3);

    let access_child = &children[2];
    match access_child.rule {
        JsRule::VariableName => vec![on_variable_name(access_child)],
        JsRule::DotAccess => {
            let mut ret = recurse_dot_access(access_child);
            ret.push(on_variable_name(&access_child.children[0]));
            ret
        }
        rule => panic!("Invalid child type {rule}"),
    }
}

impl Interpreter<'_, JsRule> for JsInterpreter {
    type Result = JsDocument;

    fn on_node(&self, document: &JsASTNode) -> Option<JsDocument> {
        let mut js_document = JsDocument::new(vec![]);
        let node_graph = &js_document.global_closure_context.nodes_graph;

        let children = extract_interpreter_children(document, JsRule::Document);

        let first_child = &children[0];

        let statements = match first_child.rule {
            JsRule::Terminator => vec![],
            JsRule::Expression => vec![JsStatement::Expression(on_expression(first_child))],
            JsRule::VarDeclaration => {
                vec![on_var_declaration(node_graph, first_child)]
            }
            JsRule::VariableAssignment => vec![on_variable_assignment(node_graph, first_child)],
            JsRule::Statements => {
                let mut statements = on_statements(node_graph, first_child);
                let second_child = &children[1];
                match second_child.rule {
                    JsRule::Expression => {
                        statements.push(JsStatement::Expression(on_expression(second_child)))
                    }
                    JsRule::VarDeclaration => {
                        statements.push(on_var_declaration(node_graph, second_child))
                    }
                    JsRule::VariableAssignment => {
                        statements.push(on_variable_assignment(node_graph, second_child))
                    }
                    _ => {}
                };
                statements
            }
            rule => panic!("Unspported first rule: {rule}"),
        };

        js_document.statements = statements;

        Some(js_document)
    }
}
