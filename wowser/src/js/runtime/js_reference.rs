use super::JsValue;

#[derive(Debug, Clone)]
pub struct JsReference {
    pub name: String,
    pub value: JsValue,
}
