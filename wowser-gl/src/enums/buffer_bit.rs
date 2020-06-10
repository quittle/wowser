use wowser_gl_sys::*;

pub enum BufferBit {
    Color,
    Depth,
    Accumulation,
    Stencil,
}

impl AsRef<GLbitfield> for BufferBit {
    fn as_ref(&self) -> &GLbitfield {
        match *self {
            BufferBit::Color => &GL_COLOR_BUFFER_BIT,
            BufferBit::Depth => &GL_DEPTH_BUFFER_BIT,
            BufferBit::Accumulation => &GL_ACCUM_BUFFER_BIT,
            BufferBit::Stencil => &GL_STENCIL_BUFFER_BIT,
        }
    }
}

impl std::ops::BitOr for BufferBit {
    type Output = GLbitfield;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as GLbitfield | rhs as GLbitfield
    }
}
