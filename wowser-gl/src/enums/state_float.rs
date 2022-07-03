use wowser_gl_sys::*;

pub enum StateFloat {
    AliasedLineWidthRange,
    LineWidthRange,
}

impl StateFloat {
    pub fn return_length(&self) -> usize {
        match *self {
            Self::AliasedLineWidthRange => 2,
            Self::LineWidthRange => 2,
        }
    }
}

impl From<StateFloat> for GLenum {
    fn from(value: StateFloat) -> GLenum {
        match value {
            StateFloat::AliasedLineWidthRange => GL_ALIASED_LINE_WIDTH_RANGE,
            StateFloat::LineWidthRange => GL_LINE_WIDTH_RANGE,
        }
    }
}
