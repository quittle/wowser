use std::collections::HashMap;

use super::{get_member_from_prototype_chain, JsClosureContext, JsValue, JsValueNode};

#[derive(Debug, PartialEq)]
pub enum JsExpression {
    Boolean(bool),
    Number(f64),
    String(String),
    Undefined,
    Null,
    Object(Vec<(String, JsExpression)>),
    TripleEquals(bool, Box<JsExpression>, Box<JsExpression>),
    DoubleEquals(bool, Box<JsExpression>, Box<JsExpression>),
    Add(Box<JsExpression>, Box<JsExpression>),
    Multiply(Box<JsExpression>, Box<JsExpression>),
    Reference(String),
    CastToNumber(Box<JsExpression>),
    InvokeFunction(Box<JsExpression>, Vec<JsExpression>),
    AccessMember(Box<JsExpression>, String),
    Condition(Box<JsExpression>, Box<JsExpression>, Box<JsExpression>),
}

impl JsExpression {
    pub fn run(&self, closure_context: &mut JsClosureContext) -> JsValueNode {
        match self {
            Self::Reference(variable_name) => closure_context
                .get_reference_mut(variable_name)
                .map(|reference| reference.value.clone())
                .unwrap_or_else(|| JsValue::reference_error_rc(&closure_context.nodes_graph)),
            Self::Boolean(b) => JsValue::bool_rc(&closure_context.nodes_graph, *b),
            Self::Number(num) => JsValue::number_rc(&closure_context.nodes_graph, *num),
            Self::String(num) => JsValue::str_rc(&closure_context.nodes_graph, num),
            Self::Undefined => JsValue::undefined_rc(&closure_context.nodes_graph),
            Self::Null => JsValue::null_rc(&closure_context.nodes_graph),
            Self::Object(members) => {
                let mut map = HashMap::with_capacity(members.len());
                for (key, value) in members {
                    map.insert(key.to_string(), value.run(closure_context));
                }
                JsValue::object_rc(&closure_context.nodes_graph, map)
            }
            Self::TripleEquals(match_equality, a, b) => {
                let a_value = a.run(closure_context);
                let b_value = b.run(closure_context);
                let result = match (a_value.get_ref(), b_value.get_ref()) {
                    (JsValue::Boolean(a), JsValue::Boolean(b)) => a == b,
                    (JsValue::Number(a), JsValue::Number(b)) => a == b,
                    (JsValue::String(a), JsValue::String(b)) => a == b,
                    (JsValue::Function(a), JsValue::Function(b)) => a == b,
                    (JsValue::Null, JsValue::Null) => true,
                    (JsValue::Undefined, JsValue::Undefined) => true,
                    (JsValue::Object(_), JsValue::Object(_)) => a_value.is_same_ref(&b_value),
                    (_, _) => false,
                };
                JsValue::bool_rc(&closure_context.nodes_graph, *match_equality == result)
            }
            Self::DoubleEquals(match_equality, a, b) => {
                let a_value = a.run(closure_context);
                let b_value = b.run(closure_context);
                let result = match (a_value.get_ref(), b_value.get_ref()) {
                    (JsValue::String(a), JsValue::String(b)) => a == b,
                    (
                        a @ JsValue::Boolean(_) | a @ JsValue::Number(_) | a @ JsValue::String(_),
                        b @ JsValue::Boolean(_) | b @ JsValue::Number(_) | b @ JsValue::String(_),
                    ) => f64::from(a) == f64::from(b),
                    (
                        JsValue::Null | JsValue::Undefined,
                        JsValue::Boolean(_)
                        | JsValue::Number(_)
                        | JsValue::String(_)
                        | JsValue::Function(_),
                    ) => false,
                    (
                        JsValue::Boolean(_)
                        | JsValue::Number(_)
                        | JsValue::String(_)
                        | JsValue::Function(_),
                        JsValue::Null | JsValue::Undefined,
                    ) => false,
                    (JsValue::Null | JsValue::Undefined, JsValue::Null | JsValue::Undefined) => {
                        true
                    }
                    (JsValue::Boolean(_) | JsValue::Number(_), JsValue::Function(_)) => false,
                    (JsValue::Function(_), JsValue::Boolean(_) | JsValue::Number(_)) => false,
                    (a @ JsValue::String(_), b @ JsValue::Function(_)) => {
                        a.to_string() == b.to_string()
                    }
                    (a @ JsValue::Function(_), b @ JsValue::String(_)) => {
                        a.to_string() == b.to_string()
                    }
                    (JsValue::Object(_), JsValue::Object(_)) => a_value.is_same_ref(&b_value),
                    (JsValue::Object(_), _) => false,
                    (_, JsValue::Object(_)) => false,
                    (JsValue::Function(a), JsValue::Function(b)) => a == b,
                };
                JsValue::bool_rc(&closure_context.nodes_graph, *match_equality == result)
            }
            Self::Add(a, b) => {
                let a_value = a.run(closure_context);
                let b_value = b.run(closure_context);
                match (a_value.get_ref(), b_value.get_ref()) {
                    (
                        a @ JsValue::Number(_)
                        | a @ JsValue::Boolean(_)
                        | a @ JsValue::Undefined
                        | a @ JsValue::Null,
                        b @ JsValue::Number(_)
                        | b @ JsValue::Boolean(_)
                        | b @ JsValue::Undefined
                        | b @ JsValue::Null,
                    ) => JsValue::number_rc(
                        &closure_context.nodes_graph,
                        f64::from(a) + f64::from(b),
                    ),
                    (a @ JsValue::Object(_), b) => JsValue::string_rc(
                        &closure_context.nodes_graph,
                        a.to_string() + &b.to_string(),
                    ),
                    (a, b @ JsValue::Object(_)) => JsValue::string_rc(
                        &closure_context.nodes_graph,
                        a.to_string() + &b.to_string(),
                    ),
                    (a @ JsValue::String(_) | a @ JsValue::Function(_), b) => JsValue::string_rc(
                        &closure_context.nodes_graph,
                        a.to_string() + &b.to_string(),
                    ),
                    (a, b @ JsValue::String(_) | b @ JsValue::Function(_)) => JsValue::string_rc(
                        &closure_context.nodes_graph,
                        a.to_string() + &b.to_string(),
                    ),
                }
            }
            Self::Multiply(a, b) => {
                let a_value = a.run(closure_context);
                let b_value = b.run(closure_context);
                JsValue::number_rc(
                    &closure_context.nodes_graph,
                    f64::from(a_value.get_ref()) * f64::from(b_value.get_ref()),
                )
            }
            Self::CastToNumber(expression) => {
                let value = expression.run(closure_context);
                JsValue::number_rc(&closure_context.nodes_graph, f64::from(value.get_ref()))
            }
            Self::InvokeFunction(reference_to_invoke, arg_expressions) => {
                let (this_value, value) =
                    if let JsExpression::AccessMember(base, name) = reference_to_invoke.as_ref() {
                        let this_value = base.run(closure_context);
                        let value = get_member_from_prototype_chain(
                            this_value.get_ref(),
                            name,
                            closure_context,
                        );
                        (this_value, value)
                    } else {
                        (
                            JsValue::undefined_rc(&closure_context.nodes_graph),
                            reference_to_invoke.run(closure_context),
                        )
                    };

                match value.get_ref() {
                    JsValue::Function(function) => {
                        let mut evaluated_args: Vec<_> = Vec::with_capacity(arg_expressions.len());
                        for expression in arg_expressions {
                            evaluated_args.push(expression.run(closure_context))
                        }
                        function.run(closure_context, this_value, &evaluated_args)
                    }
                    _ => JsValue::type_error_rc(&closure_context.nodes_graph),
                }
            }
            Self::AccessMember(reference, member_name) => {
                let base_value = reference.run(closure_context);
                get_member_from_prototype_chain(base_value.get_ref(), member_name, closure_context)
            }
            Self::Condition(conditional_expression, true_expression, false_expression) => {
                let condition_result = conditional_expression.run(closure_context);
                let condition_truthiness: bool = condition_result.get_ref().into();

                if condition_truthiness {
                    true_expression.run(closure_context)
                } else {
                    false_expression.run(closure_context)
                }
            }
        }
    }

