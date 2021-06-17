use super::super::dns::resolve_domain_name_to_ip;
use super::super::stream::AsyncTcpStream;
use super::{structures::HttpVerb, HttpHeader, HttpRequestError, HttpResponse, HttpStatus, Result};
use crate::{
    net::{Url, UrlHost},
    util::{vec_contains, vec_window_split, StringError},
};
use core::result;
use futures_util::stream::StreamExt;
use std::io::Write;
use std::net::{IpAddr, SocketAddr, TcpStream};

const SINGLE_NEWLINE_BYTES: &[u8] = b"\r\n";
const DOUBLE_NEWLINE_BYTES: &[u8] = b"\r\n\r\n";

fn contains_end_of_headers(vec: &[u8]) -> bool {
    vec_contains(vec, DOUBLE_NEWLINE_BYTES)
}

fn parse_headers(vec: &[u8]) -> Option<(HttpStatus, Vec<HttpHeader>, &[u8])> {
    let headers = vec_window_split(vec, SINGLE_NEWLINE_BYTES);
    let first_line_bytes = headers.get(0)?;
    let first_line = std::str::from_utf8(first_line_bytes).ok()?;
    let mut parts = first_line.splitn(3, ' ');
    let http_version = parts.next()?.to_owned();
    let status_code = parts.next()?.parse::<u16>().ok()?;
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

    offset += SINGLE_NEWLINE_BYTES.len();

    Some((status, ret_headers, &vec[offset..]))
}

fn determine_content_length(
    verb: &HttpVerb,
    status: &HttpStatus,
    headers: &[HttpHeader],
) -> Result<u32> {
    if *verb == HttpVerb::Head {
        return Ok(0);
    }

    match status.status_code {
        100..=199 | 204 | 304 => return Ok(0),
        _ => (),
    };

    for header in headers {
        if header.name == "Content-Length" {
            return header
                .value
                .trim()
                .parse::<u32>()
                .map_err(|e| HttpRequestError::from(Box::new(e)));
        }
    }

    Ok(u32::MAX)
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
        self.make_request(&self.url, &HttpVerb::Get).await
    }

    pub async fn head(&mut self) -> Result<HttpResponse> {
        self.make_request(&self.url, &HttpVerb::Head).await
    }

    async fn make_request(&self, url: &Url, verb: &HttpVerb) -> Result<HttpResponse> {
        let (host, ip) = self.get_ip_address(&url).await?;

        let stream = self
            .get_tcp(host, &ip, &verb)
            .await
            .map_err(|e| HttpRequestError::from(Box::new(e)))?;

        self.read_full_response(verb, stream).await
    }

    async fn get_ip_address(&self, url: &Url) -> Result<(String, IpAddr)> {
        match &(url.host) {
            UrlHost::IP(ip) => Ok((ip.to_string(), *ip)),
            UrlHost::DomainName(domain) => {
                let ip = resolve_domain_name_to_ip(domain.as_str())
                    .map_err(|e| HttpRequestError::from(Box::new(StringError::from(e))))?;
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
            HttpVerb::Get => "GET",
            HttpVerb::Head => "HEAD",
        };

        let mut stream = TcpStream::connect(SocketAddr::new(*ip, self.url.port))?;
        let request = format!(
            "{verb} {path} HTTP/1.1\r\nHost: {domain}\r\n\r\n",
            verb = verb_str,
            path = self.url.http_request_path(),
            domain = host
        );
        stream.write_all(request.as_bytes())?;
        Ok(AsyncTcpStream::from_tcp_stream(stream))
    }

    async fn read_full_response(
        &self,
        verb: &HttpVerb,
        mut stream: AsyncTcpStream,
    ) -> Result<HttpResponse> {
        let mut result = vec![];
        let mut status: HttpStatus = Default::default();
        let mut headers: Vec<HttpHeader> = vec![];
        let mut content_length = u32::MAX;
        let mut preamble_length = 0;
        while let Some(bytes) = stream.next().await {
            result.extend(&bytes.map_err(|e| HttpRequestError::from(Box::new(e)))?);
            if preamble_length == 0 && contains_end_of_headers(&result) {
                let parsed = parse_headers(&result);
                if let Some((parsed_status, parsed_headers, remainder)) = parsed {
                    status = parsed_status;
                    headers = parsed_headers;
                    preamble_length = result.len() - remainder.len();
                    content_length = determine_content_length(&verb, &status, &headers)?;
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
            headers,
            body: result,
        })
    }
}
