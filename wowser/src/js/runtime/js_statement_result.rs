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

    pub fn string<S>(string: S) -> Self
    where
        S: Into<String>,
    {
        Self::Value(JsValue::String(string.into()))
    }

    pub fn is_nan(&self) -> bool {
        match self {
            JsStatementResult::Value(JsValue::Number(n)) => n.is_nan(),
            _ => false,
        }
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

    #[test]
    fn test_string() {
        assert_eq!(
            JsStatementResult::string("123"),
            JsStatementResult::Value(JsValue::String(String::from("123")))
        );
        assert_eq!(
            JsStatementResult::string(String::from("abc")),
            JsStatementResult::Value(JsValue::String(String::from("abc")))
        );
    }

    #[test]
    fn test_is_nan() {
        assert!(JsStatementResult::NAN.is_nan());

        assert!(!JsStatementResult::string("123").is_nan());
        assert!(!JsStatementResult::number(1.0).is_nan());
        assert!(!JsStatementResult::UNDEFINED.is_nan());
    }
}
