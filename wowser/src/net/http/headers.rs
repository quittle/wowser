use crate::{net::http::constants::SINGLE_NEWLINE_BYTES, util::vec_window_split};

use super::{HttpHeader, HttpStatus};

pub fn parse_status_headers(vec: &[u8]) -> Option<(HttpStatus, Vec<HttpHeader>, &[u8])> {
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
            // Normalize the header name upon parsing by making it lower case
            let name = values.next()?.to_ascii_lowercase();
            let value = values.next()?.trim().to_owned();
            Some(HttpHeader { name, value })
        });

    let mut ret_headers = vec![];
    for header in headers {
        ret_headers.push(header?);
    }

    offset += SINGLE_NEWLINE_BYTES.len();

    Some((status, ret_headers, &vec[offset..]))
}

#[cfg(test)]
mod tests {
    use crate::net::{HttpHeader, HttpStatus};

    use super::parse_status_headers;

    #[test]
    fn test_example_headers() {
        let headers = include_bytes!("example-headers.txt");
        let (status, headers, body) =
            parse_status_headers(headers).expect("Unable to parse example headers");
        assert_eq!(
            status,
            HttpStatus {
                http_version: "HTTP/2".into(),
                status_code: 200,
                reason_phrase: "".into(),
            }
        );
        assert_eq!(
            headers,
            vec![
                HttpHeader::new("age", "431430"),
                HttpHeader::new("cache-control", "max-age=604800"),
                HttpHeader::new("content-type", "text/html; charset=UTF-8"),
                HttpHeader::new("date", "Sat, 25 Sep 2021 18:04:17 GMT"),
                HttpHeader::new("etag", "\"3147526947+ident\""),
                HttpHeader::new("expires", "Sat, 02 Oct 2021 18:04:17 GMT"),
                HttpHeader::new("last-modified", "Thu, 17 Oct 2019 07:18:26 GMT"),
                HttpHeader::new("server", "ECS (phd/FD6D)"),
                HttpHeader::new("vary", "Accept-Encoding"),
                HttpHeader::new("x-cache", "HIT"),
                HttpHeader::new("content-length", "6"),
            ],
        );
        assert_eq!(body, b"<html>");
    }
}
