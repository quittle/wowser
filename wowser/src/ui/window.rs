use super::UiResult;
use crate::util::{Point, Rect};
use wowser_gl as gl;
use wowser_glfw as glfw;

pub struct Window {
    window: glfw::Window,
    bounds: Rect<i32>,
}

impl Window {
    pub fn new() -> Result<Window, String> {
        let bounds = Rect { x: 100, y: 100, width: 800, height: 600 };

        let window = glfw::Window::new(1, 1, "Wowser - what a browser!", None)
            .map_err::<String, _>(Into::into)?;
        window.make_context_current().map_err::<String, _>(Into::into)?;

        let mut window = Window { window, bounds: Rect { x: 0, y: 0, width: 0, height: 0 } };

        window.resize(&bounds).map_err::<String, _>(Into::into)?;

        Ok(window)
    }

    pub fn resize(&mut self, new_bounds: &Rect<i32>) -> UiResult {
        if new_bounds.width != self.bounds.width || new_bounds.height != self.bounds.height {
            self.window.set_window_size(new_bounds.width, new_bounds.height)?;
        }

        if new_bounds.x != self.bounds.x || new_bounds.y != self.bounds.y {
            self.window.set_window_pos(new_bounds.x, new_bounds.y)?;
        }

        if new_bounds != &self.bounds {
            self.bounds.clone_from(&new_bounds);

            gl::viewport(0, 0, self.bounds.width, self.bounds.height)?;
            gl::ortho(0.0, self.bounds.width.into(), self.bounds.height.into(), 0.0, -1.0, 1.0)?;
            gl::clear(&[gl::BufferBit::Color])?;

            self.window.swap_buffers();
        }

        Ok(())
    }

    pub fn draw_rect(&mut self, rect: &Rect<i32>) -> UiResult {
        gl::point_size(10.0)?;
        gl::line_width(2.5)?;
        gl::color_3f(1.0, 0.0, 0.0);

        gl::begin(gl::DrawMode::LineLoop);
        gl::vertex_2i(rect.x, rect.y);
        gl::vertex_2i(rect.x + rect.width, rect.y);
        gl::vertex_2i(rect.x + rect.width, rect.y + rect.height);
        gl::vertex_2i(rect.x, rect.y + rect.height);
        gl::end()?;

        gl::flush()?;

        self.window.swap_buffers();

        Ok(())
    }

    pub fn draw_bitmap(&mut self, point: &Point<i32>, bitmap: &[u8], width: u32) -> UiResult {
        let height: usize = bitmap.len() / width as usize;

        gl::pixel_zoom(1.0, -1.0)?;
        gl::raster_pos_2i(point.x, point.y)?;
        gl::pixel_store_i(gl::Alignment::PackAlignment, gl::AlignmentValue::One);
        gl::pixel_store_i(gl::Alignment::UnpackAlignment, gl::AlignmentValue::One);
        gl::bitmap(width as i32 * 8, height as i32, 0.0, 0.0, width as f32, 0.0, &bitmap)?;
        gl::pixel_zoom(1.0, 1.0)?;
        gl::flush()?;

        self.window.swap_buffers();

        Ok(())
    }
}
