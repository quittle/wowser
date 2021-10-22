use crate::font::FontError;

pub enum IndexToLocationFormat {
    Short,
    Long,
}

impl IndexToLocationFormat {
    pub fn new(value: i16) -> Result<Self, FontError> {
        Ok(match value {
            0 => Self::Short,
            1 => Self::Long,
            _ => {
                return Err(format!("Unsupported font index to location format: {}", value).into())
            }
        })
    }
}
