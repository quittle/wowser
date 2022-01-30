use std::{collections::HashMap, rc::Rc};

use super::JsFunction;

/// Represents any type
#[derive(Debug, PartialEq)]
pub enum JsValue {
    Boolean(bool),
    Number(f64),
    String(String),
    Function(JsFunction),
    Object(HashMap<String, Rc<JsValue>>),
    Undefined,
    Null,
}

impl JsValue {
    pub const NAN: Self = Self::Number(f64::NAN);

    pub fn bool_rc(b: bool) -> Rc<Self> {
        Rc::new(Self::Boolean(b))
    }

    pub fn nan_rc() -> Rc<Self> {
        Rc::new(Self::NAN)
    }

    pub fn str(s: &str) -> Self {
        JsValue::String(s.into())
    }

    pub fn str_rc(s: &str) -> Rc<Self> {
        Rc::new(Self::str(s))
    }

    pub fn string_rc(s: String) -> Rc<Self> {
        Rc::new(Self::String(s))
    }

    pub fn number_rc<F>(value: F) -> Rc<Self>
    where
        F: Into<f64>,
    {
        Rc::new(Self::Number(value.into()))
    }

    pub fn undefined_rc() -> Rc<Self> {
        Rc::new(Self::Undefined)
    }

    pub fn null_rc() -> Rc<Self> {
        Rc::new(Self::Null)
    }

    pub fn object_rc(map: HashMap<String, Rc<JsValue>>) -> Rc<Self> {
        Rc::new(Self::Object(map))
    }

    pub fn type_error_rc() -> Rc<Self> {
        Self::undefined_rc() // TODO: These should raise exceptions when supported
    }

    pub fn type_error_or_dom_exception_rc() -> Rc<Self> {
        Self::undefined_rc() // TODO: These should raise exceptions when supported
    }

    pub fn reference_error_rc() -> Rc<Self> {
        Self::undefined_rc() // TODO: These should raise exceptions when supported
    }

    pub fn stack_overflow_error_rc() -> Rc<Self> {
        Self::undefined_rc() // TODO: This should raise RangeError: Maximum call stack size exceeded when supported
    }
}

impl ToString for JsValue {
    fn to_string(&self) -> String {
        match self {
            Self::Boolean(b) => b.to_string(),
            Self::Number(v) => v.to_string(),
            Self::String(v) => v.clone(),
            Self::Undefined => "undefined".to_string(),
            Self::Null => "null".to_string(),
            Self::Function(function) => match function {
                JsFunction::Native(name, _implementation) => {
                    format!("function {name}() {{ [native code] }}")
                }
                JsFunction::UserDefined(source, _name, _args, _implementation) => {
                    source.to_string()
                }
            },
            Self::Object(_) => "[object Object]".to_string(),
        }
    }
}

impl From<JsValue> for f64 {
    fn from(value: JsValue) -> f64 {
        From::from(&value)
    }
}

impl From<&JsValue> for f64 {
    fn from(value: &JsValue) -> f64 {
        match value {
            JsValue::Boolean(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            JsValue::Number(v) => *v,
            JsValue::String(v) => {
                let trimmed = v.trim();
                // Strings with just whitespace convert to 0
                if trimmed.is_empty() {
                    0.0
                } else {
                    trimmed.parse::<f64>().unwrap_or(f64::NAN)
                }
            }
            JsValue::Undefined => f64::NAN,
            JsValue::Null => 0.0,
            JsValue::Function(_) => f64::NAN,
            JsValue::Object(_) => f64::NAN,
        }
    }
}

impl From<&JsValue> for bool {
    fn from(value: &JsValue) -> bool {
        match value {
            JsValue::Boolean(v) => *v,
            JsValue::Number(v) => !v.is_nan() && *v != 0.0,
            JsValue::String(v) => !v.is_empty(),
            JsValue::Undefined => false,
            JsValue::Null => false,
            JsValue::Function(_) => true,
            JsValue::Object(_) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nan() {
        match JsValue::NAN {
            JsValue::Number(v) => assert!(v.is_nan()),
            v => panic!("Invalid value: {:?}", v),
        }
    }

    #[test]
    fn test_to_string() {
        assert_eq!(JsValue::NAN.to_string(), "NaN");
        assert_eq!(JsValue::Number(1.0).to_string(), "1");
        assert_eq!(JsValue::Number(1.2).to_string(), "1.2");
        assert_eq!(JsValue::Number(-1.2).to_string(), "-1.2");
        assert_eq!(JsValue::str("").to_string(), "");
        assert_eq!(JsValue::str("abc").to_string(), "abc");
        assert_eq!(JsValue::Undefined.to_string(), "undefined");
        assert_eq!(JsValue::Null.to_string(), "null");
        assert_eq!(
            JsValue::Function(JsFunction::Native("abc".to_string(), Default::default()))
                .to_string(),
            "function abc() { [native code] }"
        );
        assert_eq!(
            JsValue::Function(JsFunction::UserDefined(
                "function abc(param) {return param;}".to_string(),
                Default::default(),
                Default::default(),
                Default::default()
            ))
            .to_string(),
            "function abc(param) {return param;}"
        );
        assert_eq!(
            JsValue::Object(HashMap::from([(
                "key".to_string(),
                JsValue::str_rc("value")
            )]))
            .to_string(),
            "[object Object]"
        );
    }

    #[test]
    fn test_from_jsvalue_to_f64() {
        assert!(f64::from(JsValue::NAN).is_nan());
        assert_eq!(f64::from(JsValue::Number(1.0)), 1.0);
        assert_eq!(f64::from(JsValue::Number(1.2)), 1.2);
        assert_eq!(f64::from(JsValue::Number(-1.2)), -1.2);

        assert_eq!(f64::from(JsValue::str("")), 0.0);
        assert!(f64::from(JsValue::str("abc")).is_nan());
        assert_eq!(f64::from(JsValue::str("1")), (1.0));
        assert_eq!(f64::from(JsValue::str("-1")), (-1.0));
        assert_eq!(f64::from(JsValue::str("-1.2")), (-1.2));

        assert!(f64::from(JsValue::Undefined).is_nan());
        assert_eq!(f64::from(JsValue::Null), 0.0);

        assert!(f64::from(JsValue::Function(JsFunction::Native(
            "abc".to_string(),
            Default::default()
        )))
        .is_nan());
        assert!(f64::from(JsValue::Object(HashMap::new())).is_nan());
    }
}
