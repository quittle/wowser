use wowser_gl_sys::*;

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
        let bounds = Rect { x: 100, y: 100, width: 800, height: 600 };

        let window = wowser_glfw::Window::new(1, 1, "Wowser - what a browser!", None)
            .map_err::<String, _>(Into::into)?;
        window.make_context_current().map_err::<String, _>(Into::into)?;

        let mut window = Window { window, bounds: Rect { x: 0, y: 0, width: 0, height: 0 } };

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
                glOrtho(0.0, self.bounds.width.into(), self.bounds.height.into(), 0.0, -1.0, 1.0);
                glClear(GL_COLOR_BUFFER_BIT);

                self.window.swap_buffers();
            }
        }

        Ok(())
    }

    pub fn draw_rect(&mut self, rect: &Rect) -> Option<String> {
        unsafe {
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

    pub fn draw_bitmap(&mut self, bitmap: &[Vec<u8>]) -> Result<(), String> {
        let height = bitmap.len();
        let width = bitmap.first().expect("").len();
        let total_bytes = width * height;
        let mut gp_bitmap_vec: Vec<GLubyte> = Vec::with_capacity(total_bytes);

        let mut bits = 0;
        // Open GL renders bitmaps and textures upside down so this must be reversed
        // before being used.
        for row in bitmap.iter().rev() {
            gp_bitmap_vec.extend(row);
            bits += row.len();
        }
        assert_eq!(bits, total_bytes);
        unsafe {
            glPointSize(10.0);
            glLineWidth(2.5);
            glPixelZoom(1.0, -1.0);
            glColor3f(1.0, 0.0, 0.0);
            glRasterPos2i(100, 100);
            glPixelStorei(GL_UNPACK_ALIGNMENT, 1);
            glPixelStorei(GL_PACK_ALIGNMENT, 1);
        }
        wowser_gl::bitmap(
            width as i32 * 8,
            height as i32,
            width as f32 / 2.0,
            height as f32 / 2.0,
            10.0,
            0.0,
            &gp_bitmap_vec,
        )
        .map_err(|e| e.to_string())?;
        unsafe {
            glPixelZoom(1.0, 1.0);
            glFlush();
        }

        self.window.swap_buffers();

        Ok(())
    }
}
