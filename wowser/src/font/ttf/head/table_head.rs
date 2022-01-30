use crate::font::{
    ttf::{F16dot16, FWord, LongDateTime},
    FontError,
};

use super::{DirectionHint, Flags, IndexToLocationFormat, MacStyle};

pub struct TableHead {
    pub version_major: u16,
    pub version_minor: u16,
    pub font_revision: F16dot16,
    pub check_sum_adjustment: u32,
    pub magic_number: u32,
    pub flags: Flags,
    pub units_per_em: u16,
    pub created: LongDateTime,
    pub modified: LongDateTime,
    pub x_min: FWord,
    pub y_min: FWord,
    pub x_max: FWord,
    pub y_max: FWord,
    pub mac_style: MacStyle,
    pub lowest_rec_ppem: u16,
    pub font_direction_hint: DirectionHint,
    pub index_to_location_format: IndexToLocationFormat,
    pub glyph_data_format: i16,
}

impl TableHead {
    pub fn new(bytes: &[u8]) -> Result<Self, FontError> {
        let mut offset = 0;

        let version_major = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let version_minor = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let font_revision = F16dot16 {
            val: u32::from_be_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ]),
        };
        offset += 4;

        let check_sum_adjustment = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;

        let magic_number = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;

        if magic_number != 0x5F0F3CF5 {
            return Err(format!("Invalid magic number value: {magic_number}").into());
        }

        let flags = Flags::new(u16::from_be_bytes([bytes[offset], bytes[offset + 1]]))?;
        offset += 2;

        let units_per_em = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let created = i64::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
            bytes[offset + 4],
            bytes[offset + 5],
            bytes[offset + 6],
            bytes[offset + 7],
        ]);
        offset += 8;

        let modified = i64::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
            bytes[offset + 4],
            bytes[offset + 5],
            bytes[offset + 6],
            bytes[offset + 7],
        ]);
        offset += 8;

        let x_min = i16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let y_min = i16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let x_max = i16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let y_max = i16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let mac_style = MacStyle::new(u16::from_be_bytes([bytes[offset], bytes[offset + 1]]));
        offset += 2;

        let lowest_rec_ppem = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let font_direction_hint =
            DirectionHint::new(i16::from_be_bytes([bytes[offset], bytes[offset + 1]]))?;
        offset += 2;

        let index_to_location_format =
            IndexToLocationFormat::new(i16::from_be_bytes([bytes[offset], bytes[offset + 1]]))?;
        offset += 2;

        let glyph_data_format = i16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        if glyph_data_format != 0 {
            return Err(format!("Invalid glyph data format: {glyph_data_format}").into());
        }

        if offset != bytes.len() {
            return Err("Failed parsing the entirety of the head table".into());
        }

        Ok(TableHead {
            version_major,
            version_minor,
            font_revision,
            check_sum_adjustment,
            magic_number,
            flags,
            units_per_em,
            created,
            modified,
            x_min,
            y_min,
            x_max,
            y_max,
            mac_style,
            lowest_rec_ppem,
            font_direction_hint,
            index_to_location_format,
            glyph_data_format,
        })
    }
}
