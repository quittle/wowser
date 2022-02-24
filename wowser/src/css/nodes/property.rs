use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct CssProperty {
    pub key: String,
    pub value: String,
}

impl CssProperty {
    pub fn new(key: &str, value: &str) -> CssProperty {
        CssProperty {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn new_rc(key: &str, value: &str) -> Rc<CssProperty> {
        Rc::new(Self::new(key, value))
    }
}
