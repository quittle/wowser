use super::{EncodingRecord, Platform};
use crate::font::FontError;

pub struct TableCmap {
    pub version: u16,
    pub records: Vec<EncodingRecord>,
}

impl TableCmap {
    pub fn new(bytes: &[u8]) -> Result<Self, FontError> {
        let mut offset = 0;

        let version = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        if version != 0 {
            return Err(format!("Unexpected cmap version: {version}").into());
        }

        let num_tables = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let mut records = vec![];
        for _ in 0..num_tables {
            let (record, offset_delta) = Self::new_encoding_record(&bytes[offset..])?;
            records.push(record);
            offset += offset_delta;
        }

        Ok(Self { version, records })
    }

    fn new_encoding_record(bytes: &[u8]) -> Result<(EncodingRecord, usize), FontError> {
        let mut offset = 0;

        let platform_id = Platform::new(u16::from_be_bytes([bytes[offset], bytes[offset + 1]]));
        offset += 2;

        let encoding_id = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let subtable_offset = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;

        Ok((
            EncodingRecord {
                platform_id,
                encoding_id,
                subtable_offset,
            },
            offset,
        ))
    }
}
