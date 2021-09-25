use super::super::dns::resolve_domain_name_to_ip;
use super::super::stream::AsyncTcpStream;
use super::constants::DOUBLE_NEWLINE_BYTES;
use super::http_header_map::HttpHeaderMap;
use super::{structures::HttpVerb, HttpRequestError, HttpResponse, HttpResult, HttpStatus, Result};
use crate::net::http::headers::parse_status_headers;
use crate::{
    net::{Url, UrlHost},
    util::{vec_contains, StringError},
};
use core::result;
use futures::Future;
use futures_util::stream::StreamExt;
use std::io::Write;
use std::net::{IpAddr, SocketAddr, TcpStream};

fn contains_end_of_headers(vec: &[u8]) -> bool {
    vec_contains(vec, DOUBLE_NEWLINE_BYTES)
}

fn determine_content_length(
    verb: &HttpVerb,
    status: &HttpStatus,
    header_map: &HttpHeaderMap,
) -> Result<u32> {
    if *verb == HttpVerb::Head {
        return Ok(0);
    }

    match status.status_code {
        100..=199 | 204 | 304 => return Ok(0),
        _ => (),
    };
    header_map
        .get("content-length")
        .map(|length| {
            length
                .trim()
                .parse::<u32>()
                .map_err(|e| HttpRequestError::from(Box::new(e)))
        })
        .unwrap_or(Ok(u32::MAX))
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
    pub fn get(&self) -> impl Future<Output = HttpResult> {
        Self::make_request(self.url.clone(), HttpVerb::Get)
    }

    /// Performs a HEAD request
    pub fn head(&self) -> impl Future<Output = HttpResult> {
        Self::make_request(self.url.clone(), HttpVerb::Head)
    }

    async fn make_request(url: Url, verb: HttpVerb) -> HttpResult {
        let (host, ip) = Self::get_ip_address(&url).await?;

        let stream = Self::get_tcp(host, &ip, url.port, url.http_request_path().as_str(), &verb)
            .map_err(|e| HttpRequestError::from(Box::new(e)))?;

        Self::read_full_response(verb, stream).await
    }

    async fn get_ip_address(url: &Url) -> Result<(String, IpAddr)> {
        match &(url.host) {
            UrlHost::IP(ip) => Ok((ip.to_string(), *ip)),
            UrlHost::DomainName(domain) => {
                let ip = resolve_domain_name_to_ip(domain.as_str())
                    .map_err(|e| HttpRequestError::from(Box::new(StringError::from(e))))?;
                Ok((domain.to_string(), IpAddr::V4(ip)))
            }
        }
    }

    fn get_tcp(
        host: String,
        ip: &IpAddr,
        port: u16,
        path: &str,
        verb: &HttpVerb,
    ) -> result::Result<AsyncTcpStream, std::io::Error> {
        let verb_str = match verb {
            HttpVerb::Get => "GET",
            HttpVerb::Head => "HEAD",
        };

        let mut stream = TcpStream::connect(SocketAddr::new(*ip, port))?;
        let request = format!(
            "{verb} {path} HTTP/1.1\r\nHost: {domain}\r\n\r\n",
            verb = verb_str,
            path = path,
            domain = host
        );
        stream.write_all(request.as_bytes())?;
        Ok(AsyncTcpStream::from_tcp_stream(stream))
    }

    async fn read_full_response(
        verb: HttpVerb,
        mut stream: AsyncTcpStream,
    ) -> Result<HttpResponse> {
        let mut result = vec![];
        let mut status = HttpStatus::default();
        let mut header_map = HttpHeaderMap::default();
        let mut content_length = u32::MAX;
        let mut preamble_length = 0;
        while let Some(bytes) = stream.next().await {
            result.extend(&bytes.map_err(|e| HttpRequestError::from(Box::new(e)))?);
            if preamble_length == 0 && contains_end_of_headers(&result) {
                let parsed_status_headers = parse_status_headers(&result);
                if let Some((parsed_status, parsed_headers, remainder)) = parsed_status_headers {
                    status = parsed_status;
                    header_map = HttpHeaderMap::from(parsed_headers);
                    preamble_length = result.len() - remainder.len();
                    content_length = determine_content_length(&verb, &status, &header_map)?;
                    result = remainder.to_vec();

                    if content_length == 0 {
                        break;
                    }
                }
            } else if result.len() >= content_length as usize {
                result.truncate(content_length as usize);
                break;
            }
        }

        Ok(HttpResponse {
            status,
            headers: header_map,
            body: result,
        })
    }
}
