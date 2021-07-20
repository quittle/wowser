use std::fmt;
use std::net::IpAddr;

use regex::Regex;

/// Represents protocols supported by wowser. Future additions to include HTTPS, FTP, FILE, etc.
#[derive(Debug, PartialEq)]
pub enum UrlProtocol {
    Http,
}

impl UrlProtocol {
    fn parse(protocol: &str) -> Option<Self> {
        match protocol.to_ascii_lowercase().as_str() {
            "http" => Some(Self::Http),
            _ => None,
        }
    }
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
#[derive(Debug, PartialEq)]
pub enum UrlHost {
    IP(IpAddr),
    DomainName(String),
}

impl UrlHost {
    pub fn parse(host: &str) -> Option<Self> {
        Some(if let Ok(ip_addr) = host.parse::<IpAddr>() {
            UrlHost::IP(ip_addr)
        } else {
            UrlHost::DomainName(host.into())
        })
    }
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
#[derive(Debug, PartialEq)]
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
        let mut path_string = path.into();
        if path_string.is_empty() {
            path_string = "/".into();
        }
        Url {
            protocol,
            host,
            port,
            path: path_string,
            query_string: query_string.into(),
            fragment: fragment.into(),
        }
    }

    pub fn parse(url: &str) -> Option<Self> {
        let url_regex = Regex::new(
            r"(\w+)://([a-zA-Z\d]+([\w\d\-\.]*[a-zA-Z\d]+)?)(:(\d+))?(/[^\?]*)?(\?([^#]*))?(#(.*))?",
        )
        .unwrap();
        let captures = url_regex.captures(url)?;
        // If it failed to match the full string, bail out early
        if captures.get(0)?.range().len() != url.len() {
            return None;
        }
        let protocol = captures.get(1)?.as_str();
        let host = captures.get(2)?.as_str();
        let port = captures
            .get(5)
            .map(|capture| capture.as_str())
            .unwrap_or("80");
        let path = captures
            .get(6)
            .map(|capture| capture.as_str())
            .unwrap_or("/");
        let query_string = captures
            .get(8)
            .map(|capture| capture.as_str())
            .unwrap_or("");
        let fragment = captures
            .get(10)
            .map(|capture| capture.as_str())
            .unwrap_or("");

        Some(Url {
            protocol: UrlProtocol::parse(protocol)?,
            host: UrlHost::parse(host)?,
            port: port.parse::<u16>().ok()?,
            path: path.to_string(),
            query_string: query_string.to_string(),
            fragment: fragment.to_string(),
        })
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
        assert_eq!(
            "example.com",
            UrlHost::DomainName("example.com".to_string()).to_string()
        );
        assert_eq!(
            "1.2.3.4",
            UrlHost::IP(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))).to_string()
        );
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
            "http://example.com:80/",
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

    #[test]
    pub fn test_url_parse_invalid() {
        assert_eq!(Url::parse("foo://example.com"), None);
        assert_eq!(Url::parse("http//example.com"), None);
        assert_eq!(Url::parse("http:/example.com/"), None);
        assert_eq!(Url::parse("http:/e/"), None);
        assert_eq!(Url::parse("http://example-"), None);
        assert_eq!(Url::parse("http://-example"), None);
        assert_eq!(Url::parse("http://example."), None);
        assert_eq!(Url::parse("http://.example"), None);
        assert_eq!(Url::parse("http://.example"), None);
        assert_eq!(Url::parse("http://example:"), None);
        assert_eq!(Url::parse("http://example:/"), None);
        assert_eq!(Url::parse("http://exam:ple"), None);
        assert_eq!(Url::parse("http://example::3"), None);
    }

    #[test]
    pub fn test_url_parse_min_domain() {
        assert_eq!(
            Url::parse("http://a"),
            Some(Url::new(
                UrlProtocol::Http,
                UrlHost::DomainName("a".into()),
                80,
                "/",
                "",
                "",
            ))
        );
        assert_eq!(
            Url::parse("http://a-b"),
            Some(Url::new(
                UrlProtocol::Http,
                UrlHost::DomainName("a-b".into()),
                80,
                "/",
                "",
                "",
            ))
        );
        assert_eq!(
            Url::parse("http://a-b.c"),
            Some(Url::new(
                UrlProtocol::Http,
                UrlHost::DomainName("a-b.c".into()),
                80,
                "/",
                "",
                "",
            ))
        );
    }

    #[test]
    pub fn test_url_parse_simple() {
        assert_eq!(
            Url::parse("http://example.com"),
            Some(Url::new(
                UrlProtocol::Http,
                UrlHost::DomainName("example.com".into()),
                80,
                "/",
                "",
                "",
            ))
        );
        assert_eq!(
            Url::parse("http://example.com/path"),
            Some(Url::new(
                UrlProtocol::Http,
                UrlHost::DomainName("example.com".into()),
                80,
                "/path",
                "",
                "",
            ))
        );
        assert_eq!(
            Url::parse("http://example.com?query"),
            Some(Url::new(
                UrlProtocol::Http,
                UrlHost::DomainName("example.com".into()),
                80,
                "/",
                "query",
                "",
            ))
        );
        assert_eq!(
            Url::parse("http://example.com#fragment"),
            Some(Url::new(
                UrlProtocol::Http,
                UrlHost::DomainName("example.com".into()),
                80,
                "/",
                "",
                "fragment",
            ))
        );
        assert_eq!(
            Url::parse("http://example.com:90#fragment"),
            Some(Url::new(
                UrlProtocol::Http,
                UrlHost::DomainName("example.com".into()),
                90,
                "/",
                "",
                "fragment",
            ))
        );
    }

    #[test]
    pub fn test_url_parse_ip() {
        assert_eq!(
            Url::parse("http://1.2.3.4:5/6.7.8.9?10.11.12.13#14.15.16.17"),
            Some(Url::new(
                UrlProtocol::Http,
                UrlHost::IP(IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4))),
                5,
                "/6.7.8.9",
                "10.11.12.13",
                "14.15.16.17",
            ))
        );

        // TODO: Support IPv6 URLs
        // assert_eq!(
        //     Url::parse("http://[1:2:3:4:5:6:7:8]:9/[11:12:13:14:15:16:17:18]?[21:22:23:24:25:26:27:28]#[31:32:33:34:35:36:37:38]"),
        //     Some(Url::new(
        //         UrlProtocol::Http,
        //         UrlHost::IP(IpAddr::V6(Ipv6Addr::new(1,2,3,4,5,6,7,8))),
        //         9,
        //         "/[11:12:13:14:15:16:17:18]",
        //         "[21:22:23:24:25:26:27:28]",
        //         "[31:32:33:34:35:36:37:38]",
        //     ))
        // );
    }

    #[test]
    pub fn test_url_parse_complex() {
        assert_eq!(
            Url::parse("http://h:1/?#"),
            Some(Url::new(
                UrlProtocol::Http,
                UrlHost::DomainName("h".into()),
                1,
                "/",
                "",
                "",
            ))
        );
        assert_eq!(
            Url::parse("http://h:1?#"),
            Some(Url::new(
                UrlProtocol::Http,
                UrlHost::DomainName("h".into()),
                1,
                "/",
                "",
                "",
            ))
        );
        assert_eq!(
            Url::parse("http://example:999////?query//=?foo#frag?#frag://example"),
            Some(Url::new(
                UrlProtocol::Http,
                UrlHost::DomainName("example".into()),
                999,
                "////",
                "query//=?foo",
                "frag?#frag://example",
            ))
        );
    }
}
