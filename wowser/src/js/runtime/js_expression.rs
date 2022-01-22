use std::rc::Rc;

use super::{JsClosureContext, JsValue};

#[derive(Debug, PartialEq)]
pub enum JsExpression {
    Boolean(bool),
    Number(f64),
    String(String),
    Undefined,
    Null,
    Add(Box<JsExpression>, Box<JsExpression>),
    Multiply(Box<JsExpression>, Box<JsExpression>),
    Reference(String),
    CastToNumber(Box<JsExpression>),
    InvokeFunction(Box<JsExpression>, Vec<JsExpression>),
}

impl JsExpression {
    pub fn run(&self, closure_context: &mut JsClosureContext) -> Rc<JsValue> {
        match self {
            Self::Reference(variable_name) => closure_context
                .get_reference_mut(variable_name)
                .map(|reference| reference.value.clone())
                .unwrap_or_else(JsValue::reference_error_rc),
            Self::Boolean(b) => JsValue::bool_rc(*b),
            Self::Number(num) => JsValue::number_rc(*num),
            Self::String(num) => JsValue::str_rc(num),
            Self::Undefined => JsValue::undefined_rc(),
            Self::Null => JsValue::null_rc(),
            Self::Add(a, b) => {
                let a_value = a.run(closure_context);
                let b_value = b.run(closure_context);
                match (a_value.as_ref(), b_value.as_ref()) {
                    (
                        a @ JsValue::Number(_)
                        | a @ JsValue::Boolean(_)
                        | a @ JsValue::Undefined
                        | a @ JsValue::Null,
                        b @ JsValue::Number(_)
                        | b @ JsValue::Boolean(_)
                        | b @ JsValue::Undefined
                        | b @ JsValue::Null,
                    ) => JsValue::number_rc(f64::from(a) + f64::from(b)),
                    (a @ JsValue::String(_) | a @ JsValue::Function(_), b) => {
                        JsValue::string_rc(a.to_string() + &b.to_string())
                    }
                    (a, b @ JsValue::String(_) | b @ JsValue::Function(_)) => {
                        JsValue::string_rc(a.to_string() + &b.to_string())
                    }
                }
            }
            Self::Multiply(a, b) => {
                let a_value = a.run(closure_context);
                let b_value = b.run(closure_context);
                JsValue::number_rc(f64::from(a_value.as_ref()) * f64::from(b_value.as_ref()))
            }
            Self::CastToNumber(expression) => {
                let value = expression.run(closure_context);
                JsValue::number_rc(f64::from(value.as_ref()))
            }
            Self::InvokeFunction(reference_to_invoke, arg_expressions) => {
                let value = reference_to_invoke.run(closure_context);

                match value.as_ref() {
                    JsValue::Function(function) => {
                        let mut evaluated_args: Vec<_> = Vec::with_capacity(arg_expressions.len());
                        for expression in arg_expressions {
                            evaluated_args.push(expression.run(closure_context))
                        }
                        function.run(closure_context, &evaluated_args)
                    }
                    _ => JsValue::type_error_rc(),
                }
            }
        }
    }
}
