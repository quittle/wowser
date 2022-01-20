use std::rc::Rc;

use super::{JsClosureContext, JsValue};

#[derive(Debug, PartialEq)]
pub enum JsExpression {
    Number(f64),
    String(String),
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
            Self::Number(num) => JsValue::number_rc(*num),
            Self::String(num) => JsValue::str_rc(num),
            Self::Add(a, b) => {
                let a_value = a.run(closure_context);
                let b_value = b.run(closure_context);
                match (a_value.as_ref(), b_value.as_ref()) {
                    (JsValue::Number(num_a), JsValue::Number(num_b)) => {
                        JsValue::number_rc(num_a + num_b)
                    }
                    (JsValue::Number(_), JsValue::Undefined) => JsValue::nan_rc(),
                    (JsValue::Undefined, JsValue::Number(_)) => JsValue::nan_rc(),
                    (JsValue::Undefined, JsValue::Undefined) => JsValue::nan_rc(),
                    (a @ JsValue::String(_), b) => {
                        JsValue::string_rc(a.to_string() + &b.to_string())
                    }
                    (a, b @ JsValue::String(_)) => {
                        JsValue::string_rc(a.to_string() + &b.to_string())
                    }
                    (a @ JsValue::Function(_), b) => {
                        JsValue::string_rc(a.to_string() + &b.to_string())
                    }
                    (a, b @ JsValue::Function(_)) => {
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
