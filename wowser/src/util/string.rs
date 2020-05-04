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

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| byte_to_hex(*b)).collect::<String>()
}

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

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

        let arr: [u8; 1] = [0];
        assert_eq!(bytes_to_hex(&arr), "00");
    }
}
