use std::{num::NonZeroUsize, rc::Weak};

use super::NETWORK_EXECUTOR;
use crate::util::{Executor, LRUCache, TaskToken};

use super::{HttpRequest, HttpRequestError, HttpResult, Url};

const NETWORK_REQUEST_CACHE_CAPACITY: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(100) };

#[allow(clippy::type_complexity)]
pub struct NetworkResourceManager {
    url_token_cache: LRUCache<Url, TaskToken, Box<dyn Fn(&Url) -> Option<TaskToken>>>,
    token_result_cache:
        LRUCache<TaskToken, HttpResult, Box<dyn Fn(&TaskToken) -> Option<HttpResult>>>,
}

unsafe impl Send for NetworkResourceManager {}

impl Default for NetworkResourceManager {
    fn default() -> Self {
        Self {
            url_token_cache: LRUCache::new(
                NETWORK_REQUEST_CACHE_CAPACITY,
                Box::new(Self::initiate_network_request),
            ),
            token_result_cache: LRUCache::new(
                NETWORK_REQUEST_CACHE_CAPACITY,
                Box::new(Self::read_request_response),
            ),
        }
    }
}

impl NetworkResourceManager {
    pub fn get_or_request(&mut self, url: &Url) -> Option<Weak<HttpResult>> {
        let token_weak = self.url_token_cache.get(url)?;
        let token = *token_weak.upgrade()?;
        self.token_result_cache.get(&token)
    }

    fn initiate_network_request(url: &Url) -> Option<TaskToken> {
        let mut executor = NETWORK_EXECUTOR.lock().ok()?;
        let token = executor.run(HttpRequest::new(url.clone()).get()).ok()?;
        Some(token)
    }

    fn read_request_response(token: &TaskToken) -> Option<HttpResult> {
        let executor = NETWORK_EXECUTOR.lock().ok()?;
        let result = executor.get_result(*token)?;
        Some(match result {
            Ok(http_result) => http_result,
            Err(err) => Err(HttpRequestError::from(Box::new(err))),
        })
    }
}
