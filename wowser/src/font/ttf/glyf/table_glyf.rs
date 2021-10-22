use crate::font::{FontError, TableMaxp};

use super::Glyph;

pub struct TableGlyf {
    pub glyphs: Vec<Glyph>,
}

impl TableGlyf {
    pub fn new(bytes: &[u8], table_maxp: &TableMaxp) -> Result<Self, FontError> {
        let mut offset = 0;

        let mut glyphs = vec![];
        for _ in 0..table_maxp.num_glyphs {
            let (glyph, new_offset) = Glyph::new(&bytes[offset..])?;
            offset = new_offset;
            glyphs.push(glyph);
        }

        Ok(TableGlyf { glyphs })
    }
}
