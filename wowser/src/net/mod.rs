mod dns;
mod http;
mod stream;
mod url;

pub const NETWORK_BUFFER_SIZE: usize = 512;

pub use dns::{build_resolve_bytes, resolve_domain_name_to_ip};
pub use http::{HttpHeader, HttpRequest, HttpResponse, HttpStatus, Result};
pub use stream::AsyncTcpStream;
pub use url::{Url, UrlHost, UrlProtocol};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn head_example_com() {
        let mut request = HttpRequest::new(Url::new(
            UrlProtocol::HTTP,
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
        let mut request = HttpRequest::new(Url::new(
            UrlProtocol::HTTP,
            UrlHost::DomainName("example.com".to_string()),
            80,
            "",
            "",
            "",
        ));
        let response = futures::executor::block_on(request.get()).expect("request failed");

        assert!(response.headers.contains(&HttpHeader {
            name: "Vary".to_string(),
            value: " Accept-Encoding".to_string()
        }));

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

        assert!(response
            .headers
            .iter()
            .find(|header| header.name == "Date")
            .is_some());
        assert!(response
            .headers
            .iter()
            .find(|header| header.name == "Expires")
            .is_some());
        assert!(response
            .headers
            .iter()
            .find(|header| header.name == "Etag")
            .is_some());
        assert!(response
            .headers
            .iter()
            .find(|header| header.name == "Cache-Control")
            .is_some());
        let content_length = response
            .headers
            .iter()
            .find(|header| header.name == "Content-Length")
            .expect("Content-Length required");

        usize::from_str_radix(&content_length.value.trim(), 10)
            .expect(format!("Invalid content length: {}", content_length.value).as_str())
    }
}
