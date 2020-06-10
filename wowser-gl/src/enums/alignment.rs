use wowser_gl_sys::*;

pub enum Alignment {
    PackAlignment,
    UnpackAlignment,
}

impl Into<GLenum> for Alignment {
    fn into(self) -> GLenum {
        match self {
            Alignment::PackAlignment => GL_PACK_ALIGNMENT,
            Alignment::UnpackAlignment => GL_UNPACK_ALIGNMENT,
        }
    }
}

pub enum AlignmentValue {
    One,
    Two,
    Four,
    Eight,
}

impl Into<i32> for AlignmentValue {
    fn into(self) -> i32 {
        match self {
            AlignmentValue::One => 1,
            AlignmentValue::Two => 2,
            AlignmentValue::Four => 4,
            AlignmentValue::Eight => 8,
        }
    }
}
