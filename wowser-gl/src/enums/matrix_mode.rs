use wowser_gl_sys::*;

pub enum MatrixMode {
    ModelView,
    Projection,
    Texture,
    Color,
}

impl Into<GLenum> for MatrixMode {
    fn into(self) -> GLenum {
        match self {
            MatrixMode::ModelView => GL_MODELVIEW,
            MatrixMode::Projection => GL_PROJECTION,
            MatrixMode::Texture => GL_TEXTURE,
            MatrixMode::Color => GL_COLOR,
        }
    }
}
