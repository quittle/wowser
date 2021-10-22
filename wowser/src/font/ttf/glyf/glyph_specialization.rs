use std::ops::Neg;

use super::{Flag, FlagCoordinate, GlyfCoordinate};
use crate::font::FontError;

pub enum GlyphSpecialization {
    Complex,
    Simple {
        end_pts_of_contours: Vec<u16>,
        instruction_length: u16,
        instructions: Vec<u8>,
        flags: Vec<Flag>,
        x_coordinates: Vec<GlyfCoordinate>,
        y_coordinates: Vec<GlyfCoordinate>,
    },
}

impl GlyphSpecialization {
    pub fn new(bytes: &[u8], num_of_contours: i16) -> Result<(Self, usize), FontError> {
        if num_of_contours >= 0 {
            Self::new_simple(bytes, num_of_contours)
        } else {
            Ok((Self::Complex, 0))
        }
    }

    fn new_simple(bytes: &[u8], num_of_contours: i16) -> Result<(Self, usize), FontError> {
        let num_of_contours = usize::try_from(num_of_contours)?;
        let mut offset = num_of_contours * 2;
        let end_pts_of_contours: Vec<u16> = bytes[..num_of_contours * 2]
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect();
        if end_pts_of_contours.len() != num_of_contours {
            return Err("Not enough bytes to read contours".into());
        }

        let instruction_length = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
        offset += 2;

        let instruction_length_usize = usize::try_from(instruction_length)?;

        let instructions = bytes[offset..offset + instruction_length_usize].into();
        offset += instruction_length_usize;

        let num_of_points = usize::try_from(
            end_pts_of_contours
                .last()
                .ok_or("Missing points in glyph contour")?
                + 1,
        )?;

        let mut flags = vec![];
        while flags.len() < num_of_points {
            let flag = Flag::new(bytes[offset]);
            offset += 1;
            flags.push(flag.clone());
            if flag.repeats() {
                let repeat_count = bytes[offset] as usize;
                offset += 1;
                flags.append(&mut vec![flag; repeat_count]);
            }
        }

        let mut parse_coordinates = |get_setting: &dyn Fn(&Flag) -> FlagCoordinate| -> Result<Vec<GlyfCoordinate>, FontError> {
            let mut coordinates = vec![];
            for flag in &flags {
                coordinates.push(match get_setting(flag) {
                    FlagCoordinate::PositiveU8 => {
                        let byte = bytes[offset];
                        offset += 1;
                        GlyfCoordinate::U8(byte)
                    }
                    FlagCoordinate::NegativeU8 => {
                        let byte = bytes[offset];
                        offset += 1;
                        GlyfCoordinate::I16((byte as i16).neg())
                    }
                    FlagCoordinate::CopyPrevious => {
                        *coordinates.last().ok_or("Missing previous coordinates for Copy Prevoius")?
                    }
                    FlagCoordinate::DeltaI16 => {
                        let previous =
                            coordinates.last()
                                // This sometimes happens for the first value
                                .unwrap_or(&GlyfCoordinate::U8(0));
                        let delta = i16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
                        offset += 2;

                        match previous {
                            GlyfCoordinate::U8(value) => {
                                GlyfCoordinate::I16(*value as i16 + delta)
                            }
                            GlyfCoordinate::I16(value) => {
                                GlyfCoordinate::I16(value + delta)
                            }
                        }
                    }
                })
            }
            Ok(coordinates)
        };

        let x_coordinates = parse_coordinates(&|flag: &Flag| flag.get_x_coordinate_setting())?;
        let y_coordinates = parse_coordinates(&|flag: &Flag| flag.get_y_coordinate_setting())?;

        Ok((
            Self::Simple {
                end_pts_of_contours,
                instruction_length,
                instructions,
                flags,
                x_coordinates,
                y_coordinates,
            },
            offset,
        ))
    }
}
