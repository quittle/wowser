use wowser_gl_sys::*;

pub enum MatrixMode {
    ModelView,
    Projection,
    Texture,
    Color,
}

impl From<MatrixMode> for GLenum {
    fn from(matrix_mode: MatrixMode) -> GLenum {
        match matrix_mode {
            MatrixMode::ModelView => GL_MODELVIEW,
            MatrixMode::Projection => GL_PROJECTION,
            MatrixMode::Texture => GL_TEXTURE,
            MatrixMode::Color => GL_COLOR,
        }
    }
}
