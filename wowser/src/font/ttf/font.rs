use crate::{
    font::{ttf::glyf, RenderedCharacter, TableLoca, TableMaxp, TtfTable},
    util::Point,
};

use super::{
    super::{Font, FontError},
    head, ScalarType,
};

#[allow(dead_code)]
pub struct Ttf {
    bytes: Vec<u8>,
    scalar_type: ScalarType,
    num_tables: u16,
    search_range: u16,
    entry_selector: u16,
    range_shift: u16,
    tables: Vec<TtfTable>,
}

impl Ttf {
    pub fn load(bytes: Vec<u8>) -> Result<Ttf, FontError> {
        if bytes.len() < 12 {
            return Err("Not enough bytes".into());
        }

        let mut offset = 0;
        let scalar_type = ScalarType::new(&bytes[..4])?;
        offset += 4;
        let num_tables = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;
        let search_range = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;
        let entry_selector = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;
        let range_shift = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let mut tables = vec![];
        for _ in 0..num_tables {
            let (table, table_len) = TtfTable::new(&bytes[offset..])?;
            tables.push(table);
            offset += table_len;
        }

        Ok(Ttf {
            bytes,
            scalar_type,
            num_tables,
            search_range,
            entry_selector,
            range_shift,
            tables,
        })
    }

    fn get_table_bytes(&self, table_tag: &str) -> Option<&[u8]> {
        let table = self.tables.iter().find(|table| table.tag == table_tag)?;
        let offset = table.offset as usize;
        let length = table.length as usize;
        Some(&self.bytes[offset..offset + length])
    }
}

impl Font for Ttf {
    fn render_character(&self, _character: char) -> Option<RenderedCharacter> {
        let table_head = head::TableHead::new(self.get_table_bytes("head")?).ok()?;
        let table_maxp = TableMaxp::new(self.get_table_bytes("maxp")?).ok()?;
        let _table_loca =
            TableLoca::new(self.get_table_bytes("loca")?, &table_head, &table_maxp).unwrap();
        let _table_glyf = glyf::TableGlyf::new(self.get_table_bytes("glyf")?, &table_maxp).unwrap();

        // Character rendering not working yet
        Some(RenderedCharacter {
            bitmap: vec![],
            width: 0_f32,
            offset: Point { x: 0_f32, y: 0_f32 },
            next_char_offset: 0_f32,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Ttf;
    use crate::font::api::Font;

    #[test]
    fn test_load() {
        let ttf_bytes = include_bytes!("../../../data/Italianno.ttf");
        let ttf = Ttf::load(ttf_bytes.to_vec()).unwrap();
        ttf.render_character('a').unwrap();
    }
}
