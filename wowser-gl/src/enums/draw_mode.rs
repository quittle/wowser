use wowser_gl_sys::*;

pub enum DrawMode {
    Points,
    Lines,
    LineStrip,
    LineLoop,
    Triangles,
    TriangleStrip,
    TriangleFan,
    Quads,
    QuadStrip,
    Polygon,
}

impl From<DrawMode> for GLenum {
    fn from(draw_mode: DrawMode) -> GLenum {
        match draw_mode {
            DrawMode::Points => GL_POINTS,
            DrawMode::Lines => GL_LINES,
            DrawMode::LineStrip => GL_LINE_STRIP,
            DrawMode::LineLoop => GL_LINE_LOOP,
            DrawMode::Triangles => GL_TRIANGLES,
            DrawMode::TriangleStrip => GL_TRIANGLE_STRIP,
            DrawMode::TriangleFan => GL_TRIANGLE_FAN,
            DrawMode::Quads => GL_QUADS,
            DrawMode::QuadStrip => GL_QUAD_STRIP,
            DrawMode::Polygon => GL_POLYGON,
        }
    }
}
