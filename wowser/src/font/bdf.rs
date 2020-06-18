use super::{Font, FontError, RenderedCharacter};
use crate::util::string_to_bytes;
use std::io::{BufRead, BufReader, Lines};
use std::{borrow::Cow, cmp, iter, str::FromStr};

#[derive(Debug, Default)]
pub struct BDFFont {
    pub version: Option<f32>,
    pub name: Option<String>,
    pub size: Option<BDFPropertySize>,
    pub properties: Option<BDFRealProperties>,
    pub characters: Option<Vec<BDFCharacter>>,
}

#[derive(Debug, Default)]
pub struct BDFCharacter {
    pub name: Option<String>,
    /// The character code point represented
    pub encoding: Option<u32>,
    pub s_width: Option<(u32, u32)>,
    pub d_width: Option<(u32, u32)>,
    pub bbx: Option<(i32, i32, i32, i32)>,
    /// The outer vector represents the rows.
    /// The inner vector represents the columns. Columns are right padded to fill out the full byte.
    /// Each bit represents a single pixel.
    nested_bitmap: Option<Vec<Vec<u8>>>,
    /// The final represedntation of the bitmap as a flattend vector of bytes
    /// wrapped at `d_width` columns.
    pub bitmap: Option<Vec<u8>>,
}

#[derive(Debug, Default)]
pub struct BDFRealProperties {
    copyright: Option<String>,
}

#[derive(Debug)]
pub struct BDFPropertySize {
    point_size: u32,
    x_resolution: u32,
    y_resolution: u32,
}

impl BDFFont {
    pub fn load(bytes: &[u8]) -> Result<BDFFont, FontError> {
        let reader = BufReader::new(bytes);

        let mut lines = reader.lines();

        parse_bdf_font(&mut lines)
    }
}

impl Font for BDFFont {
    fn render_character(&self, character: char) -> Option<RenderedCharacter<'_>> {
        for c in self.characters.as_deref()? {
            if c.encoding == Some(character as u32) {
                return Some(RenderedCharacter {
                    bitmap: Cow::from(c.bitmap.as_deref()?),
                    width: c.d_width?.0,
                    next_char_offset: c.d_width?.0,
                });
            }
        }
        None
    }
}

fn next_line(lines: &mut Lines<BufReader<&[u8]>>) -> Result<Option<String>, FontError> {
    Ok(lines.next().transpose()?)
}

fn parse_bdf_font(lines: &mut Lines<BufReader<&[u8]>>) -> Result<BDFFont, FontError> {
    let mut font = BDFFont::default();
    while let Some(line) = next_line(lines)? {
        let mut parts = line.splitn(2, ' ');
        let property_name = parts.next().ok_or_else(|| "Missing name of property line")?;
        let property_value_literal = parts.next().ok_or_else(|| "Missing value")?;
        match property_name {
            "STARTFONT" => font.version = Some(f32::from_str(property_value_literal)?),
            "FONT" => font.name = Some(property_value_literal.to_string()),
            "SIZE" => {
                let mut parts = property_value_literal.splitn(3, ' ');
                let point_size = parts.next().ok_or_else(|| "Missing point size")?;
                let x_resolution = parts.next().ok_or_else(|| "Missing x resolution")?;
                let y_resolution = parts.next().ok_or_else(|| "Missing y resolution")?;

                font.size = Some(BDFPropertySize {
                    point_size: u32::from_str_radix(point_size, 10)?,
                    x_resolution: u32::from_str_radix(x_resolution, 10)?,
                    y_resolution: u32::from_str_radix(y_resolution, 10)?,
                })
            }
            "COMMENT" => { /* Skip */ }
            "STARTPROPERTIES" => {
                let _property_count = u32::from_str_radix(property_value_literal, 10)?;
                let real_properties = parse_real_properties(lines)?;
                // TODO: Verify the right number of properties were found.
                font.properties = Some(real_properties)
            }
            "CHARS" => {
                let char_count = u32::from_str_radix(property_value_literal, 10)?;
                let chars = parse_chars(lines)?;
                if chars.len() != char_count as usize {
                    return Err("Unexpected number of characters found".into());
                }

                font.characters = Some(chars);
            }
            _ => println!("Unknown property {}", line),
        };
    }

    Ok(font)
}

fn parse_bitmap(lines: &mut Lines<BufReader<&[u8]>>) -> Result<Vec<Vec<u8>>, FontError> {
    let mut bytes: Vec<Vec<u8>> = vec![];
    while let Some(line) = next_line(lines)? {
        if line == "ENDCHAR" {
            break;
        }

        bytes.push(string_to_bytes(&line)?);
    }
    Ok(bytes)
}