    pub fn get_referenced_nodes(&self) -> Vec<JsValueNode> {
        match self {
            Self::Boolean(_) => vec![],
            Self::Number(_) => vec![],
            Self::String(_) => vec![],
            Self::Undefined => vec![],
            Self::Null => vec![],
            Self::Object(entries) => entries
                .iter()
                .flat_map(|(_name, expression)| expression.get_referenced_nodes())
                .collect(),
            Self::TripleEquals(_, a, b) => [a, b]
                .iter()
                .flat_map(|expression| expression.get_referenced_nodes())
                .collect(),
            Self::DoubleEquals(_, a, b) => [a, b]
                .iter()
                .flat_map(|expression| expression.get_referenced_nodes())
                .collect(),
            Self::Add(a, b) => [a, b]
                .iter()
                .flat_map(|expression| expression.get_referenced_nodes())
                .collect(),
            Self::Multiply(a, b) => [a, b]
                .iter()
                .flat_map(|expression| expression.get_referenced_nodes())
                .collect(),
            Self::Reference(_) => vec![],
            Self::CastToNumber(expression) => expression.get_referenced_nodes(),
            Self::InvokeFunction(this_expression, args_expressions) => [
                this_expression.get_referenced_nodes(),
                args_expressions
                    .iter()
                    .flat_map(|expression| expression.get_referenced_nodes())
                    .collect(),
            ]
            .concat(),
            Self::AccessMember(expression, _) => expression.get_referenced_nodes(),
            Self::Condition(a, b, c) => [a, b, c]
                .iter()
                .flat_map(|expression| expression.get_referenced_nodes())
                .collect(),
        }
    }
}
