use wowser_gl_sys::*;

pub enum Format {
    StencilIndex,
    DepthComponent,
    DepthStencil,
    Red,
    Green,
    Blue,
    Rgb,
    Rgba,
    Bgr,
    Bgra,
}

impl Format {
    /// How many bytes a given pixel represents for the format
    pub fn get_stride(&self) -> u8 {
        match self {
            Self::StencilIndex => 1,
            Self::DepthComponent => 1,
            Self::DepthStencil => 1,
            Self::Red => 1,
            Self::Green => 1,
            Self::Blue => 1,
            Self::Rgb => 3,
            Self::Rgba => 4,
            Self::Bgr => 3,
            Self::Bgra => 4,
        }
    }
}

impl From<Format> for GLenum {
    fn from(format: Format) -> GLenum {
        match format {
            Format::StencilIndex => GL_STENCIL_INDEX,
            Format::DepthComponent => GL_DEPTH_COMPONENT,
            Format::DepthStencil => GL_DEPTH_STENCIL,
            Format::Red => GL_RED,
            Format::Green => GL_GREEN,
            Format::Blue => GL_BLUE,
            Format::Rgb => GL_RGB,
            Format::Rgba => GL_RGBA,
            Format::Bgr => GL_BGR,
            Format::Bgra => GL_BGRA,
        }
    }
}
