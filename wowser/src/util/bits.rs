pub fn u8_arr_to_u16(a: u8, b: u8) -> u16 {
    ((a as u16) << 8) + b as u16
}

/// This compacts the bytes in the order provided
pub fn u8_to_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    ((a as u32) << 24) + ((b as u32) << 16) + ((c as u32) << 8) + d as u32
}

pub fn u8_to_i32(a: u8, b: u8, c: u8, d: u8) -> i32 {
    u8_to_u32(a, b, c, d) as i32
}

/// Given the following fictitous byte of data and an offset of 3
///
///  0 1 2 3 4 5 6 7
/// +-+-+-+-+-+-+-+-+
/// |A|B|C|D|E|F|G|H|
/// +-+-+-+-+-+-+-+-+
///
/// The returned fictitious byte will be
///
///  0 1 2 3 4 5 6 7
/// +-+-+-+-+-+-+-+-+
/// |0|0|0|0|D|E|F|G|
/// +-+-+-+-+-+-+-+-+
///
/// The returned u8 will never have a value > 15 since it should represent a u4 value.
pub fn u4_from_u8(byte: u8, start_offset: U4BitOffset) -> u8 {
    let shifted_byte = byte >> (4 - (start_offset as u8));
    (8 + 4 + 2 + 1) & shifted_byte
}

pub enum U4BitOffset {
    Zero,
    One,
    Two,
    Three,
    Four,
}

impl From<&U4BitOffset> for u8 {
    fn from(bit: &U4BitOffset) -> u8 {
        match bit {
            U4BitOffset::Zero => 0,
            U4BitOffset::One => 1,
            U4BitOffset::Two => 2,
            U4BitOffset::Three => 3,
            U4BitOffset::Four => 4,
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

impl From<u8> for Bit {
    fn from(bit: u8) -> Bit {
        match bit {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            _ => panic!("Invalid bit value: {bit}"),
        }
    }
}

impl From<usize> for Bit {
    fn from(bit: usize) -> Bit {
        debug_assert!(bit <= u8::MAX as usize);
        Bit::from(bit as u8)
    }
}

/// The index of a bit in a byte, starting from the left
pub enum U16Bit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

impl From<&U16Bit> for u8 {
    fn from(bit: &U16Bit) -> u8 {
        match bit {
            U16Bit::Zero => 0,
            U16Bit::One => 1,
            U16Bit::Two => 2,
            U16Bit::Three => 3,
            U16Bit::Four => 4,
            U16Bit::Five => 5,
            U16Bit::Six => 6,
            U16Bit::Seven => 7,
            U16Bit::Eight => 8,
            U16Bit::Nine => 9,
            U16Bit::Ten => 10,
            U16Bit::Eleven => 11,
            U16Bit::Twelve => 12,
            U16Bit::Thirteen => 13,
            U16Bit::Fourteen => 14,
            U16Bit::Fifteen => 15,
        }
    }
}

impl From<u8> for U16Bit {
    fn from(bit: u8) -> U16Bit {
        match bit {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Eleven,
            12 => Self::Twelve,
            13 => Self::Thirteen,
            14 => Self::Fourteen,
            15 => Self::Fifteen,
            _ => panic!("Invalid bit value: {bit}"),
        }
    }
}

pub trait BitExtractor {
    type BitIndex;

    fn get_bit(&self, index: Self::BitIndex) -> bool;
}

impl BitExtractor for u8 {
    type BitIndex = Bit;

    fn get_bit(&self, index: Self::BitIndex) -> bool {
        self & (1 << (7 - index as u8)) > 0
    }
}

impl BitExtractor for u16 {
    type BitIndex = U16Bit;

    fn get_bit(&self, index: Self::BitIndex) -> bool {
        self & (1 << (15 - index as u8)) > 0
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
    fn test_u4_from_u8() {
        assert_eq!(0, u4_from_u8(0, U4BitOffset::Zero));
        assert_eq!(0, u4_from_u8(0, U4BitOffset::Three));

        let byte = 0b1010_0011;
        assert_eq!(0b000_1010, u4_from_u8(byte, U4BitOffset::Zero));
        assert_eq!(0b000_0100, u4_from_u8(byte, U4BitOffset::One));
        assert_eq!(0b000_1000, u4_from_u8(byte, U4BitOffset::Two));
        assert_eq!(0b000_0001, u4_from_u8(byte, U4BitOffset::Three));
        assert_eq!(0b000_0011, u4_from_u8(byte, U4BitOffset::Four));
    }

    #[test]
    fn test_get_bit() {
        let byte = 0b1010_0011;
        assert!(get_bit(byte, Bit::Zero));
        assert!(!get_bit(byte, Bit::One));
        assert!(get_bit(byte, Bit::Two));
        assert!(!get_bit(byte, Bit::Three));
        assert!(!get_bit(byte, Bit::Four));
        assert!(!get_bit(byte, Bit::Five));
        assert!(get_bit(byte, Bit::Six));
        assert!(get_bit(byte, Bit::Seven));
    }
}
