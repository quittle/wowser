use wowser_gl_sys::*;

pub enum StateString {
    Vendor,
    Renderer,
    Version,
    ShadingLanguageVersion,
    Extensions,
}

impl From<StateString> for GLenum {
    fn from(value: StateString) -> GLenum {
        match value {
            StateString::Vendor => GL_VENDOR,
            StateString::Renderer => GL_RENDERER,
            StateString::Version => GL_VERSION,
            StateString::ShadingLanguageVersion => GL_SHADING_LANGUAGE_VERSION,
            StateString::Extensions => GL_EXTENSIONS,
        }
    }
}
