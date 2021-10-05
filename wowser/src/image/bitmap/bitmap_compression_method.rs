#[derive(Debug, PartialEq, Clone)]
pub enum BitmapCompressionMethod {
    Rgb,
    Rle8,
    Rle4,
    BitFields,
    Jpeg,
    Png,
    AlphaBitFields,
    Cmyk,
    CmykRle8,
    CmykRle4,
}

impl BitmapCompressionMethod {
    pub fn deserialize(compression_method: u32) -> Option<Self> {
        match compression_method {
            0 => Some(Self::Rgb),
            1 => Some(Self::Rle8),
            2 => Some(Self::Rle4),
            3 => Some(Self::BitFields),
            4 => Some(Self::Jpeg),
            5 => Some(Self::Png),
            6 => Some(Self::AlphaBitFields),
            11 => Some(Self::Cmyk),
            12 => Some(Self::CmykRle8),
            13 => Some(Self::CmykRle4),
            _ => None,
        }
    }
}

impl From<BitmapCompressionMethod> for u32 {
    fn from(compression_method: BitmapCompressionMethod) -> u32 {
        match compression_method {
            BitmapCompressionMethod::Rgb => 0,
            BitmapCompressionMethod::Rle8 => 1,
            BitmapCompressionMethod::Rle4 => 2,
            BitmapCompressionMethod::BitFields => 3,
            BitmapCompressionMethod::Jpeg => 4,
            BitmapCompressionMethod::Png => 5,
            BitmapCompressionMethod::AlphaBitFields => 6,
            BitmapCompressionMethod::Cmyk => 11,
            BitmapCompressionMethod::CmykRle8 => 12,
            BitmapCompressionMethod::CmykRle4 => 13,
        }
    }
}
