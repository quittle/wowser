use super::JsValueNode;

#[derive(Debug, Clone)]
pub struct JsReference {
    pub name: String,
    pub value: JsValueNode,
}

impl JsReference {
    pub fn get_referenced_nodes(&self) -> Vec<JsValueNode> {
        vec![self.value.clone()]
    }
}

impl PartialEq for JsReference {
    fn eq(&self, other: &Self) -> bool {
        self.value.is_same_ref(&other.value)
    }
}
