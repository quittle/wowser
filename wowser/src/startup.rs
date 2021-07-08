use wowser_gl as gl;
use wowser_glfw as glfw;

glfw::ErrorCallback!(
    fn callback(err: wowser_glfw::GlfwError, m: str) {
        println!("GLFW Error {}: {}", err, m);
    }
);

fn initialize_glfw() -> glfw::GlfwResult {
    glfw::set_error_callback(Some(callback));
    glfw::init()?;
    Ok(())
}

fn initialize_gl() -> gl::GlResult {
    gl::disable(gl::Capability::DepthTest)?;
    gl::matrix_mode(gl::MatrixMode::Projection)?;
    gl::load_identity()?;
    Ok(())
}

pub fn start() {
    initialize_glfw().expect("Unable to initialize GLFW");
    initialize_gl().expect("Unable to initialize GL");
}

fn destroy_glfw() -> glfw::GlfwResult {
    glfw::terminate()?;
    Ok(())
}

pub fn stop() {
    destroy_glfw().expect("Unable to destroy glfw");
}
