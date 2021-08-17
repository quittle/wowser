use std::str;
pub trait HexConversion {
    fn hex_to_byte(&self) -> Result<u8, String>;
}

impl HexConversion for char {
    fn hex_to_byte(&self) -> Result<u8, String> {
        Ok(match self {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'a' => 10,
            'b' => 11,
            'c' => 12,
            'd' => 13,
            'e' => 14,
            'f' => 15,
            _ => {
                return Err(format!("Invalid character {}", self));
            }
        })
    }
}

fn u4_to_hex(u4: u8) -> char {
    match u4 {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        15 => 'f',
        _ => panic!("Invalid u4 passed in {}", u4),
    }
}

pub fn byte_to_hex(byte: u8) -> String {
    let a = byte >> 4;
    let b = byte & 0xf;

    format!("{}{}", u4_to_hex(a), u4_to_hex(b))
}

#[allow(dead_code)]
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| byte_to_hex(*b)).collect::<String>()
}

pub fn string_to_bytes(s: &str) -> Result<Vec<u8>, String> {
    let normalized_string = s
        .replace(|c: char| c.is_whitespace(), "")
        .to_ascii_lowercase();
    if normalized_string.len() % 2 != 0 {
        return Err("Even number of characters required".to_string());
    }
    let mut ret = vec![];
    for i in (0..normalized_string.len()).step_by(2) {
        let chars: Vec<char> = normalized_string.get(i..i + 2).expect("").chars().collect();
        let a = chars.get(0).expect("").hex_to_byte();
        let b = chars.get(1).expect("").hex_to_byte();
        match a {
            Ok(a) => match b {
                Ok(b) => {
                    ret.push(a << 4 | b);
                }
                Err(e) => return Err(e),
            },
            Err(e) => return Err(e),
        }
    }
    Ok(ret)
}

pub fn u8_to_str(bytes: &[u8]) -> Result<&str, String> {
    str::from_utf8(bytes).map_err(|e| {
        format!(
            "{} - Original String<{}>",
            e.to_string(),
            String::from_utf8_lossy(bytes)
        )
    })
}

pub fn split_str_into_2<T, Transform, EM, EF, E>(
    s: &str,
    pattern: &str,
    transform: Transform,
    missing_message: EM,
) -> Result<(T, T), E>
where
    Transform: Fn(&str) -> Result<T, EF>,
    EM: Copy,
    E: From<EM> + From<EF>,
{
    let mut split = s.splitn(2, pattern);
    Ok((
        transform(split.next().ok_or(missing_message)?)?,
        transform(split.next().ok_or(missing_message)?)?,
    ))
}

pub fn split_str_into_3<T, Transform, EM, EF, E>(
    s: &str,
    pattern: &str,
    transform: Transform,
    missing_message: EM,
) -> Result<(T, T, T), E>
where
    Transform: Fn(&str) -> Result<T, EF>,
    EM: Copy,
    E: From<EM> + From<EF>,
{
    let mut split = s.splitn(3, pattern);
    Ok((
        transform(split.next().ok_or(missing_message)?)?,
        transform(split.next().ok_or(missing_message)?)?,
        transform(split.next().ok_or(missing_message)?)?,
    ))
}

pub fn split_str_into_4<T, Transform, EM, EF, E>(
    s: &str,
    pattern: &str,
    transform: Transform,
    missing_message: EM,
) -> Result<(T, T, T, T), E>
where
    Transform: Fn(&str) -> Result<T, EF>,
    EM: Copy,
    E: From<EM> + From<EF>,
{
    let mut split = s.splitn(4, pattern);
    Ok((
        transform(split.next().ok_or(missing_message)?)?,
        transform(split.next().ok_or(missing_message)?)?,
        transform(split.next().ok_or(missing_message)?)?,
        transform(split.next().ok_or(missing_message)?)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::ParseIntError;

    #[derive(Clone, Copy, Debug)]
    struct TestError {}

    impl From<ParseIntError> for TestError {
        fn from(_: ParseIntError) -> Self {
            TestError {}
        }
    }

    impl From<i32> for TestError {
        fn from(_: i32) -> Self {
            TestError {}
        }
    }

    impl From<&str> for TestError {
        fn from(_: &str) -> Self {
            TestError {}
        }
    }

    #[test]
    fn test_split_str() {
        let result: Result<(i32, i32), TestError> =
            split_str_into_2("1 2 3", " ", |s| s.parse::<i32>(), "missing");
        result.expect_err("Invalid result");

        let result: Result<(i32, i32), TestError> =
            split_str_into_2("1 2", " ", |v| v.parse::<i32>(), "missing");
        assert_eq!(result.unwrap(), (1, 2));

        let result: Result<(i32, i32, i32), TestError> =
            split_str_into_3("1 2 3", " ", |v| v.parse::<i32>(), "missing");
        assert_eq!(result.unwrap(), (1, 2, 3));

        let result: Result<(i32, i32, i32, i32), TestError> =
            split_str_into_4("1 2 3 4", " ", |v| v.parse::<i32>(), "missing");
        assert_eq!(result.unwrap(), (1, 2, 3, 4));
    }

    #[test]
    fn hex_decode() -> Result<(), String> {
        assert_eq!(string_to_bytes("00")?, vec!(0));
        assert_eq!(string_to_bytes("01")?, vec!(1));
        assert_eq!(string_to_bytes("0a")?, vec!(10));
        assert_eq!(string_to_bytes("0f")?, vec!(15));
        assert_eq!(string_to_bytes("ff")?, vec!(255));
        assert_eq!(string_to_bytes("0100")?, vec!(1, 0));
        Ok(())
    }

    #[test]
    fn hex_encode() {
        assert_eq!(byte_to_hex(0), "00");
        assert_eq!(byte_to_hex(255), "ff");
        assert_eq!(byte_to_hex(171), "ab");
        assert_eq!(bytes_to_hex(&[0]), "00");
    }

    #[test]
    fn test_u8_to_str() {
        assert_eq!(
            u8_to_str([0xc0].as_ref()).expect_err(""),
            "invalid utf-8 sequence of 1 bytes from index 0 - Original String<�>"
        );
        assert_eq!(u8_to_str([b'a'].as_ref()).expect(""), "a");
    }
}
