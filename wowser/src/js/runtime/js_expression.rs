use super::JsValue;

#[derive(Debug)]
pub enum JsExpression {
    Number(f64),
    Add(Box<JsExpression>, Box<JsExpression>),
}

impl JsExpression {
    pub fn run(&self) -> JsValue {
        match self {
            Self::Number(num) => JsValue::Number(*num),
            Self::Add(a, b) => {
                let a_value = a.run();
                let b_value = b.run();
                match a_value {
                    JsValue::Number(num_a) => match b_value {
                        JsValue::Number(num_b) => JsValue::Number(num_a + num_b),
                    },
                }
            }
        }
    }
}
