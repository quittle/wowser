use super::HttpResult;
use crate::util::DedicatedThreadExecutor;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref NETWORK_EXECUTOR: Mutex<DedicatedThreadExecutor<HttpResult>> =
        Mutex::new(DedicatedThreadExecutor::default());
}
