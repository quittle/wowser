use super::super::dns::resolve_domain_name_to_ip;
use crate::{
    net::{Url, UrlHost},
    util::StringError,
};
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::{error::Error, fmt::Display};

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
    pub async fn get(&mut self) -> Result<Vec<u8>, HttpRequestError> {
        let (host, ip) = match &(self.url.host) {
            UrlHost::IP(ip) => (ip.to_string(), *ip),
            UrlHost::DomainName(domain) => {
                let ip =
                    resolve_domain_name_to_ip(domain.as_str()).map_err(|e| HttpRequestError {
                        err: Box::new(StringError::from(e)),
                    })?;
                (domain.to_string(), IpAddr::V4(ip))
            }
        };

        self.get_tcp(host, &ip)
            .await
            .map_err(|e| HttpRequestError { err: Box::new(e) })
    }

    async fn get_tcp(&mut self, host: String, ip: &IpAddr) -> Result<Vec<u8>, std::io::Error> {
        let mut stream = TcpStream::connect(SocketAddr::new(*ip, self.url.port))?;
        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\n\r\n",
            self.url.http_request_path(),
            host
        );
        stream.write_all(request.as_bytes())?;
        let mut response = [0u8; 1024];
        let response_bytes = stream.read(&mut response)?;
        Ok(response[..response_bytes].to_vec())
    }
}
