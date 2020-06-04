use super::FontError;
use crate::util::string_to_bytes;
use std::io::{BufRead, BufReader, Lines};
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct BDFFont {
    version: Option<f32>,
    name: Option<String>,
    size: Option<BDFPropertySize>,
    properties: Option<BDFRealProperties>,
    characters: Option<Vec<BDFCharacter>>,
}

#[derive(Debug, Default)]
pub struct BDFCharacter {
    name: Option<String>,
    encoding: Option<u32>,
    s_width: Option<(u32, u32)>,
    d_width: Option<(u32, u32)>,
    bbx: Option<(i32, i32, i32, i32)>,
    bitmap: Option<BDFBitmap>,
}

#[derive(Debug, Default)]
pub struct BDFBitmap {
    /// The outer vector represents the rows.
    /// The inner vector represents the columns. Columns are right padded to fill out the full byte.
    /// Each bit represents a single pixel.
    bytes: Vec<Vec<u8>>,
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

// pub struct BDFFont {
//     version: f32,
//     name: String,
// }

impl BDFFont {
    pub fn load(bytes: &[u8]) -> Result<BDFFont, FontError> {
        let reader = BufReader::new(bytes);

        let mut lines = reader.lines();

        // let version = parse_first_line(&next_line(&mut lines)?)?;
        // let name = parse_second_line(&next_line(&mut lines)?)?;

        parse_bdf_font(&mut lines)

        // println!("Props: {:?}", properties);

        // Err("a")?
        // Ok(BDFFont {
        //     version,
        //     name
        // })
    }
}

fn next_line(lines: &mut Lines<BufReader<&[u8]>>) -> Result<Option<String>, FontError> {
    Ok(lines.next().transpose()?)
    // Ok(.ok_or_else(|| "Unexpected end of line")??)
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

fn parse_bitmap(lines: &mut Lines<BufReader<&[u8]>>) -> Result<BDFBitmap, FontError> {
    let mut bytes: Vec<Vec<u8>> = vec![];
    while let Some(line) = next_line(lines)? {
        if line == "ENDCHAR" {
            break;
        }

        bytes.push(string_to_bytes(&line)?);
    }
    Ok(BDFBitmap { bytes })
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
        let property_value_literal = parts.next().ok_or_else(|| "Missing value")?;

        match property_name {
            "ENCODING" => {
                character.encoding = Some(u32::from_str_radix(property_value_literal, 10)?)
            }
            "BITMAP" => {
                character.bitmap = Some(parse_bitmap(lines)?);
                break;
            }
            "SWIDTH" | "DWIDTH" | "BBX" => {}
            _ => println!("Unexpected character property {}", line),
        }
    }
    Ok(character)
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
        let property_value_literal = parts.next().ok_or_else(|| "Missing value")?;

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
