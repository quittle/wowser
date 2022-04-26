use crate::util::{BitExtractor, Point};

pub trait Font {
    fn render_character(&self, character: char, point_size: f32) -> Option<RenderedCharacter>;
}

/// A bitmap binary rendering of a character, each byte in the bitmap represents 8 horizontal pixels.
#[derive(Debug, Clone, PartialEq)]
pub struct RenderedCharacter {
    /// Bit-packed bytes of on/off pixels
    pub bitmap: Vec<u8>,
    /// The width of the bitmap in bytes
    pub width: f32,
    /// The offset from where the character should be drawn to actually draw thebitmap
    pub offset: Point<f32>,
    /// The offset, in pixels, from where the character should be drawn to place the next character.
    /// The width should generally be ignored for layout calculations.
    pub next_char_offset: f32,
}

impl RenderedCharacter {
    pub fn render_pixels_to_string(&self) -> String {
        if self.width == 0.0 || self.bitmap.is_empty() {
            return String::new();
        }

        let capacity =
            // Length * bits * 3 bytes per character for █
            self.bitmap.len() * 8 * 3
            // Include newlines
            + self.bitmap.len() / (self.width as usize)
            // Exclude newline at beginning
            - if self.bitmap.is_empty() {0} else {1};
        let mut ret = String::with_capacity(capacity);
        for (index, byte) in self.bitmap.iter().enumerate() {
            if index != 0 && index % (self.width as usize) == 0 {
                ret.push('\n');
            }
            for bit in 0_u8..8 {
                ret.push(if byte.get_bit(bit.into()) { '█' } else { ' ' });
            }
        }
        debug_assert!(ret.len() <= capacity, "Initial capacity calculation is off");
        ret
    }

    pub fn flip_vertically(&self) -> RenderedCharacter {
        RenderedCharacter {
            bitmap: self
                .bitmap
                .chunks(self.width as usize)
                .rev()
                .flatten()
                .copied()
                .collect(),
            width: self.width,
            offset: self.offset.clone(),
            next_char_offset: self.next_char_offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Point;

    use super::RenderedCharacter;

    #[rustfmt::skip]
    const BITMAP_BYTES: &[u8] = &[
        174, 85,
        228, 37,
        174, 39,
    ];

    #[rustfmt::skip]
    const BITMAP_STR: &str =
"█ █ ███  █ █ █ █
███  █    █  █ █
█ █ ███   █  ███";

    #[test]
    fn test_render_pixels_to_string_empty() {
        let char = RenderedCharacter {
            bitmap: vec![],
            width: 0.0,
            offset: Point { x: 0.0, y: 0.0 },
            next_char_offset: 0.0,
        };
        assert_eq!(char.render_pixels_to_string(), { String::new() })
    }

    #[test]
    fn test_render_pixels_to_string_single_line() {
        let char = RenderedCharacter {
            bitmap: vec![170],
            width: 1.0,
            offset: Point { x: 0.0, y: 0.0 },
            next_char_offset: 0.0,
        };
        assert_eq!(char.render_pixels_to_string(), String::from("█ █ █ █ "))
    }

    #[test]
    fn test_render_pixels_to_string() {
        let char = RenderedCharacter {
            bitmap: Vec::from(BITMAP_BYTES),
            width: 2.0,
            offset: Point { x: 0.0, y: 0.0 },
            next_char_offset: 0.0,
        };
        assert_eq!(char.render_pixels_to_string(), String::from(BITMAP_STR))
    }

    #[test]
    fn test_flip_vertically() {
        let char = RenderedCharacter {
            bitmap: Vec::from(BITMAP_BYTES),
            width: 2.0,
            offset: Point { x: 0.0, y: 0.0 },
            next_char_offset: 0.0,
        };
        let flipped = char.flip_vertically();
        assert_eq!(
            flipped,
            RenderedCharacter {
                #[rustfmt::skip]
                bitmap: vec![
                    174, 39,
                    228, 37,
                    174, 85,
                ],
                width: 2.0,
                offset: Point { x: 0.0, y: 0.0 },
                next_char_offset: 0.0,
            },
            "Flipped character should look like 'HI YU' upside-down but instead is:\n{}",
            flipped.render_pixels_to_string()
        );
    }
}
