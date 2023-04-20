//! See <https://en.wikipedia.org/wiki/BMP_file_format> and
// <https://www-user.tu-chemnitz.de/~heha/petzold/ch15b.htm> for details on the format

use std::{collections::HashSet, io::Write};

use super::bitmap_compression_method::BitmapCompressionMethod;
use super::bitmap_header::BitmapHeader;
use super::bitmap_info_header::BitmapInfoHeader;
use crate::{
    render::Color,
    util::{get_bit, u4_from_u8, Bit, U4BitOffset},
};

#[derive(Debug, PartialEq)]
pub struct Bitmap {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Bitmap {
    pub fn new(bytes: &[u8]) -> Result<Bitmap, String> {
        let header = BitmapHeader::parse(bytes)?;

        let bitmap_info_header = &header.bitmap_info_header;

        let bitmask_length = match bitmap_info_header.compression_method {
            BitmapCompressionMethod::BitFields => 12,
            BitmapCompressionMethod::AlphaBitFields => 16,
            _ => 0,
        };

        if bitmap_info_header.compression_method != BitmapCompressionMethod::Rgb {
            return Err(format!(
                "Currently unsupported bitmap compression method: {:?}",
                bitmap_info_header.compression_method
            ));
        }
        let init_header_size = 14;
        let color_table_offset =
            (init_header_size + bitmap_info_header.header_size + bitmask_length) as usize;

        // TODO check for color table if `num_of_colors > 0`
        if bitmap_info_header.bits_per_pixel <= 8 {
            let (colors, color_table_length) = parse_color_table(
                &bytes[color_table_offset..],
                bitmap_info_header.bits_per_pixel,
            )?;

            let pixel_bits_offset = color_table_offset + color_table_length;
            assert_eq!(pixel_bits_offset, header.pixel_offset as usize);
            let pixels = get_pixels(
                &bytes[pixel_bits_offset..],
                &colors,
                bitmap_info_header.bits_per_pixel,
                bitmap_info_header.width,
                bitmap_info_header.height,
            )?;

            return Ok(Bitmap {
                width: bitmap_info_header.width as usize,
                height: bitmap_info_header.height.unsigned_abs() as usize,
                pixels,
            });
        } else if bitmap_info_header.bits_per_pixel == 24 {
            let color_table_length = 0;
            let pixel_bits_offset = color_table_offset + color_table_length;
            assert_eq!(pixel_bits_offset, header.pixel_offset as usize);

            let pixels = get_24bit_pixels(
                &bytes[pixel_bits_offset..],
                bitmap_info_header.width,
                bitmap_info_header.height,
            )?;

            return Ok(Bitmap {
                width: bitmap_info_header.width as usize,
                height: bitmap_info_header.height as usize,
                pixels,
            });
        }

        Err(format!("Unsupported Bitmap: {:?}", header))
    }

