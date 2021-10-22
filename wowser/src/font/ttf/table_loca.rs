use crate::font::FontError;

use super::{head, TableMaxp};

pub enum TableLoca {
    Short(Vec<u16>),
    Long(Vec<u32>),
}

impl TableLoca {
    pub fn new(
        bytes: &[u8],
        table_head: &head::TableHead,
        table_maxp: &TableMaxp,
    ) -> Result<Self, FontError> {
        let num_glyphs = table_maxp.num_glyphs as usize;
        let format = &table_head.index_to_location_format;
        let table = match format {
            head::IndexToLocationFormat::Short => TableLoca::Short(
                bytes
                    .chunks_exact(2)
                    .map(|window| u16::from_be_bytes([window[0], window[1]]))
                    .collect(),
            ),
            head::IndexToLocationFormat::Long => TableLoca::Long(
                bytes
                    .chunks_exact(4)
                    .map(|window| u32::from_be_bytes([window[0], window[1], window[2], window[3]]))
                    .collect(),
            ),
        };

        let vec_len = match &table {
            TableLoca::Short(vec) => vec.len(),
            TableLoca::Long(vec) => vec.len(),
        };

        // Last entry is an extra to determine the length of the penultimate entry
        if vec_len != num_glyphs + 1 {
            return Err("Failed to parse the correct number of glyphs in loca table".into());
        }

        Ok(table)
    }
}
