use super::{JsClosure, JsValue};

#[derive(Debug)]
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
    pub fn run(&self, global_references: &mut JsClosure) -> JsValue {
        match self {
            Self::Reference(variable_name) => global_references
                .get_reference_mut(variable_name)
                .map(|reference| reference.value.clone())
                // TODO: This should raise a ReferenceError instead when they exist
                .unwrap_or(JsValue::Undefined),
            Self::Number(num) => JsValue::Number(*num),
            Self::String(num) => JsValue::String(num.clone()),
            Self::Add(a, b) => {
                let a_value = a.run(global_references);
                let b_value = b.run(global_references);
                match (a_value, b_value) {
                    (JsValue::Number(num_a), JsValue::Number(num_b)) => {
                        JsValue::Number(num_a + num_b)
                    }
                    (JsValue::Number(_), JsValue::Undefined) => JsValue::NAN,
                    (JsValue::Undefined, JsValue::Number(_)) => JsValue::NAN,
                    (JsValue::Undefined, JsValue::Undefined) => JsValue::NAN,
                    (a @ JsValue::String(_), b) => JsValue::String(a.to_string() + &b.to_string()),
                    (a, b @ JsValue::String(_)) => JsValue::String(a.to_string() + &b.to_string()),
                    (a @ JsValue::Function(_), b) => {
                        JsValue::String(a.to_string() + &b.to_string())
                    }
                    (a, b @ JsValue::Function(_)) => {
                        JsValue::String(a.to_string() + &b.to_string())
                    }
                }
            }
            Self::Multiply(a, b) => {
                let a_value = a.run(global_references);
                let b_value = b.run(global_references);
                JsValue::Number(f64::from(a_value) * f64::from(b_value))
            }
            Self::CastToNumber(expression) => {
                let value = expression.run(global_references);
                JsValue::Number(f64::from(value))
            }
            Self::InvokeFunction(reference_to_invoke, args) => {
                let value = reference_to_invoke.run(global_references);
                match value {
                    JsValue::Function(function) => {
                        let evaluated_args: Vec<JsValue> =
                            args.iter().map(|arg| arg.run(global_references)).collect();
                        function.run(&evaluated_args)
                    }
                    _ => JsValue::Undefined, // TODO: This should be a TypeError when they exist
                }
            }
        }
    }
}
