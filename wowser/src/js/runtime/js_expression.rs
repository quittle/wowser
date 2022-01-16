use std::rc::Rc;

use super::{JsClosure, JsValue};

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
    pub fn run(&self, global_references: &mut JsClosure) -> Rc<JsValue> {
        match self {
            Self::Reference(variable_name) => global_references
                .get_reference_mut(variable_name)
                .map(|reference| reference.value.clone())
                // TODO: This should raise a ReferenceError instead when they exist
                .unwrap_or_else(JsValue::undefined_rc),
            Self::Number(num) => JsValue::number_rc(*num),
            Self::String(num) => JsValue::str_rc(num),
            Self::Add(a, b) => {
                let a_value = a.run(global_references);
                let b_value = b.run(global_references);
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
                let a_value = a.run(global_references);
                let b_value = b.run(global_references);
                JsValue::number_rc(f64::from(a_value.as_ref()) * f64::from(b_value.as_ref()))
            }
            Self::CastToNumber(expression) => {
                let value = expression.run(global_references);
                JsValue::number_rc(f64::from(value.as_ref()))
            }
            Self::InvokeFunction(reference_to_invoke, args) => {
                let value = reference_to_invoke.run(global_references);
                match value.as_ref() {
                    JsValue::Function(function) => {
                        let evaluated_args: Vec<Rc<JsValue>> =
                            args.iter().map(|arg| arg.run(global_references)).collect();
                        function.run(&evaluated_args)
                    }
                    _ => JsValue::undefined_rc(), // TODO: This should be a TypeError when they exist
                }
            }
        }
    }
}
