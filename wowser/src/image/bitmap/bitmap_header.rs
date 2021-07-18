use crate::util::u8_to_u32;

use super::bitmap_info_header::BitmapInfoHeader;

#[derive(Debug)]
pub struct BitmapHeader {
    pub id: String,
    pub size: u32,
    pub reserved_chunk_a: Vec<u8>,
    pub reserved_chunk_b: Vec<u8>,
    pub pixel_offset: u32,
    pub bitmap_info_header: BitmapInfoHeader,
}

impl BitmapHeader {
    pub fn parse(bytes: &[u8]) -> Result<BitmapHeader, String> {
        let id = &bytes[0..2];
        let size = &bytes[2..6];
        let reserved_chunk_a = &bytes[6..8];
        let reserved_chunk_b = &bytes[8..10];
        let offset = &bytes[10..14];
        let id = String::from_utf8(id.into()).map_err(|err| err.to_string())?;
        let size = u8_to_u32(size[3], size[2], size[1], size[0]);
        let pixel_offset = u8_to_u32(offset[3], offset[2], offset[1], offset[0]);

        let bitmap_info_header = match id.as_ref() {
            "BM" => BitmapInfoHeader::parse(&bytes[14..54]),
            _ => Err(format!("Unsupported bitmap format: {}", id)),
        }?;

        Ok(BitmapHeader {
            id,
            size,
            reserved_chunk_a: reserved_chunk_a.into(),
            reserved_chunk_b: reserved_chunk_b.into(),
            pixel_offset,
            bitmap_info_header,
        })
    }
}
