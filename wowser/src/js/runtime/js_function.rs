use std::rc::Rc;

use super::JsValue;

#[derive(Clone)]
pub struct JsFunctionImplementation {
    pub func: Rc<dyn Fn(&[JsValue]) -> JsValue>,
}

impl Default for JsFunctionImplementation {
    fn default() -> Self {
        Self {
            func: Rc::new(|_| JsValue::Undefined),
        }
    }
}

impl std::fmt::Debug for JsFunctionImplementation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsFunctionImplementation")
            .field("func", &"[Native]".to_string())
            .finish()
    }
}

impl std::cmp::PartialEq for JsFunctionImplementation {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

// Functions cannot be compared and are always considered unequal
#[derive(Debug, PartialEq, Clone)]
pub enum JsFunction {
    Native(String, JsFunctionImplementation),
}

impl JsFunction {
    pub fn get_name(&self) -> &str {
        match self {
            Self::Native(name, _) => name,
        }
    }

    pub fn run(&self, args: &[JsValue]) -> JsValue {
        match self {
            Self::Native(_, implementation) => implementation.func.as_ref()(args),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_name() {
        assert_eq!(
            "abc",
            JsFunction::Native("abc".to_string(), Default::default()).get_name()
        );
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn test_partial_eq() {
        assert_ne!(
            JsFunctionImplementation::default(),
            JsFunctionImplementation::default(),
        );

        let a = JsFunctionImplementation::default();
        assert_ne!(a, a);
    }
}
