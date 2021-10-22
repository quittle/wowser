use crate::font::{FWord, FontError};

use super::GlyphSpecialization;

pub struct Glyph {
    pub num_of_contours: i16,
    pub x_min: FWord,
    pub y_min: FWord,
    pub x_max: FWord,
    pub y_max: FWord,
    pub glyph_specialization: GlyphSpecialization,
}

impl Glyph {
    pub fn new(bytes: &[u8]) -> Result<(Self, usize), FontError> {
        let mut offset = 0;

        let num_of_contours = i16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let x_min = i16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let y_min = i16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let x_max = i16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let y_max = i16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let (glyph_specialization, offset) =
            GlyphSpecialization::new(&bytes[offset..], num_of_contours)?;

        Ok((
            Glyph {
                num_of_contours,
                x_min,
                y_min,
                x_max,
                y_max,
                glyph_specialization,
            },
            offset,
        ))
    }
}
