use super::super::dns::resolve_domain_name_to_ip;
use super::super::stream::AsyncTcpStream;
use crate::{
    net::{Url, UrlHost},
    util::{vec_contains, vec_window_split, StringError},
};
use core::result;
use futures_util::stream::StreamExt;
use std::io::Write;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::{error::Error, fmt::Display};

pub type Result<T> = core::result::Result<T, HttpRequestError>;

static SINGLE_NEWLINE_BYTES: &[u8] = b"\r\n";
static DOUBLE_NEWLINE_BYTES: &[u8] = b"\r\n\r\n";

enum HttpVerb {
    GET,
    HEAD,
}

/// Represents errors that occur when making an HTTP request
#[derive(Debug)]
pub struct HttpRequestError {
    err: Box<dyn Error>,
}

impl Error for HttpRequestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.err.as_ref())
    }
}

impl Display for HttpRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP Request error: {}", self.err)
    }
}

fn contains_end_of_headers(vec: &[u8]) -> bool {
    vec_contains(vec, DOUBLE_NEWLINE_BYTES)
}

fn parse_headers(vec: &[u8]) -> Option<(HttpStatus, Vec<HttpHeader>, &[u8])> {
    let headers = vec_window_split(vec, SINGLE_NEWLINE_BYTES);
    let first_line_bytes = headers.get(0)?;
    let first_line = std::str::from_utf8(first_line_bytes).ok()?;
    let mut parts = first_line.splitn(3, ' ');
    let http_version = parts.next()?.to_owned();
    let status_code = u16::from_str_radix(parts.next()?, 10).ok()?;
    let reason_phrase = parts.next()?.to_owned();

    let status = HttpStatus {
        http_version,
        status_code,
        reason_phrase,
    };

    let mut offset = first_line_bytes.len() + 2;
    let headers = headers[1..]
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|vec| {
            offset += vec.len() + 2;
            std::str::from_utf8(vec).ok()
        })
        .map(|line| -> Option<HttpHeader> {
            let mut values = line?.splitn(2, ':');
            let name = values.next()?.to_owned();
            let value = values.next()?.to_owned();
            Some(HttpHeader { name, value })
        });

    let mut ret_headers = vec![];
    for header in headers {
        ret_headers.push(header?);
    }

    Some((status, ret_headers, &vec[offset..]))
}
/// Holds the state of an HTTP request
pub struct HttpRequest {
    url: Url,
}

impl HttpRequest {
    /// Creates a new HTTP request
    pub fn new(url: Url) -> HttpRequest {
        HttpRequest { url }
    }

    /// Performs a GET request
    pub async fn get(&mut self) -> Result<HttpResponse> {
        self.make_request(&self.url, &HttpVerb::GET).await
    }

    pub async fn head(&mut self) -> Result<HttpResponse> {
        self.make_request(&self.url, &HttpVerb::HEAD).await
    }

    async fn make_request(&self, url: &Url, verb: &HttpVerb) -> Result<HttpResponse> {
        let (host, ip) = self.get_ip_address(&url).await?;

        let stream = self
            .get_tcp(host, &ip, &verb)
            .await
            .map_err(|e| HttpRequestError { err: Box::new(e) })?;

        self.read_full_response(stream)
            .await
            .map_err(|e| HttpRequestError { err: Box::new(e) })
    }

    async fn get_ip_address(&self, url: &Url) -> Result<(String, IpAddr)> {
        match &(url.host) {
            UrlHost::IP(ip) => Ok((ip.to_string(), *ip)),
            UrlHost::DomainName(domain) => {
                let ip =
                    resolve_domain_name_to_ip(domain.as_str()).map_err(|e| HttpRequestError {
                        err: Box::new(StringError::from(e)),
                    })?;
                Ok((domain.to_string(), IpAddr::V4(ip)))
            }
        }
    }

    async fn get_tcp(
        &self,
        host: String,
        ip: &IpAddr,
        verb: &HttpVerb,
    ) -> result::Result<AsyncTcpStream, std::io::Error> {
        let verb_str = match verb {
            HttpVerb::GET => "GET",
            HttpVerb::HEAD => "HEAD",
        };

        let mut stream = TcpStream::connect(SocketAddr::new(*ip, self.url.port))?;
        let request = format!(
            "{} {} HTTP/1.1\r\nHost: {}\r\n\r\n",
            verb_str,
            self.url.http_request_path(),
            host
        );
        stream.write_all(request.as_bytes())?;
        Ok(AsyncTcpStream::from_tcp_stream(stream))
    }

    async fn read_full_response(
        &self,
        mut stream: AsyncTcpStream,
    ) -> result::Result<HttpResponse, std::io::Error> {
        let mut result = vec![];
        let mut status: HttpStatus = Default::default();
        let mut headers: Vec<HttpHeader> = vec![];
        while let Some(bytes) = stream.next().await {
            result.extend(&bytes?);
            if contains_end_of_headers(&result) {
                let parsed = parse_headers(&result);
                println!("Response Headers: {:?}", parsed);
                if let Some((parsed_status, parsed_headers, _remainer)) = parsed {
                    status = parsed_status;
                    headers = parsed_headers;
                }
            }
        }

        Ok(HttpResponse {
            status,
            headers,
            body: vec![],
        })
    }
}

#[derive(Debug, Default)]
pub struct HttpStatus {
    http_version: String,
    status_code: u16,
    reason_phrase: String,
}

#[derive(Debug)]
pub struct HttpResponse {
    status: HttpStatus,
    headers: Vec<HttpHeader>,
    body: Vec<u8>,
}

#[derive(Debug)]
pub struct HttpHeader {
    name: String,
    value: String,
}
