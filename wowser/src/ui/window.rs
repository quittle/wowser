use wowser_glfw_sys::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
pub struct Window {
    window: wowser_glfw::Window,
    bounds: Rect,
}

impl Window {
    pub fn new() -> Result<Window, String> {
        let bounds = Rect {
            x: 100,
            y: 100,
            width: 800,
            height: 600,
        };

        let window = wowser_glfw::Window::new(1, 1, "Wowser - what a browser!", None)
            .map_err::<String, _>(Into::into)?;
        window
            .make_context_current()
            .map_err::<String, _>(Into::into)?;

        let mut window = Window {
            window,
            bounds: Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
        };

        window.resize(&bounds).map_err::<String, _>(Into::into)?;

        Ok(window)
    }

    pub fn resize(&mut self, new_bounds: &Rect) -> Result<(), String> {
        if new_bounds.width != self.bounds.width || new_bounds.height != self.bounds.height {
            self.window
                .set_window_size(new_bounds.width, new_bounds.height)
                .map_err::<String, _>(Into::into)?;
        }

        if new_bounds.x != self.bounds.x || new_bounds.y != self.bounds.y {
            self.window
                .set_window_pos(new_bounds.x, new_bounds.y)
                .map_err::<String, _>(Into::into)?;
        }

        if new_bounds != &self.bounds {
            self.bounds.clone_from(&new_bounds);

            unsafe {
                glViewport(0, 0, self.bounds.width, self.bounds.height);
                glOrtho(
                    0.0,
                    self.bounds.width.into(),
                    self.bounds.height.into(),
                    0.0,
                    -1.0,
                    1.0,
                );

                glClearColor(0f32, 1f32, 1f32, 0.5f32);
                glClear(GL_COLOR_BUFFER_BIT);

                self.window.swap_buffers();
            }
        }

        Ok(())
    }

    pub fn draw_rect(&mut self, rect: &Rect) -> Option<String> {
        unsafe {
            glClearColor(0f32, 1f32, 1f32, 0.5f32);
            glClear(GL_COLOR_BUFFER_BIT);
            glPointSize(10.0);
            glLineWidth(2.5);
            glColor3f(1.0, 0.0, 0.0);

            glBegin(GL_LINE_LOOP);
            glVertex2i(rect.x, rect.y);
            glVertex2i(rect.x + rect.width, rect.y);
            glVertex2i(rect.x + rect.width, rect.y + rect.height);
            glVertex2i(rect.x, rect.y + rect.height);
            glEnd();

            glFlush();
        }

        self.window.swap_buffers();
        None
    }
}
