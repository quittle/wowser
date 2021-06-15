use super::UiResult;
use crate::{
    render::Color,
    util::{Point, Rect},
};
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

    pub fn draw_rect(
        &mut self,
        rect: &Rect<i32>,
        fill_color: &Color,
        border_color: &Color,
        border_width: f32,
    ) -> UiResult {
        let mut action_taken = false;

        if border_width > 0_f32 && border_color.a != 0 {
            gl::point_size(10.0)?;
            gl::line_width(border_width)?;
            gl::color_4ub(border_color.r, border_color.g, border_color.b, border_color.a);

            gl::begin(gl::DrawMode::LineLoop);
            gl::vertex_2i(rect.x, rect.y);
            gl::vertex_2i(rect.x + rect.width, rect.y);
            gl::vertex_2i(rect.x + rect.width, rect.y + rect.height);
            gl::vertex_2i(rect.x, rect.y + rect.height);
            gl::end()?;
            action_taken = true;
        }

        if fill_color.a != 0 {
            gl::point_size(1_f32)?;
            gl::line_width(1_f32)?;
            gl::color_4ub(fill_color.r, fill_color.g, fill_color.b, fill_color.a);
            gl::begin(gl::DrawMode::Polygon);
            gl::vertex_2i(rect.x, rect.y);
            gl::vertex_2i(rect.x + rect.width, rect.y);
            gl::vertex_2i(rect.x + rect.width, rect.y + rect.height);
            gl::vertex_2i(rect.x, rect.y + rect.height);
            gl::end()?;
            action_taken = true;
        }

        if action_taken {
            gl::flush()?;

            self.window.swap_buffers();
        }

        Ok(())
    }

    pub fn draw_bitmap(
        &mut self,
        point: &Point<i32>,
        bitmap: &[u8],
        width: u32,
        color: &Color,
    ) -> UiResult {
        let height = (bitmap.len() as u32 / width) as i32;

        if color.a != 0 {
            gl::color_4ub(color.r, color.g, color.b, color.a);
            gl::pixel_zoom(1.0, -1.0)?;
            gl::raster_pos_2i(point.x, point.y + height)?;
            gl::pixel_store_i(gl::Alignment::PackAlignment, gl::AlignmentValue::One);
            gl::pixel_store_i(gl::Alignment::UnpackAlignment, gl::AlignmentValue::One);
            gl::bitmap(width as i32 * 8, height, 0.0, 0.0, width as f32, 0.0, &bitmap)?;
            gl::pixel_zoom(1.0, 1.0)?;
            gl::flush()?;

            self.window.swap_buffers();
        }

        Ok(())
    }

    pub fn get_bounds(&self) -> &Rect<i32> {
        &self.bounds
    }
}
