use crate::util::mut_vec_find_or_insert;

use super::{JsReference, JsValue};

#[derive(Debug, Default)]
pub struct JsClosure {
    pub references: Vec<JsReference>,
}

impl JsClosure {
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
                value: JsValue::undefined_rc(),
            },
        );
        self.references.last_mut().unwrap()
    }
}
