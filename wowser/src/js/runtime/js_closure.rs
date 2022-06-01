use crate::util::mut_vec_find_or_insert;

use super::{JsReference, JsValue, JsValueGraph};

#[derive(Debug)]
pub struct JsClosure {
    pub references: Vec<JsReference>,
    pub node_graph: JsValueGraph,
}

impl JsClosure {
    pub fn new(node_graph: &JsValueGraph) -> Self {
        Self {
            references: vec![],
            node_graph: node_graph.clone(),
        }
    }

    pub fn has_reference(&self, variable_name: &str) -> bool {
        self.references
            .iter()
            .any(|reference| reference.name == variable_name)
    }

    pub fn get_reference(&self, variable_name: &str) -> Option<&JsReference> {
        self.references
            .iter()
            .find(|reference| reference.name == variable_name)
    }

    pub fn get_reference_mut(&mut self, variable_name: &str) -> Option<&mut JsReference> {
        self.references
            .iter_mut()
            .find(|reference| reference.name == variable_name)
    }

    pub fn get_or_declare_reference_mut(&mut self, variable_name: &str) -> &mut JsReference {
        mut_vec_find_or_insert(
            &mut self.references,
            |reference| reference.name == variable_name,
            || JsReference {
                name: variable_name.into(),
                value: JsValue::undefined_rc(&self.node_graph),
            },
        );
        self.references.last_mut().unwrap()
    }
}
