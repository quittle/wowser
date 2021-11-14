use super::Platform;

pub struct EncodingRecord {
    pub platform_id: Platform,
    pub encoding_id: u16,
    pub subtable_offset: u32,
}
