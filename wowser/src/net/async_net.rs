use super::{HttpResult, NetworkResourceManager};
use crate::util::DedicatedThreadExecutor;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref NETWORK_EXECUTOR: Mutex<DedicatedThreadExecutor<HttpResult>> =
        Mutex::new(DedicatedThreadExecutor::default());
    pub static ref NETWORK_RESOURCE_MANAGER: Mutex<NetworkResourceManager> =
        Mutex::new(NetworkResourceManager::default());
}
