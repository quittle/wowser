use super::HttpHeader;

#[derive(Debug, PartialEq, Default)]
pub struct HttpHeaderMap {
    headers: Vec<HttpHeader>,
}

impl From<Vec<HttpHeader>> for HttpHeaderMap {
    fn from(headers: Vec<HttpHeader>) -> Self {
        Self { headers }
    }
}

impl HttpHeaderMap {
    /// Gets a computed value for a header. For certain headers, namely ones that hold list-based
    /// data, this may concatenate headers with duplicate field names together. In non-list headers,
    /// only the value of the first occurrence of a header is returned.
    pub fn get(&self, header_name: &str) -> Option<String> {
        if is_list_based_header(header_name) {
            let mut ret = String::new();
            let mut found = false;
            for header in &self.headers {
                if header.name == header_name {
                    found = true;
                    if header.value.is_empty() {
                        continue;
                    } else if ret.is_empty() {
                        ret += &header.value;
                    } else {
                        ret = ret + "," + &header.value;
                    }
                }
            }
            if found {
                Some(ret)
            } else {
                None
            }
        } else {
            self.headers
                .iter()
                .find(|header| header.name == header_name)
                .map(|header| header.value.clone())
        }
    }
}

/// Determines if a header's values are in the form of a comma-separated list.
/// <https://stackoverflow.com/a/4371395/1554990>
fn is_list_based_header(header_name: &str) -> bool {
    matches!(header_name, "cache-control" | "www-authenticate")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_from() {
        let header_map = HttpHeaderMap::from(vec![HttpHeader::new("foo", "bar")]);
        assert_eq!(header_map.headers, vec![HttpHeader::new("foo", "bar")]);
    }

    #[test]
    fn test_regular_values() {
        assert_eq!(HttpHeaderMap::from(vec![]).get("abc"), None);
        assert_eq!(
            HttpHeaderMap::from(vec![HttpHeader::new("foo", "bar")]).get("abc"),
            None
        );
        assert_eq!(
            HttpHeaderMap::from(vec![HttpHeader::new("abc", "123")]).get("abc"),
            Some("123".into())
        );
        assert_eq!(
            HttpHeaderMap::from(vec![HttpHeader::new("abc", "")]).get("abc"),
            Some("".into())
        );
        assert_eq!(
            HttpHeaderMap::from(vec![
                HttpHeader::new("foo", "bar"),
                HttpHeader::new("abc", "123"),
                // The second value should be ignored
                HttpHeader::new("abc", "456")
            ])
            .get("abc"),
            Some("123".into())
        );
    }

    #[test]
    fn test_list_separated_values() {
        assert_eq!(HttpHeaderMap { headers: vec![] }.get("cache-control"), None);
        assert_eq!(
            HttpHeaderMap::from(vec![HttpHeader::new("foo", "bar")]).get("cache-control"),
            None
        );
        assert_eq!(
            HttpHeaderMap::from(vec![HttpHeader::new("cache-control", "abc")]).get("cache-control"),
            Some("abc".into())
        );
        assert_eq!(
            HttpHeaderMap::from(vec![HttpHeader::new("cache-control", "")]).get("cache-control"),
            Some("".into())
        );
        assert_eq!(
            HttpHeaderMap::from(vec![
                HttpHeader::new("cache-control", ""),
                HttpHeader::new("cache-control", "abc"),
                HttpHeader::new("cache-control", ""),
                HttpHeader::new("cache-control", "def"),
            ])
            .get("cache-control"),
            // Note the lack space after the appended one. The whitespace is optional
            Some("abc,def".into()),
            "Ignore empty header values"
        );

        assert_eq!(
            HttpHeaderMap::from(vec![
                HttpHeader::new("foo", "bar"),
                HttpHeader::new("cache-control", "abc, def"),
                // The second value should be appended
                HttpHeader::new("cache-control", "ghi")
            ])
            .get("cache-control"),
            // Note the lack space after the appended one. The whitespace is optional
            Some("abc, def,ghi".into())
        );
    }
}
