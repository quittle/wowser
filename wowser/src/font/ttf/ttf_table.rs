use crate::font::FontError;

pub struct TtfTable {
    pub tag: String,
    pub checksum: u32,
    pub offset: u32,
    pub length: u32,
}

impl TtfTable {
    /// Returns the table and how many bytes from the input were used to construct it.
    pub fn new(bytes: &[u8]) -> Result<(TtfTable, usize), FontError> {
        if 32 > bytes.len() {
            return Err("Not enough bytes for table directory".into());
        }

        let mut offset = 0;

        let tag = std::str::from_utf8(&[
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ])?
        .to_string();
        offset += 4;

        let checksum = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;

        let table_offset = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;

        let length = u32::from_be_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]);
        offset += 4;

        Ok((
            TtfTable {
                tag,
                checksum,
                offset: table_offset,
                length,
            },
            offset,
        ))
    }
}
