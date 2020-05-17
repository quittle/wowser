wowser_glfw::ErrorCallback!(
    fn callback(err: wowser_glfw::GlfwError, m: str) {
        println!("Err {}: {}", err, m);
    }
);

pub fn start() {
    wowser_glfw::set_error_callback(Some(callback));
    wowser_glfw::init().expect("Unable to initialize GLFW");
}
