/// Represents any type
#[derive(Debug, PartialEq, Clone)]
pub enum JsValue {
    Number(f64),
    Undefined,
}

impl JsValue {
    pub const NAN: Self = Self::Number(f64::NAN);
}

#[cfg(test)]
mod tests {
    use super::JsValue;

    #[test]
    fn test_nan() {
        match JsValue::NAN {
            JsValue::Number(v) => assert!(v.is_nan()),
            v => panic!("Invalid value: {:?}", v),
        }
    }
}
