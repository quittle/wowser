use crate::net;

use super::http_header_map::HttpHeaderMap;

#[derive(Debug, Default, PartialEq)]
pub struct HttpStatus {
    pub http_version: String,
    pub status_code: u16,
    pub reason_phrase: String,
}

impl HttpStatus {
    pub fn contains_success_content(&self) -> bool {
        self.status_code == 200
    }
}

#[derive(Debug, PartialEq)]
pub struct HttpResponse {
    pub status: HttpStatus,
    pub headers: HttpHeaderMap,
    pub body: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

impl HttpHeader {
    pub fn new(name: &str, value: &str) -> HttpHeader {
        HttpHeader {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(PartialEq)]
pub enum HttpVerb {
    Get,
    Head,
}

pub type HttpResult = net::Result<HttpResponse>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_header_new() {
        let http_header = HttpHeader::new("abc", "123");
        assert_eq!(http_header.name, "abc");
        assert_eq!(http_header.value, "123");
    }
}
