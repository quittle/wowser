pub mod async_net;
mod dns;
mod http;
mod network_resource_manager;
mod stream;
mod url;

pub const NETWORK_BUFFER_SIZE: usize = 512;

pub use async_net::*;
pub use dns::{build_resolve_bytes, resolve_domain_name_to_ip};
pub use http::{
    HttpHeader, HttpHeaderMap, HttpRequest, HttpRequestError, HttpResponse, HttpResult, HttpStatus,
    Result,
};
pub use network_resource_manager::*;
pub use stream::AsyncTcpStream;
pub use url::{Url, UrlHost, UrlProtocol};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn head_example_com() {
        let request = HttpRequest::new(Url::new(
            UrlProtocol::Http,
            UrlHost::DomainName("example.com".to_string()),
            80,
            "",
            "",
            "",
        ));
        let response = futures::executor::block_on(request.head()).expect("request failed");
        let content_length = assert_example_com_prelude_ret_content_length(&response);
        assert!(content_length > 100);
        assert_eq!(Vec::new() as Vec<u8>, response.body);
    }

    #[test]
    pub fn get_example_com() {
        let request = HttpRequest::new(Url::new(
            UrlProtocol::Http,
            UrlHost::DomainName("example.com".to_string()),
            80,
            "",
            "",
            "",
        ));
        let response = futures::executor::block_on(request.get()).expect("request failed");

        assert_eq!(response.headers.get("vary"), Some("Accept-Encoding".into()));

        let content_length = assert_example_com_prelude_ret_content_length(&response);
        assert_eq!(content_length, response.body.len());
        assert!(response.body.len() > 100);
        let body = std::str::from_utf8(&response.body).expect("Invalid reponse body");
        assert!(body.contains("<h1>Example Domain</h1>"));
        assert_eq!(include_str!("test_data/example_com.html"), body);
    }

    fn assert_example_com_prelude_ret_content_length(response: &HttpResponse) -> usize {
        assert_eq!(
            response.status,
            HttpStatus {
                http_version: "HTTP/1.1".to_string(),
                status_code: 200,
                reason_phrase: "OK".to_string(),
            }
        );

        assert!(response.headers.get("date").is_some());
        assert!(response.headers.get("expires").is_some());
        assert!(response.headers.get("etag").is_some());
        assert!(response.headers.get("cache-control").is_some());
        let content_length = response
            .headers
            .get("content-length")
            .expect("Content-Length required");

        content_length
            .trim()
            .parse::<usize>()
            .unwrap_or_else(|_| panic!("Invalid content length: {}", content_length))
    }
}
