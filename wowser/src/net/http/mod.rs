mod error;
mod http_request;
mod structures;

pub use error::{HttpRequestError, Result};
pub use http_request::HttpRequest;
pub use structures::{HttpHeader, HttpResponse, HttpResult, HttpStatus};
