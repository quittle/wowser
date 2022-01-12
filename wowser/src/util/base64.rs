/// Encodes and Decodes to and from [Base64](https://en.wikipedia.org/wiki/Base64)
pub trait Base64 {
    /// Encodes the data to a Base64 string. This operation should never fail
    fn base64_encode(&self) -> String;

    /// Attempts to decode the data from Base64 to its consistuent bytes. This
    /// may fail and return None if the input is invalid Base64 encoding.
    fn base64_decode(&self) -> Option<Vec<u8>>;
}

impl Base64 for &str {
    fn base64_encode(&self) -> String {
        base64_encode(self.as_bytes())
    }

    fn base64_decode(&self) -> Option<Vec<u8>> {
        base64_decode(self)
    }
}

impl Base64 for String {
    fn base64_encode(&self) -> String {
        base64_encode(self.as_bytes())
    }

    fn base64_decode(&self) -> Option<Vec<u8>> {
        base64_decode(self)
    }
}

impl Base64 for &[u8] {
    fn base64_encode(&self) -> String {
        base64_encode(self)
    }

    fn base64_decode(&self) -> Option<Vec<u8>> {
        base64_decode(std::str::from_utf8(self).ok()?)
    }
}

impl Base64 for Vec<u8> {
    fn base64_encode(&self) -> String {
        base64_encode(self)
    }

    fn base64_decode(&self) -> Option<Vec<u8>> {
        base64_decode(std::str::from_utf8(self).ok()?)
    }
}

fn base64_encode(data: &[u8]) -> String {
    let extra = (3 - data.len() % 3) % 3;
    let padded_data = [data, &vec![0u8; extra]].concat();
    let mut string: String = padded_data
        .chunks(3)
        .flat_map(|bytes| {
            let a = base64_encode_6bit_byte((bytes[0] & 0b11111100) >> 2);
            let b = base64_encode_6bit_byte(
                ((bytes[0] & 0b00000011) << 4) | ((bytes[1] & 0b11110000) >> 4),
            );
            let c = base64_encode_6bit_byte(
                ((bytes[1] & 0b00001111) << 2) | ((bytes[2] & 0b11000000) >> 6),
            );
            let d = base64_encode_6bit_byte(bytes[2] & 0b00111111);

            [a, b, c, d]
        })
        // Remove the trailing characters added as padding
        .take(padded_data.len() * 4 / 3 - extra)
        .collect();
    // Add "=" as padding
    string.push_str(&str::repeat("=", extra));

    string
}

fn base64_decode(data: &str) -> Option<Vec<u8>> {
    let data_len = data.len();
    if data_len % 4 != 0 {
        return None;
    }
    let padding_len = data_len - data.trim_end_matches('=').len();
    let result_len = data_len * 3 / 4 - padding_len;

    let mut result = Vec::with_capacity(result_len);
    for chunk in data.as_bytes().chunks(4) {
        let a = base64_decode_6bit_byte(chunk[0])?;
        let b = base64_decode_6bit_byte(chunk[1])?;
        let c = base64_decode_6bit_byte(chunk[2])?;
        let d = base64_decode_6bit_byte(chunk[3])?;

        let byte1 = (a << 2) | ((b & 0b00110000) >> 4);
        let byte2 = ((b & 0b00001111) << 4) | ((c & 0b00111100) >> 2);
        let byte3 = ((c & 0b00000011) << 6) | d;

        result.push(byte1);
        if result.len() != result_len {
            result.push(byte2);
        }
        if result.len() != result_len {
            result.push(byte3);
        }
    }
    Some(result)
}

fn base64_encode_6bit_byte(byte: u8) -> char {
    (match byte {
        0..=25 => 65 + byte,                                   // A..
        26..=51 => 97 + byte - 26,                             // a..
        52..=61 => 48 + byte - 52,                             // 0..
        62 => 43,                                              // +
        63 => 47,                                              // /
        _ => unreachable!("Invalid byte passed in: {}", byte), // This should never happen due to how base64_encode shifts bits
    }) as char
}

fn base64_decode_6bit_byte(byte: u8) -> Option<u8> {
    let c = byte as char;
    Some(match c {
        'A'..='Z' => byte - 65,
        'a'..='z' => byte - 97 + 26,
        '0'..='9' => byte - 48 + 52,
        '+' => 62,
        '/' => 63,
        '=' => 0, // padding
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Manually generated using US
    /// ```js
    /// let str = "";
    /// for (let i = 0; i < 256; i++) {
    ///     str += String.fromCharCode(i);
    /// };
    /// btoa(a);
    /// ```
    const ALL_BYTES_ENCODED: &str = "AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8gISIjJCUmJygpKissLS4vMDEyMzQ1Njc4OTo7PD0+P0BBQkNERUZHSElKS0xNTk9QUVJTVFVWV1hZWltcXV5fYGFiY2RlZmdoaWprbG1ub3BxcnN0dXZ3eHl6e3x9fn+AgYKDhIWGh4iJiouMjY6PkJGSk5SVlpeYmZqbnJ2en6ChoqOkpaanqKmqq6ytrq+wsbKztLW2t7i5uru8vb6/wMHCw8TFxsfIycrLzM3Oz9DR0tPU1dbX2Nna29zd3t/g4eLj5OXm5+jp6uvs7e7v8PHy8/T19vf4+fr7/P3+/w==";

    fn all_bytes() -> Vec<u8> {
        (0u8..=255).collect()
    }

    #[test]
    fn test_base64_encode() {
        assert_eq!("".base64_encode(), "");
        assert_eq!("abc".base64_encode(), "YWJj");
        assert_eq!("1234".base64_encode(), "MTIzNA==");
        assert_eq!(base64_encode(&all_bytes()), ALL_BYTES_ENCODED);
    }

    #[test]
    fn test_base64_decode() {
        assert_eq!("abc".base64_decode(), None, "Not divisible by 4");
        assert_eq!("abc-".base64_decode(), None, "Invalid character");

        assert_eq!("".base64_decode(), Some(vec![]));
        assert_eq!("YWJj".base64_decode(), Some(b"abc".to_vec()));
        assert_eq!("MTIzNA==".base64_decode(), Some(b"1234".to_vec()));
        assert_eq!(ALL_BYTES_ENCODED.base64_decode(), Some(all_bytes()));
    }
}
