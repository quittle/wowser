use crate::{
    font::FontError,
    util::{BitExtractor, U16Bit},
};

pub struct Flags {
    value: u16,
}

impl Flags {
    pub fn new(value: u16) -> Result<Self, FontError> {
        if value.get_bit(U16Bit::Six) {
            return Err("Invalid value for 6th flag bit".into());
        }

        Ok(Self { value })
    }

    pub fn get_xpos_of_left_most_black_bit_is_lsb(&self) -> bool {
        self.value.get_bit(U16Bit::One)
    }

    pub fn get_font_requires_layout_for_rendering(&self) -> bool {
        self.value.get_bit(U16Bit::Seven)
    }
}
