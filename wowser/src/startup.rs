use wowser_gl_sys::*;

wowser_glfw::ErrorCallback!(
    fn callback(err: wowser_glfw::GlfwError, m: str) {
        println!("Err {}: {}", err, m);
    }
);

pub fn initialize_gl() {
    wowser_glfw::set_error_callback(Some(callback));
    wowser_glfw::init().expect("Unable to initialize GLFW");

    unsafe {
        glDisable(GL_DEPTH_TEST);
        glMatrixMode(GL_PROJECTION);
        glLoadIdentity();
    }
}

pub fn start() {
    initialize_gl();
}
