#[derive(Debug, Default, PartialEq)]
pub struct HttpStatus {
    pub http_version: String,
    pub status_code: u16,
    pub reason_phrase: String,
}

#[derive(Debug, PartialEq)]
pub struct HttpResponse {
    pub status: HttpStatus,
    pub headers: Vec<HttpHeader>,
    pub body: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

#[derive(PartialEq)]
pub enum HttpVerb {
    Get,
    Head,
}
