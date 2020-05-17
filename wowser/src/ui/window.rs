pub struct Window {
    _window: wowser_glfw::Window,
}

impl Window {
    pub fn new() -> Result<Window, String> {
        let window = wowser_glfw::create_window(800, 600, "Wowser - what a browser!", None)
            .map_err::<String, _>(Into::into)?;

        Ok(Window { _window: window })
    }
}
