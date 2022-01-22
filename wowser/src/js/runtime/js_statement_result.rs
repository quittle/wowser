use std::rc::Rc;

use super::JsValue;

/// Represents the resulting value of evaluating a statement
#[derive(Debug, PartialEq)]
pub enum JsStatementResult {
    Value(Rc<JsValue>),
    ReturnValue(Rc<JsValue>),
    Void,
}

impl JsStatementResult {
    pub fn bool(b: bool) -> Self {
        Self::Value(JsValue::bool_rc(b))
    }

    pub fn number<F>(v: F) -> Self
    where
        F: Into<f64>,
    {
        Self::Value(JsValue::number_rc(v.into()))
    }

    pub fn nan() -> Self {
        Self::Value(JsValue::nan_rc())
    }

    pub fn string<S>(string: S) -> Self
    where
        S: Into<String>,
    {
        Self::Value(JsValue::string_rc(string.into()))
    }

    pub fn undefined() -> Self {
        Self::Value(JsValue::undefined_rc())
    }

    pub fn null() -> Self {
        Self::Value(JsValue::null_rc())
    }

    pub fn is_nan(&self) -> bool {
        match self {
            JsStatementResult::Value(value) => match value.as_ref() {
                JsValue::Number(n) => n.is_nan(),
                _ => false,
            },
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::JsStatementResult;
    use crate::js::JsValue;

    #[test]
    fn test_nan() {
        match JsStatementResult::nan() {
            JsStatementResult::Value(v) => match v.as_ref() {
                JsValue::Number(n) => assert!(n.is_nan()),
                v => panic!("Invalid value type: {:?}", v),
            },
            v => panic!("Invalid result value: {:?}", v),
        }
    }

    #[test]
    fn test_undefined() {
        match JsStatementResult::undefined() {
            JsStatementResult::Value(v) => match v.as_ref() {
                JsValue::Undefined => {}
                v => panic!("Invalid value type: {:?}", v),
            },
            v => panic!("Invalid result value: {:?}", v),
        }
    }

    #[test]
    fn test_number() {
        match JsStatementResult::number(123.0) {
            JsStatementResult::Value(v) => match v.as_ref() {
                JsValue::Number(v) => assert_eq!(*v, 123.0),
                v => panic!("Invalid value type: {:?}", v),
            },
            v => panic!("Invalid result value: {:?}", v),
        }
    }

    #[test]
    fn test_string() {
        assert_eq!(
            JsStatementResult::string("123"),
            JsStatementResult::Value(Rc::new(JsValue::String(String::from("123"))))
        );
        assert_eq!(
            JsStatementResult::string(String::from("abc")),
            JsStatementResult::Value(Rc::new(JsValue::String(String::from("abc"))))
        );
    }

    #[test]
    fn test_is_nan() {
        assert!(JsStatementResult::nan().is_nan());

        assert!(!JsStatementResult::string("123").is_nan());
        assert!(!JsStatementResult::number(1.0).is_nan());
        assert!(!JsStatementResult::undefined().is_nan());
    }
}
