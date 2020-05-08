pub fn u8_arr_to_u16(a: u8, b: u8) -> u16 {
    ((a as u16) << 8) + b as u16
}

pub fn u8_to_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    ((a as u32) << 24) + ((b as u32) << 16) + ((c as u32) << 8) + d as u32
}

pub fn u8_to_i32(a: u8, b: u8, c: u8, d: u8) -> i32 {
    u8_to_u32(a, b, c, d) as i32
}

pub fn u4_from_u8(byte: u8, start_offset: U4Bit) -> u8 {
    let bitmask = (8 + 4 + 2 + 1) << (4 - (start_offset as u8));
    bitmask & byte
}

pub enum U4Bit {
    Zero,
    One,
    Two,
    Three,
    Four,
}

impl From<&U4Bit> for u8 {
    fn from(bit: &U4Bit) -> u8 {
        match bit {
            U4Bit::Zero => 0,
            U4Bit::One => 1,
            U4Bit::Two => 2,
            U4Bit::Three => 3,
            U4Bit::Four => 4,
        }
    }
}

/// The index of a bit in a byte, starting from the left
pub enum Bit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl From<&Bit> for u8 {
    fn from(bit: &Bit) -> u8 {
        match bit {
            Bit::Zero => 0,
            Bit::One => 1,
            Bit::Two => 2,
            Bit::Three => 3,
            Bit::Four => 4,
            Bit::Five => 5,
            Bit::Six => 6,
            Bit::Seven => 7,
        }
    }
}

pub fn get_bit(byte: u8, index: Bit) -> bool {
    byte & (1 << (7 - index as u8)) > 0
}

pub fn offset_bit_merge(byte_a: u8, offset: Bit, byte_b: u8) -> u16 {
    u8_arr_to_u16(byte_a & (255 >> offset as u8), byte_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit() {
        let v = 0b1010_0011;
        assert!(get_bit(v, Bit::Zero));
        assert!(!get_bit(v, Bit::One));
        assert!(get_bit(v, Bit::Two));
        assert!(!get_bit(v, Bit::Three));
        assert!(!get_bit(v, Bit::Four));
        assert!(!get_bit(v, Bit::Five));
        assert!(get_bit(v, Bit::Six));
        assert!(get_bit(v, Bit::Seven));
    }
}
