use super::super::dns::resolve_domain_name_to_ip;
use super::super::stream::AsyncTcpStream;
use super::constants::{DOUBLE_NEWLINE_BYTES, SINGLE_NEWLINE_BYTES};
use super::http_header_map::HttpHeaderMap;
use super::{structures::HttpVerb, HttpRequestError, HttpResponse, HttpResult, HttpStatus, Result};
use crate::net::http::headers::parse_status_headers;
use crate::util::vec_find_subslice;
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

enum Chunk<'a> {
    Data { data: &'a [u8], offset: usize },
    Incomplete,
    End { offset: usize },
}

/// Tries to parse bytes as chunk from an HTTP body with Transfer-Encoding set to "chunked".
fn try_parse_chunk(bytes: &[u8]) -> Result<Chunk> {
    if let Some(end_of_length) = vec_find_subslice(bytes, SINGLE_NEWLINE_BYTES) {
        let chunk_len_hex_bytes = &bytes[..end_of_length];
        let chunk_len_hex = std::str::from_utf8(chunk_len_hex_bytes)?;
        let chunk_len = usize::from_str_radix(chunk_len_hex, 16)?;
        let data_offset = end_of_length + SINGLE_NEWLINE_BYTES.len();
        let total_chunk_len = data_offset + chunk_len + SINGLE_NEWLINE_BYTES.len();

        if bytes.len() < total_chunk_len {
            return Ok(Chunk::Incomplete);
        }

        if chunk_len == 0 {
            return Ok(Chunk::End {
                offset: total_chunk_len,
            });
        }

        Ok(Chunk::Data {
            data: &bytes[data_offset..data_offset + chunk_len],
            offset: total_chunk_len,
        })
    } else {
        Ok(Chunk::Incomplete)
    }
}

async fn chunked_transfer_encoding(
    mut stream: AsyncTcpStream,
    body_start: &[u8],
) -> Result<Vec<u8>> {
    let mut ret = vec![];
    let mut chunk: Vec<u8> = body_start.into();

    loop {
        match try_parse_chunk(&chunk)? {
            Chunk::Data { data, offset } => {
                ret.extend(data);
                chunk.drain(0..offset);
            }
            Chunk::Incomplete => {
                if let Some(bytes) = stream.next().await {
                    let bytes = bytes?;
                    chunk.extend(bytes);
                } else {
                    return Err(StringError::from(
                        "Unexpected end of stream during chunked transfer decoding",
                    )
                    .into());
                }
            }
            Chunk::End { offset } => {
                chunk.drain(0..offset);
                if !chunk.is_empty() {
                    return Err(StringError::from("Data at end of chunked transfer").into());
                }

                return Ok(ret);
            }
        }
    }
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
        let request = format!("{verb_str} {path} HTTP/1.1\r\nHost: {host}\r\n\r\n");
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
                    if header_map.get("transfer-encoding") == Some("chunked".into()) {
                        let body = chunked_transfer_encoding(stream, remainder).await?;
                        return Ok(HttpResponse {
                            status,
                            headers: header_map,
                            body,
                        });
                    }
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
