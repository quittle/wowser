use wowser_gl_sys::*;

#[derive(Debug, PartialEq)]
pub enum PixelDataType {
    UnsignedByte,
    Byte,
    UnsignedShort,
    Short,
    UnsignedInt,
    Int,
    HalfFloat,
    Float,
    UnsignedByte332,
    UnsightedByte233Rev,
    UnsignedShort565,
    UnsignedShort565Rev,
    UnsignedShort4444,
    UnsignedShort4444Rev,
    UnsignedShort5551,
    UnsignedShort1555Rev,
    UnsignedInt8888,
    UnsignedInt8888Rev,
    UnsignedInt1010102,
    UnsignedInt2101010Rev,
    UnsignedInt248,
    UnsignedInt10f11f11fRev,
    UnsignedInt5999Rev,
    UnsignedFloat32UnsignedInt248Rev,
}

impl From<PixelDataType> for GLenum {
    fn from(pixel_data_type: PixelDataType) -> GLenum {
        match pixel_data_type {
            PixelDataType::UnsignedByte => GL_UNSIGNED_BYTE,
            PixelDataType::Byte => GL_BYTE,
            PixelDataType::UnsignedShort => GL_UNSIGNED_SHORT,
            PixelDataType::Short => GL_SHORT,
            PixelDataType::UnsignedInt => GL_UNSIGNED_INT,
            PixelDataType::Int => GL_INT,
            PixelDataType::HalfFloat => GL_HALF_FLOAT,
            PixelDataType::Float => GL_FLOAT,
            PixelDataType::UnsignedByte332 => GL_UNSIGNED_BYTE_3_3_2,
            PixelDataType::UnsightedByte233Rev => GL_UNSIGNED_BYTE_2_3_3_REV,
            PixelDataType::UnsignedShort565 => GL_UNSIGNED_SHORT_5_6_5,
            PixelDataType::UnsignedShort565Rev => GL_UNSIGNED_SHORT_5_6_5_REV,
            PixelDataType::UnsignedShort4444 => GL_UNSIGNED_SHORT_4_4_4_4,
            PixelDataType::UnsignedShort4444Rev => GL_UNSIGNED_SHORT_4_4_4_4_REV,
            PixelDataType::UnsignedShort5551 => GL_UNSIGNED_SHORT_5_5_5_1,
            PixelDataType::UnsignedShort1555Rev => GL_UNSIGNED_SHORT_1_5_5_5_REV,
            PixelDataType::UnsignedInt8888 => GL_UNSIGNED_INT_8_8_8_8,
            PixelDataType::UnsignedInt8888Rev => GL_UNSIGNED_INT_8_8_8_8_REV,
            PixelDataType::UnsignedInt1010102 => GL_UNSIGNED_INT_10_10_10_2,
            PixelDataType::UnsignedInt2101010Rev => GL_UNSIGNED_INT_2_10_10_10_REV,
            PixelDataType::UnsignedInt248 => GL_UNSIGNED_INT_24_8,
            PixelDataType::UnsignedInt10f11f11fRev => GL_UNSIGNED_INT_10F_11F_11F_REV,
            PixelDataType::UnsignedInt5999Rev => GL_UNSIGNED_INT_5_9_9_9_REV,
            PixelDataType::UnsignedFloat32UnsignedInt248Rev => GL_FLOAT_32_UNSIGNED_INT_24_8_REV,
        }
    }
}
