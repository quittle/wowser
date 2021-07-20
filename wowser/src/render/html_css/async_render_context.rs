use std::rc::Rc;

use crate::net::{HttpResult, Url, NETWORK_RESOURCE_MANAGER};

/// Abstraction for managing async tasks when rendering
#[derive(Default)]
pub struct AsyncRenderContext {
    // TODO: Add support for cheaper tracking and checking if new resources became available
}

impl AsyncRenderContext {
    pub fn get_resource(&mut self, url: &Url) -> Option<Rc<HttpResult>> {
        NETWORK_RESOURCE_MANAGER
            .lock()
            .map(|mut network_resource_manager| network_resource_manager.get_or_request(&url))
            .ok()
            .flatten()
            .map(|result_weak| result_weak.upgrade())
            .flatten()
    }
}
