use super::{JsClosure, JsValue};

#[derive(Debug)]
pub enum JsExpression {
    Number(f64),
    Add(Box<JsExpression>, Box<JsExpression>),
    Multiply(Box<JsExpression>, Box<JsExpression>),
    Reference(String),
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
                }
            }
            Self::Multiply(a, b) => {
                let a_value = a.run(global_references);
                let b_value = b.run(global_references);
                match (a_value, b_value) {
                    (JsValue::Number(num_a), JsValue::Number(num_b)) => {
                        JsValue::Number(num_a * num_b)
                    }
                    (JsValue::Number(_), JsValue::Undefined) => JsValue::NAN,
                    (JsValue::Undefined, JsValue::Number(_)) => JsValue::NAN,
                    (JsValue::Undefined, JsValue::Undefined) => JsValue::NAN,
                }
            }
        }
    }
}
