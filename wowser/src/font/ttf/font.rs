use crate::{
    font::{ttf::glyf, RenderedCharacter, TableLoca, TableMaxp, TtfTable},
    util::{Point, Sqrt},
};

use super::{
    super::{Font, FontError},
    cmap, head, ScalarType,
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

#[allow(dead_code)]
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
    fn render_character(&self, character: char, _point_size: f32) -> Option<RenderedCharacter> {
        let table_head = head::TableHead::new(self.get_table_bytes("head")?).ok()?;
        let table_maxp = TableMaxp::new(self.get_table_bytes("maxp")?).ok()?;
        let _table_cmap = cmap::TableCmap::new(self.get_table_bytes("cmap")?).ok()?;
        let table_loca =
            TableLoca::new(self.get_table_bytes("loca")?, &table_head, &table_maxp).unwrap();
        let table_glyf = glyf::TableGlyf::new(self.get_table_bytes("glyf")?, &table_maxp).unwrap();

        let units_per_em = table_head.units_per_em;
        let funits_per_axis = units_per_em.sqrt()?;
        if funits_per_axis.pow(2) != units_per_em {
            // Possibly off due to rounding, but unlikely. This is an expensive
            // check to verify they units_per_em is valid. An odd number, for instance,
            // could be specified but doesn't fit the required square shape.
            return None;
        }

        let glyph = find_glyph(character, &table_loca, &table_glyf)?;

        // TODO: Get contours
        let _specialization = &glyph.glyph_specialization;

        let mut bits = vec![];
        for x in 0..funits_per_axis {
            for y in 0..funits_per_axis {
                let transitions = compute_transitions_counts(x, y, vec![]);
                if transitions % 2 == 0 {
                    bits.push(false);
                } else {
                    bits.push(true);
                }
            }
        }

        // Character rendering not working yet
        Some(RenderedCharacter {
            bitmap: vec![],
            width: 0_f32,
            offset: Point { x: 0_f32, y: 0_f32 },
            next_char_offset: 0_f32,
        })
    }
}

fn char_offset<T>(v: &[T], char_id: usize) -> usize
where
    T: Into<u32>,
    T: PartialEq,
    T: Copy,
{
    let mut last_offset = 0_u32;
    let mut glyf_index: i32 = -1;

    for offset in &v[..char_id] {
        let offset_u32 = (*offset).into();
        if offset_u32 != last_offset {
            last_offset = offset_u32;
            glyf_index += 1;
        }
    }

    glyf_index as usize
}

fn find_glyph<'glyf>(
    character: char,
    table_loca: &TableLoca,
    table_glyf: &'glyf glyf::TableGlyf,
) -> Option<&'glyf glyf::Glyph> {
    let char_id = character as usize;
    let loca_offset = match table_loca {
        TableLoca::Long(v) => char_offset(v, char_id),
        TableLoca::Short(v) => char_offset(v, char_id),
    };

    table_glyf.glyphs.get(loca_offset)
}

fn compute_transitions_counts(_x: u16, _y: u16, _contours: Vec<u8>) -> u8 {
    0
}

/// Just a magic number based on convention inherited from print units
/// https://docs.microsoft.com/en-us/typography/opentype/spec/ttch01
#[allow(dead_code)]
const POINTS_PER_INCH: u16 = 72;

/// points_per_inch is dpi
fn _scale_coordinate(
    funit_coordinate_value: u16,
    point_size: f32,
    resolution_dpi: u16,
    units_per_em: u16,
) -> f32 {
    (funit_coordinate_value as f32 * point_size * resolution_dpi as f32)
        / (POINTS_PER_INCH * units_per_em) as f32
}

fn _pixels_per_em(point_size: f32, resolution_dpi: u16) -> u16 {
    (point_size * resolution_dpi as f32 / POINTS_PER_INCH as f32) as u16
}

fn _pixel_coordinate(funit_coordinate_value: u16, pixels_per_em: u16, funits_per_em: u16) -> u16 {
    funit_coordinate_value * pixels_per_em / funits_per_em
}

#[cfg(test)]
mod tests {
    use super::Ttf;
    use crate::font::api::Font;

    #[test]
    fn test_load() {
        let ttf_bytes = include_bytes!("../../../data/Italianno.ttf");
        let ttf = Ttf::load(ttf_bytes.to_vec()).unwrap();
        ttf.render_character('a', 12.0);
    }
}