    pub fn write(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        let pixel_set: HashSet<&Color> = self.pixels.iter().collect();

        // These headers have fixed lengths
        let bh_size = 14;
        let bih_size = 40;
        let pixel_bytes = (self.width * self.height * 3) as u32;
        // Pixel rows must be multiples of 4 bytes
        let pixel_shortage = (self.width * 3) % 4;
        let pixel_padding = (self.height
            * (if pixel_shortage > 0 {
                4 - pixel_shortage
            } else {
                // If it perfectly lines up, avoid adding an unnecessary padding of 4 bytes
                0
            })) as u32;
        let file_size = bh_size + bih_size + pixel_bytes + pixel_padding;

        let header = BitmapHeader {
            id: "BM".into(),
            size: file_size,
            reserved_chunk_a: vec![0, 0],
            reserved_chunk_b: vec![0, 0],
            pixel_offset: bh_size + bih_size,
            bitmap_info_header: BitmapInfoHeader {
                header_size: bih_size,
                width: self.width as i32,
                height: self.height as i32,
                color_planes: 1, // Bitmaps only ever have one color plane
                bits_per_pixel: 24,
                compression_method: BitmapCompressionMethod::Rgb,
                image_size: 0, // Can be computed by parser
                h_resolution: 0,
                v_resolution: 0,
                num_of_colors: pixel_set.len() as u32,
                num_of_important_colors: 0,
            },
        };

        header.write(writer)?;

        let mut pixel_offset = 0;

        for pixel in &self.pixels {
            writer.write_all(&[pixel.b, pixel.g, pixel.r])?;
            pixel_offset += 3;
            if pixel_offset == self.width * 3 {
                let row_offset = (4 - pixel_offset % 4) % 4;
                if row_offset != 0 {
                    writer.write_all(&vec![0; row_offset])?;
                }
                pixel_offset = 0;
            }
        }

        Ok(())
    }
}

fn parse_color_table(bytes: &[u8], bits_per_pixel: u16) -> Result<(Vec<Color>, usize), String> {
    let num_of_colors = 1 << bits_per_pixel;
    let mut colors = Vec::with_capacity(num_of_colors);
    for pixel in bytes.chunks_exact(4) {
        colors.push(Color {
            b: pixel[0],
            g: pixel[1],
            r: pixel[2],
            a: 255, // 4th byte should be 0x00
        });
    }
    Ok((colors, num_of_colors * 4))
}

fn get_24bit_pixels(bytes: &[u8], width: i32, height: i32) -> Result<Vec<Color>, String> {
    let mut pixels = Vec::with_capacity((height * width) as usize);
    let mut byte_offset = 0;
    for _y in 0..height as usize {
        for _x in 0..width as usize {
            pixels.push(Color {
                b: bytes[byte_offset],
                g: bytes[byte_offset + 1],
                r: bytes[byte_offset + 2],
                a: 255,
            });
            byte_offset += 3;
        }
        // Each row is padded to a multiple of 4 bytes
        if byte_offset % 4 != 0 {
            byte_offset += 4 - (byte_offset % 4);
        }
    }
    Ok(pixels)
}

fn get_pixels(
    bytes: &[u8],
    colors: &[Color],
    bits_per_pixel: u16,
    width: i32,
    height: i32,
) -> Result<Vec<Color>, String> {
    let mut pixels = Vec::with_capacity((height * width) as usize);
    let mut byte_offset = 0;
    for _y in 0..height.unsigned_abs() as usize {
        for x in 0..width as usize {
            let color_offset = bytes[byte_offset];
            if bits_per_pixel == 8 {
                pixels.push(colors[color_offset as usize]);
                byte_offset += 1;
            } else if bits_per_pixel == 4 {
                let byte_offset = if x % 2 == 0 {
                    U4BitOffset::Zero
                } else {
                    byte_offset += 1;
                    U4BitOffset::Four
                };

                let specific_color_offset = u4_from_u8(color_offset, byte_offset);
                pixels.push(colors[specific_color_offset as usize]);
            } else if bits_per_pixel == 1 {
                let bit_offset = (x % 8) as u8;
                let bit = get_bit(color_offset, Bit::from(bit_offset));
                let specific_color_offset = if bit { 1 } else { 0 };

                pixels.push(colors[specific_color_offset]);

                if bit_offset == 7 {
                    byte_offset += 1;
                }
            } else {
                panic!("Not yet supported");
            }
        }
        // Each row is padded to a multiple of 4 bytes, even in the silly case of a 1-bit per pixel
        // image with less than 8 pixels per row. Each row must start at the beginning of a byte and
        // each row must take up at least 4 bytes due to padding.
        byte_offset += 4 - (byte_offset % 4);
    }

    // Flip rows if negative
    if height < 0 {
        let mut flipped_pixels = Vec::with_capacity(pixels.len());
        for row in pixels.chunks_exact(width as usize).rev() {
            flipped_pixels.extend(row.to_owned());
        }
        pixels = flipped_pixels;
    }

    Ok(pixels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mono_bit() {
        // Most tools force monochrome to be black+white. There is technically a tiny color pallete that
        // should support two specific colors.
        let bitmap = Bitmap::new(include_bytes!("test_data/min-mono.bmp")).unwrap();
        assert_eq!(2, bitmap.height);
        assert_eq!(3, bitmap.width);
        assert_eq!(
            vec![
                Color::BLUE,
                Color::BLUE,
                Color::RED,
                Color::BLUE,
                Color::RED,
                Color::RED,
            ],
            bitmap.pixels
        );
    }

    #[test]
    fn test_16_bit() {
        let bitmap = Bitmap::new(include_bytes!("test_data/min-16.bmp")).unwrap();
        assert_eq!(2, bitmap.height);
        assert_eq!(3, bitmap.width);
        assert_eq!(
            vec![
                Color::rgb(128, 128, 128),
                Color::rgb(192, 192, 192),
                Color::rgb(0, 0, 255),
                Color::rgb(192, 192, 192),
                Color::rgb(255, 0, 0),
                Color::rgb(0, 255, 0),
            ],
            bitmap.pixels
        );
    }

    #[test]
    fn test_24_bit() {
        let bitmap = Bitmap::new(include_bytes!("test_data/min-24.bmp")).unwrap();
        assert_eq!(2, bitmap.height);
        assert_eq!(3, bitmap.width);
        assert_eq!(
            vec![
                Color::rgb(128, 128, 128),
                Color::rgb(224, 160, 192),
                Color::rgb(64, 64, 192),
                Color::rgb(160, 160, 164),
                Color::rgb(224, 32, 64),
                Color::rgb(32, 192, 64),
            ],
            bitmap.pixels
        );
    }

    #[test]
    fn test_256_bit() {
        let bitmap = Bitmap::new(include_bytes!("test_data/min-256.bmp")).unwrap();
        assert_eq!(2, bitmap.height);
        assert_eq!(3, bitmap.width);
        assert_eq!(
            vec![
                Color::rgb(128, 128, 128),
                Color::rgb(224, 160, 192),
                Color::rgb(64, 64, 192),
                Color::rgb(160, 160, 164),
                Color::rgb(224, 32, 64),
                Color::rgb(32, 192, 64),
            ],
            bitmap.pixels
        );
    }

    /// If this test fails and this needs to be updates, manually verify the output bitmap
    /// loads properly in image viewers like MS Paint or VS Code can open it successfully and
    /// that it looks correct. Ideally use `imagemagick identify -regard-warnings` to verify the
    /// image is valid.
    #[test]
    fn test_read_write() {
        let mut output_bytes = vec![];
        Bitmap::new(include_bytes!("test_data/input-marbles.bmp"))
            .unwrap()
            .write(&mut output_bytes)
            .unwrap();

        assert_eq!(
            &output_bytes,
            include_bytes!("test_data/output-marbles.bmp")
        );
    }
}
