use std::{collections::HashMap, rc::Rc};

use crate::{
    css::CssDocument,
    net::{HttpResult, Url, NETWORK_RESOURCE_MANAGER},
    parse::ParsedTokenOffset,
};

/// Abstraction for managing async tasks when rendering
#[derive(Default)]
pub struct AsyncRenderContext {
    pub css_documents: HashMap<String, (ParsedTokenOffset, CssDocument)>,
}

impl AsyncRenderContext {
    pub fn get_resource(&mut self, url: &Url) -> Option<Rc<HttpResult>> {
        NETWORK_RESOURCE_MANAGER
            .lock()
            .map(|mut network_resource_manager| network_resource_manager.get_or_request(url))
            .ok()
            .flatten()
            .map(|result_weak| result_weak.upgrade())
            .flatten()
    }
}
