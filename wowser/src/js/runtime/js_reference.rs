use std::rc::Rc;

use super::JsValue;

#[derive(Debug, Clone)]
pub struct JsReference {
    pub name: String,
    pub value: Rc<JsValue>,
}

impl PartialEq for JsReference {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.value, &other.value)
    }
}
