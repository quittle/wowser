use std::fmt;
use std::net::IpAddr;

/// Represents protocols supported by wowser. Future additions to include HTTPS, FTP, FILE, etc.
pub enum UrlProtocol {
    Http,
}

impl fmt::Display for UrlProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let protocol = match self {
            UrlProtocol::Http => "http",
        };

        write!(f, "{}", protocol)
    }
}

/// Represents the host portion of a URL.
pub enum UrlHost {
    IP(IpAddr),
    DomainName(String),
}

impl fmt::Display for UrlHost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UrlHost::IP(ip) => write!(f, "{}", ip),
            UrlHost::DomainName(host) => write!(f, "{}", host),
        }
    }
}

/// Represents an entire URL in a way helpful for an HTTP library. The querystring is intentionally
/// not a represented as a map because it's simply a string for the purposes of the protocol.
pub struct Url {
    pub protocol: UrlProtocol,
    pub host: UrlHost,
    pub port: u16,
    pub path: String,
    pub query_string: String,
    pub fragment: String,
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}://{}:{}", self.protocol, self.host, self.port)?;
        if !self.path.is_empty() {
            write!(f, "{}", self.path)?;
        }
        if !self.query_string.is_empty() {
            write!(f, "?{}", self.query_string)?;
        }
        if !self.fragment.is_empty() {
            write!(f, "#{}", self.fragment)?;
        }

        Ok(())
    }
}

impl Url {
    pub fn new<S>(
        protocol: UrlProtocol,
        host: UrlHost,
        port: u16,
        path: S,
        query_string: S,
        fragment: S,
    ) -> Url
    where
        S: Into<String>,
    {
        Url {
            protocol,
            host,
            port,
            path: path.into(),
            query_string: query_string.into(),
            fragment: fragment.into(),
        }
    }
    /// The representation of the path to send in HTTP requests
    pub fn http_request_path(&self) -> String {
        let mut ret = String::new();
        if self.path.is_empty() {
            ret += "/";
        } else {
            ret += &self.path;
        }
        if !self.query_string.is_empty() {
            ret += "?";
            ret += &self.query_string;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    pub fn test_url_protocol() {
        assert_eq!("http", UrlProtocol::Http.to_string());
    }

    #[test]
    pub fn test_url_host() {
        assert_eq!("example.com", UrlHost::DomainName("example.com".to_string()).to_string());
        assert_eq!("1.2.3.4", UrlHost::IP(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))).to_string());
        assert_eq!(
            "1:2:3:4:5:6:7:8",
            UrlHost::IP(IpAddr::V6(Ipv6Addr::new(1, 2, 3, 4, 5, 6, 7, 8))).to_string()
        );
    }

    #[test]
    pub fn test_url_display_full() {
        assert_eq!(
            "http://example.com:80/path?query#fragment",
            Url::new(
                UrlProtocol::Http,
                UrlHost::DomainName("example.com".to_string()),
                80,
                "/path",
                "query",
                "fragment",
            )
            .to_string()
        );
    }

    #[test]
    pub fn test_url_display_mimimal() {
        assert_eq!(
            "http://example.com:80",
            Url::new(
                UrlProtocol::Http,
                UrlHost::DomainName("example.com".to_string()),
                80,
                "",
                "",
                "",
            )
            .to_string()
        );
    }

    #[test]
    pub fn test_http_request_path() {
        let mut url = Url::new(
            UrlProtocol::Http,
            UrlHost::DomainName("example.com".to_string()),
            80,
            "",
            "",
            "",
        );
        assert_eq!("/", url.http_request_path());
        url.path = "/".to_string();
        assert_eq!("/", url.http_request_path());
        url.query_string = "query".to_string();
        assert_eq!("/?query", url.http_request_path());
        url.fragment = "fragment".to_string();
        assert_eq!("/?query", url.http_request_path());
    }
}
