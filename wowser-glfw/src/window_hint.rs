use std::os::raw::c_int;
use wowser_glfw_sys::*;

pub enum WindowHint {
    Stereo,
    Doublebuffer,
    ClientApi,
    ContextCreationApi,
    OpenglForwardCompat,
    OpenglProfile,
    Resizable,
    Visible,
    Decorated,
    Focused,
    AutoIconify,
    Floating,
    Maximized,
    CenterCursor,
    TransparentFramebuffer,
    FocusOnShow,
    ScaleToMonitor,
}

impl From<WindowHint> for c_int {
    fn from(hint: WindowHint) -> c_int {
        (match hint {
            WindowHint::Stereo => GLFW_STEREO,
            WindowHint::Doublebuffer => GLFW_DOUBLEBUFFER,
            WindowHint::ClientApi => GLFW_CLIENT_API,
            WindowHint::ContextCreationApi => GLFW_CONTEXT_CREATION_API,
            WindowHint::OpenglForwardCompat => GLFW_OPENGL_FORWARD_COMPAT,
            WindowHint::OpenglProfile => GLFW_OPENGL_PROFILE,
            WindowHint::Resizable => GLFW_RESIZABLE,
            WindowHint::Visible => GLFW_VISIBLE,
            WindowHint::Decorated => GLFW_DECORATED,
            WindowHint::Focused => GLFW_FOCUSED,
            WindowHint::AutoIconify => GLFW_AUTO_ICONIFY,
            WindowHint::Floating => GLFW_FLOATING,
            WindowHint::Maximized => GLFW_MAXIMIZED,
            WindowHint::CenterCursor => GLFW_CENTER_CURSOR,
            WindowHint::TransparentFramebuffer => GLFW_TRANSPARENT_FRAMEBUFFER,
            WindowHint::FocusOnShow => GLFW_FOCUS_ON_SHOW,
            WindowHint::ScaleToMonitor => GLFW_SCALE_TO_MONITOR,
        }) as c_int
    }
}
