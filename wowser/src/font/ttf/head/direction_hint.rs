use crate::font::FontError;

pub enum DirectionHint {
    Mixed,
    OnlyLTR,
    OnlyLTRNeutral,
    OnlyRTL,
    OnlyRtlNeutral,
}

impl DirectionHint {
    pub fn new(value: i16) -> Result<Self, FontError> {
        Ok(match value {
            -2 => Self::OnlyRtlNeutral,
            -1 => Self::OnlyRTL,
            0 => Self::Mixed,
            1 => Self::OnlyLTR,
            2 => Self::OnlyLTRNeutral,
            _ => return Err(format!("Unsupported font direction hint: {value}").into()),
        })
    }
}
