use std::io::Write;

use crate::util::{u8_arr_to_u16, u8_to_i32, u8_to_u32};

use super::bitmap_compression_method::BitmapCompressionMethod;

#[derive(Debug)]
pub struct BitmapInfoHeader {
    pub header_size: u32,
    pub width: i32,
    pub height: i32,
    pub color_planes: u16,
    pub bits_per_pixel: u16,
    pub compression_method: BitmapCompressionMethod,
    pub image_size: u32,
    pub h_resolution: i32,
    pub v_resolution: i32,
    pub num_of_colors: u32,
    pub num_of_important_colors: u32,
}

impl BitmapInfoHeader {
    pub fn parse(bytes: &[u8]) -> Result<BitmapInfoHeader, String> {
        if bytes.len() != 40 {
            return Err("Unexpected number of bytes provided to parse bitmap info header".into());
        }

        let header_size = &bytes[0..4];
        let bitmap_width = &bytes[4..8];
        let bitmap_height = &bytes[8..12];
        let color_planes = &bytes[12..14];
        let bits_per_pixel = &bytes[14..16];
        let compression_method = &bytes[16..20];
        let image_size = &bytes[20..24];
        let h_resolution = &bytes[24..28];
        let v_resolution = &bytes[28..32];
        let num_of_colors = &bytes[32..36];
        let num_of_important_colors = &bytes[36..40];

        let header_size = u8_to_u32(
            header_size[3],
            header_size[2],
            header_size[1],
            header_size[0],
        );

        if header_size != 40 {
            return Err(format!("Header of unexpected length: {header_size}"));
        }

        Ok(BitmapInfoHeader {
            header_size,
            width: u8_to_i32(
                bitmap_width[3],
                bitmap_width[2],
                bitmap_width[1],
                bitmap_width[0],
            ),
            height: u8_to_i32(
                bitmap_height[3],
                bitmap_height[2],
                bitmap_height[1],
                bitmap_height[0],
            ),
            color_planes: u8_arr_to_u16(color_planes[1], color_planes[0]),
            bits_per_pixel: u8_arr_to_u16(bits_per_pixel[1], bits_per_pixel[0]),
            compression_method: BitmapCompressionMethod::deserialize(u8_to_u32(
                compression_method[3],
                compression_method[2],
                compression_method[1],
                compression_method[0],
            ))
            .ok_or("Invalid Compression Method values")?,
            image_size: u8_to_u32(image_size[3], image_size[2], image_size[1], image_size[0]),
            h_resolution: u8_to_i32(
                h_resolution[3],
                h_resolution[2],
                h_resolution[1],
                h_resolution[0],
            ),
            v_resolution: u8_to_i32(
                v_resolution[3],
                v_resolution[2],
                v_resolution[1],
                v_resolution[0],
            ),
            num_of_colors: u8_to_u32(
                num_of_colors[3],
                num_of_colors[2],
                num_of_colors[1],
                num_of_colors[0],
            ),
            num_of_important_colors: u8_to_u32(
                num_of_important_colors[3],
                num_of_important_colors[2],
                num_of_important_colors[1],
                num_of_important_colors[0],
            ),
        })
    }

    pub fn write(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        writer.write_all(&self.header_size.to_le_bytes())?;
        writer.write_all(&self.width.to_le_bytes())?;
        writer.write_all(&self.height.to_le_bytes())?;
        writer.write_all(&self.color_planes.to_le_bytes())?;
        writer.write_all(&self.bits_per_pixel.to_le_bytes())?;
        writer.write_all(&(Into::<u32>::into(self.compression_method.clone())).to_le_bytes())?;
        writer.write_all(&self.image_size.to_le_bytes())?;
        writer.write_all(&self.h_resolution.to_le_bytes())?;
        writer.write_all(&self.v_resolution.to_le_bytes())?;
        writer.write_all(&self.num_of_colors.to_le_bytes())?;
        writer.write_all(&self.num_of_important_colors.to_le_bytes())?;
        Ok(())
    }
}