fn parse_char(
    lines: &mut Lines<BufReader<&[u8]>>,
    first_line: &str,
) -> Result<BDFCharacter, FontError> {
    let mut character = BDFCharacter::default();
    let name = first_line.splitn(2, ' ').nth(1).ok_or("Missing character name")?;
    character.name = Some(name.to_string());
    while let Some(line) = next_line(lines)? {
        if line == "ENDCHAR" {
            break;
        }

        let mut parts = line.splitn(2, ' ');
        let property_name = parts.next().ok_or_else(|| "Missing name of property line")?;
        let property_value_literal = parts.next();

        match property_name {
            "BITMAP" => {
                character.nested_bitmap = Some(parse_bitmap(lines)?);
                break;
            }
            _ => {
                let property_value_literal =
                    property_value_literal.ok_or_else(|| "Missing encoding value")?;
                match property_name {
                    "ENCODING" => {
                        character.encoding = Some(u32::from_str_radix(property_value_literal, 10)?)
                    }
                    "DWIDTH" => {
                        let mut entries = property_value_literal.splitn(2, ' ');
                        let d_width_x = entries
                            .next()
                            .ok_or_else(|| "Missing DWIDTH x")
                            .map(|v| u32::from_str_radix(v, 10))??;
                        let d_width_y = entries
                            .next()
                            .ok_or_else(|| "Missing DWIDTH y")
                            .map(|v| u32::from_str_radix(v, 10))??;

                        character.d_width = Some((d_width_x, d_width_y))
                    }
                    "BBX" => {
                        let mut entries = property_value_literal.splitn(4, ' ');
                        let width = entries
                            .next()
                            .ok_or_else(|| "Missing BBX width")
                            .map(|v| i32::from_str_radix(v, 10))??;
                        let height = entries
                            .next()
                            .ok_or_else(|| "Missing BBX height")
                            .map(|v| i32::from_str_radix(v, 10))??;
                        let x = entries
                            .next()
                            .ok_or_else(|| "Missing BBX x")
                            .map(|v| i32::from_str_radix(v, 10))??;
                        let y = entries
                            .next()
                            .ok_or_else(|| "Missing BBX y")
                            .map(|v| i32::from_str_radix(v, 10))??;
                        character.bbx = Some((width, height, x, y))
                    }
                    "SWIDTH" => {}
                    _ => println!("Unexpected character property {}", line),
                }
            }
        }
    }
    let nested_bitmap = character.nested_bitmap.ok_or("BITMAP not provided for character")?;
    let d_width_x = character.d_width.ok_or("DWIDTH not provided for character")?.0;

    let mut bitmap = Vec::with_capacity(d_width_x as usize * cmp::max(1, nested_bitmap.len()));

    for row in nested_bitmap.iter().rev() {
        bitmap.extend(row);
        if row.len() < d_width_x as usize {
            bitmap.extend(iter::repeat(0_u8).take(d_width_x as usize - row.len()));
        }
    }
    character.bitmap = Some(bitmap);
    character.nested_bitmap = None;
    Ok(character)
}

#[allow(dead_code)]
fn split_str_into_2<'a, T>(
    s: &'a str,
    pattern: &str,
    transform: &dyn Fn(&str) -> T,
    missing_message: &str,
) -> Result<(T, T), FontError> {
    let mut split = s.splitn(2, pattern);
    Ok((
        transform(split.next().ok_or_else(|| missing_message)?),
        transform(split.next().ok_or_else(|| missing_message)?),
    ))
}

fn parse_chars(lines: &mut Lines<BufReader<&[u8]>>) -> Result<Vec<BDFCharacter>, FontError> {
    let mut ret = vec![];

    while let Some(line) = next_line(lines)? {
        if line == "ENDFONT" {
            break;
        } else if line.starts_with("STARTCHAR") {
            let char = parse_char(lines, &line)?;
            ret.push(char);
        } else {
            return Err(format!("Invalid font file. Unexpected line '{}'", line).into());
        }
    }

    Ok(ret)
}

fn parse_real_properties(
    lines: &mut Lines<BufReader<&[u8]>>,
) -> Result<BDFRealProperties, FontError> {
    let mut ret = BDFRealProperties::default();
    while let Some(line) = next_line(lines)? {
        if line == "ENDPROPERTIES" {
            break;
        }

        let mut parts = line.splitn(2, ' ');
        let property_name = parts.next().ok_or_else(|| "Missing name of property line")?;
        let property_value_literal = parts.next().ok_or_else(|| "Missing property value")?;

        match property_name {
            "COPYRIGHT" => ret.copyright = Some(property_value_literal.to_string()),
            _ => println!("Dropping property {}", line),
        }
    }
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_bdf() {
        let bytes = include_bytes!("../../data/unifont-13.0.02.bdf");
        let font = BDFFont::load(bytes).expect("Expected font to load");
        assert_eq!(font.version, Some(2.1));
        assert_eq!(
            font.name,
            Some("-gnu-Unifont-Medium-R-Normal-Sans-16-160-75-75-c-80-iso10646-1".to_string())
        );
        font.properties.expect("Expecte propertiers");
    }
}
