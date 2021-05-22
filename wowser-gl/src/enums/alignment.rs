use wowser_gl_sys::*;

pub enum Alignment {
    PackAlignment,
    UnpackAlignment,
}

impl From<Alignment> for GLenum {
    fn from(alignment: Alignment) -> GLenum {
        match alignment {
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

impl From<AlignmentValue> for i32 {
    fn from(alignment_value: AlignmentValue) -> i32 {
        match alignment_value {
            AlignmentValue::One => 1,
            AlignmentValue::Two => 2,
            AlignmentValue::Four => 4,
            AlignmentValue::Eight => 8,
        }
    }
}
