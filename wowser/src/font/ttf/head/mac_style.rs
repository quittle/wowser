use crate::util::{BitExtractor, U16Bit};

pub struct MacStyle {
    value: u16,
}

impl MacStyle {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn is_bold(&self) -> bool {
        self.value.get_bit(U16Bit::Zero)
    }

    pub fn is_italic(&self) -> bool {
        self.value.get_bit(U16Bit::One)
    }

    pub fn is_underline(&self) -> bool {
        self.value.get_bit(U16Bit::Two)
    }

    pub fn is_outline(&self) -> bool {
        self.value.get_bit(U16Bit::Three)
    }

    pub fn is_shadow(&self) -> bool {
        self.value.get_bit(U16Bit::Four)
    }

    pub fn is_condensed(&self) -> bool {
        self.value.get_bit(U16Bit::Five)
    }

    pub fn is_extended(&self) -> bool {
        self.value.get_bit(U16Bit::Six)
    }
}
