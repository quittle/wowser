mod dns;
mod http;
mod url;

pub use dns::{build_resolve_bytes, resolve_domain_name_to_ip};
pub use http::HttpRequest;
pub use url::{Url, UrlHost, UrlProtocol};
