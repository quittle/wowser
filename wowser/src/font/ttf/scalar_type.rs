use crate::font::FontError;

#[derive(Debug, PartialEq)]
pub enum ScalarType {
    True,
    Typ1,
    Cff,
    Otto,
    Glyf,
}

impl ScalarType {
    pub fn new(scalar_type: &[u8]) -> Result<Self, FontError> {
        Ok(match scalar_type {
            b"true" | [0, 1, 0, 0] => Self::True,
            b"typ1" => Self::Typ1,
            b"OTTO" => Self::Otto,
            b"CFF " => Self::Cff,
            b"glyf" => Self::Glyf,
            _ => return Err(format!("Invalid scalar type: {:?}", scalar_type).into()),
        })
    }
}
