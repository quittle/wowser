use std::rc::Rc;

use super::JsFunction;

/// Represents any type
#[derive(Debug, PartialEq)]
pub enum JsValue {
    Number(f64),
    String(String),
    Function(JsFunction),
    Undefined,
}

impl JsValue {
    pub const NAN: Self = Self::Number(f64::NAN);

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

    pub fn number_rc(value: f64) -> Rc<Self> {
        Rc::new(Self::Number(value))
    }

    pub fn undefined_rc() -> Rc<Self> {
        Rc::new(Self::Undefined)
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
}

impl ToString for JsValue {
    fn to_string(&self) -> String {
        match self {
            Self::Number(v) => v.to_string(),
            Self::String(v) => v.clone(),
            Self::Undefined => "undefined".to_string(),
            Self::Function(function) => {
                format!("function {}() {{ [native code] }}", function.get_name())
            }
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
            JsValue::Function(_) => f64::NAN,
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
        assert_eq!(
            JsValue::Function(JsFunction::Native("abc".to_string(), Default::default()))
                .to_string(),
            "function abc() { [native code] }"
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

        assert!(f64::from(JsValue::Function(JsFunction::Native(
            "abc".to_string(),
            Default::default()
        )))
        .is_nan());
    }
}
