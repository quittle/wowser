use regex::Regex;

use crate::util::{string_to_bytes, HexConversion};

pub enum CssDisplay {
    Block,
    Inline,
}

impl CssDisplay {
    pub fn from_raw_value(value: &str) -> Option<Self> {
        if value.eq_ignore_ascii_case("block") {
            Some(Self::Block)
        } else if value.eq_ignore_ascii_case("inline") {
            Some(Self::Inline)
        } else {
            None
        }
    }
}
pub enum CssBackgroundColor {
    Rgba(u8, u8, u8, u8),
}

impl CssBackgroundColor {
    pub fn from_raw_value(value: &str) -> Option<Self> {
        if Self::rgb3_hex().is_match(value) {
            let chars: Vec<char> = value.chars().collect();
            let r = chars[1].hex_to_byte().ok()?;
            let g = chars[2].hex_to_byte().ok()?;
            let b = chars[3].hex_to_byte().ok()?;
            println!("{} {} {}", r, g, b);
            Some(CssBackgroundColor::Rgba((r << 4) + r, (g << 4) + g, (b << 4) + b, 255))
        } else if Self::rgb6_hex().is_match(value) {
            let rgb = string_to_bytes(&value[1..]).ok()?;
            Some(CssBackgroundColor::Rgba(rgb[0], rgb[1], rgb[2], 255))
        } else {
            None
        }
    }

    fn rgb3_hex() -> Regex {
        Regex::new(r#"#([\da-f]){3}"#).unwrap()
    }

    fn rgb6_hex() -> Regex {
        Regex::new(r#"#([\da-f]){6}"#).unwrap()
    }
}
