use regex::Regex;

use crate::util::{string_to_bytes, HexConversion};

#[derive(PartialEq, Debug)]
pub enum CssDisplay {
    Block,
    Inline,
    None,
}

impl CssDisplay {
    pub fn from_raw_value(value: &str) -> Option<Self> {
        if value.eq_ignore_ascii_case("block") {
            Some(Self::Block)
        } else if value.eq_ignore_ascii_case("inline") {
            Some(Self::Inline)
        } else if value.eq_ignore_ascii_case("none") {
            Some(Self::None)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum CssColor {
    Rgba(u8, u8, u8, u8),
}

impl CssColor {
    pub fn from_raw_value(value: &str) -> Option<Self> {
        if Self::rgb3_hex().is_match(value) {
            let chars: Vec<char> = value.chars().collect();
            let r = chars[1].hex_to_byte().ok()?;
            let g = chars[2].hex_to_byte().ok()?;
            let b = chars[3].hex_to_byte().ok()?;
            Some(Self::Rgba((r << 4) + r, (g << 4) + g, (b << 4) + b, 255))
        } else if Self::rgb6_hex().is_match(value) {
            let rgb = string_to_bytes(&value[1..]).ok()?;
            Some(Self::Rgba(rgb[0], rgb[1], rgb[2], 255))
        } else {
            None
        }
    }

    fn rgb3_hex() -> Regex {
        Regex::new(r#"^#([\da-f]){3}$"#).unwrap()
    }

    fn rgb6_hex() -> Regex {
        Regex::new(r#"^#([\da-f]){6}$"#).unwrap()
    }
}

#[derive(PartialEq, Debug)]
pub enum CssDimension {
    Px(f32),
}

impl CssDimension {
    pub fn from_raw_value(value: &str) -> Option<Self> {
        Self::px()
            .captures(value)
            .map(|captures| Self::Px(captures[1].parse().unwrap()))
    }

    fn px() -> Regex {
        Regex::new(r#"^(-?\d+(\.\d+)?)px$"#).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(None, CssDisplay::from_raw_value("foo"));
        assert_eq!(Some(CssDisplay::Block), CssDisplay::from_raw_value("block"));
        assert_eq!(
            Some(CssDisplay::Inline),
            CssDisplay::from_raw_value("inline")
        );
    }

    #[test]
    fn test_color() {
        assert_eq!(None, CssColor::from_raw_value("foo"));
        assert_eq!(None, CssColor::from_raw_value("#fo0"));
        assert_eq!(None, CssColor::from_raw_value("#12"));
        assert_eq!(None, CssColor::from_raw_value("#12345"));
        assert_eq!(
            Some(CssColor::Rgba(170, 187, 204, 255)),
            CssColor::from_raw_value("#abc")
        );
        assert_eq!(
            Some(CssColor::Rgba(18, 171, 240, 255)),
            CssColor::from_raw_value("#12abf0")
        );
    }

    #[test]
    fn test_dimension() {
        assert_eq!(None, CssDimension::from_raw_value("foo"));
        assert_eq!(None, CssDimension::from_raw_value("13"));
        assert_eq!(None, CssDimension::from_raw_value("13pxx"));
        assert_eq!(None, CssDimension::from_raw_value("13 px"));
        assert_eq!(
            Some(CssDimension::Px(0.0)),
            CssDimension::from_raw_value("0px")
        );
        assert_eq!(
            Some(CssDimension::Px(10.3)),
            CssDimension::from_raw_value("10.3px")
        );
        assert_eq!(
            Some(CssDimension::Px(-10.3)),
            CssDimension::from_raw_value("-10.3px")
        );
    }
}
