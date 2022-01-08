use super::JsValue;

/// Represents the resulting value of evaluating a statement
#[derive(Debug, PartialEq)]
pub enum JsStatementResult {
    Value(JsValue),
    Void,
}

impl JsStatementResult {
    pub const NAN: Self = Self::Value(JsValue::NAN);
    pub const UNDEFINED: Self = Self::Value(JsValue::Undefined);

    pub fn number(v: f64) -> Self {
        Self::Value(JsValue::Number(v))
    }
}

#[cfg(test)]
mod tests {
    use super::JsStatementResult;
    use crate::js::JsValue;

    #[test]
    fn test_nan() {
        match JsStatementResult::NAN {
            JsStatementResult::Value(JsValue::Number(v)) => assert!(v.is_nan()),
            v => panic!("Invalid result value: {:?}", v),
        }
    }

    #[test]
    fn test_undefined() {
        match JsStatementResult::UNDEFINED {
            JsStatementResult::Value(JsValue::Undefined) => {}
            v => panic!("Invalid result value: {:?}", v),
        }
    }

    #[test]
    fn test_number() {
        match JsStatementResult::number(123.0) {
            JsStatementResult::Value(JsValue::Number(v)) => assert_eq!(v, 123.0),
            v => panic!("Invalid result value: {:?}", v),
        }
    }
}
