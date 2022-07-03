use std::cmp;

use super::{UiError, UiResult};
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
        let bounds = Rect {
            x: 100,
            y: 100,
            width: 800,
            height: 600,
        };

        let window = glfw::Window::new(1, 1, "Wowser - what a browser!", None)
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

        window
            .window
            .set_window_size_callback(Some(|width, height| {
                // TODO: Update window bounds
                log!(DEBUG: "new size:", width, "x", height);
            }))
            .map_err::<String, _>(Into::into)?;

        window.resize(&bounds).map_err::<String, _>(Into::into)?;

        Ok(window)
    }

    /// Attempts to update the position and size of the window. Not all OS's support all possible
    /// values and may move or resize windows after attempting to handle the resize. After resizing,
    /// check the bounds to get the actual values.
    pub fn resize(&mut self, new_bounds: &Rect<i32>) -> UiResult {
        if new_bounds.width != self.bounds.width || new_bounds.height != self.bounds.height {
            self.window
                .set_window_size(new_bounds.width, new_bounds.height)?;
        }

        if new_bounds.x != self.bounds.x || new_bounds.y != self.bounds.y {
            self.window.set_window_pos(new_bounds.x, new_bounds.y)?;
        }

        // Despite attempting to set bounds, OS's may not accept a given value. For instance,
        // they may require a minimum width and height, or not support a position that is fully off
        // screen. We need to read the actual bounds after attempting to set them to avoid bounds
        // getting out of sync.
        let (x, y, width, height) = self.window.get_window_bounds()?;
        let actualized_bounds = Rect {
            x: x as i32,
            y: y as i32,
            width: width as i32,
            height: height as i32,
        };

        if actualized_bounds != self.bounds {
            self.bounds.clone_from(&actualized_bounds);
            gl::viewport(0, 0, self.bounds.width, self.bounds.height)?;
            // Reset the projection matrix before resetting ortho
            gl::load_identity()?;
            gl::ortho(
                0.0,
                self.bounds.width.into(),
                self.bounds.height.into(),
                0.0,
                -1.0,
                1.0,
            )?;

            self.window.swap_buffers();
        }

        Ok(())
    }

    pub fn mutate(&mut self) -> WindowMutator {
        WindowMutator { window: self }
    }

    pub fn get_pixels_rgb(&self) -> Result<Vec<u8>, UiError> {
        Ok(gl::read_pixels(
            0,
            0,
            self.bounds.width as usize,
            self.bounds.height as usize,
            gl::Format::Rgb,
            gl::PixelDataType::UnsignedByte,
        )?)
    }

    pub fn get_bounds(&self) -> &Rect<i32> {
        &self.bounds
    }

    pub fn get_glfw_window(&self) -> &glfw::Window {
        &self.window
    }

    /// Call to have the window
    pub fn check_for_updates(&mut self) -> UiResult {
        if self.window.should_close()? {
            self.window.close()?;
        }
        Ok(())
    }

    pub fn is_alive(&self) -> bool {
        self.window.is_alive()
    }
}

/// Performs mutations to a window, drop when complete to actually render the update.
pub struct WindowMutator<'window> {
    window: &'window mut Window,
}

impl WindowMutator<'_> {
    // Note that the `fill_color goes inside the `rect` while the `border_color` goes outside `rect`
    // by `border_width` pixels.
    pub fn draw_rect(
        &mut self,
        rect: &Rect<i32>,
        fill_color: &Color,
        border_color: &Color,
        border_width: f32,
    ) -> UiResult {
        if border_width > 0_f32 && !border_color.is_transparent() {
            // Depending on the hardware and driver, GL may not support more than 1px wide line
            // widths on all platforms. Using a quadstrip can emulate these lines and give increased
            // standardization and control over rendering.
            gl::color_4ub(
                border_color.r,
                border_color.g,
                border_color.b,
                border_color.a,
            );

            let x = rect.x as f32;
            let y = rect.y as f32;
            let width = rect.width as f32;
            let height = rect.height as f32;

            gl::begin(gl::DrawMode::QuadStrip);
            gl::vertex_2f(x, y);
            gl::vertex_2f(x - border_width, y - border_width);
            gl::vertex_2f(x, y + height);
            gl::vertex_2f(x - border_width, y + height + border_width);
            gl::vertex_2f(x + width + border_width, y + width + border_width);
            gl::vertex_2f(x + width, y + height);
            gl::vertex_2f(x + width + border_width, y - border_width);
            gl::vertex_2f(x + width, y);
            gl::vertex_2f(x - border_width, y - border_width);
            gl::vertex_2f(x, y);
            gl::end()?;
        }

        if !fill_color.is_transparent() {
            gl::point_size(10_f32)?;
            gl::line_width(10_f32)?;
            gl::color_4ub(fill_color.r, fill_color.g, fill_color.b, fill_color.a);
            gl::begin(gl::DrawMode::Polygon);
            gl::vertex_2i(rect.x, rect.y);
            gl::vertex_2i(rect.x + rect.width, rect.y);
            gl::vertex_2i(rect.x + rect.width, rect.y + rect.height);
            gl::vertex_2i(rect.x, rect.y + rect.height);
            gl::end()?;
        }

        Ok(())
    }

    pub fn draw_pixels(&mut self, point: &Point<i32>, pixels: &[Color], width: usize) -> UiResult {
        let height = pixels.len() / width;

        gl::pixel_store_i(gl::Alignment::UnpackAlignment, gl::AlignmentValue::One);

        let bounds = self.window.get_bounds();

        let left_offset = if point.x < 0 { point.x.abs() } else { 0 };

        let bottom_offset = if (point.y + height as i32) > bounds.height {
            point.y + height as i32 - bounds.height
        } else {
            0
        };

        let adjusted_x = cmp::max(0, point.x);
        let adjusted_y = point.y + height as i32 - bottom_offset;

        if left_offset >= width as i32
            || adjusted_x >= bounds.width as i32
            || adjusted_x < 0
            || bottom_offset >= height as i32
            || adjusted_y > bounds.height as i32
            || adjusted_y < 0
        {
            return Ok(());
        }

        gl::raster_pos_2i(adjusted_x, adjusted_y)?;

        let pixel_data: Vec<u8> = pixels
            .iter()
            .enumerate()
            .flat_map(|(index, color)| {
                let x = index % width;
                let y = index / width;
                if x as i32 >= left_offset && y as i32 >= bottom_offset {
                    vec![color.r, color.g, color.b]
                } else {
                    vec![]
                }
            })
            .collect();
        gl::draw_pixels(
            width - left_offset as usize,
            height - bottom_offset as usize,
            gl::Format::Rgb,
            gl::PixelDataType::UnsignedByte,
            &pixel_data,
        )?;
        Ok(())
    }

    pub fn draw_bitmap(
        &mut self,
        point: &Point<i32>,
        bitmap: &[u8],
        width: u32,
        color: &Color,
    ) -> UiResult {
        if width > 0 && color.a != 0 {
            let height = (bitmap.len() as u32 / width) as i32;
            gl::color_4ub(color.r, color.g, color.b, color.a);
            gl::pixel_zoom(1.0, -1.0)?;
            gl::raster_pos_2i(point.x, point.y + height)?;
            gl::pixel_store_i(gl::Alignment::UnpackAlignment, gl::AlignmentValue::One);
            gl::bitmap(
                width as i32 * 8,
                height,
                0.0,
                0.0,
                width as f32,
                0.0,
                bitmap,
            )?;
            gl::pixel_zoom(1.0, 1.0)?;
        }

        Ok(())
    }
}

impl Drop for WindowMutator<'_> {
    fn drop(&mut self) {
        let _ignore_error = gl::flush();
        self.window.window.swap_buffers();
    }
}

#[cfg(test)]
mod tests {
    use crate::{render::Color, startup, ui::tests::lock_for_ui_threads, util::Point};

    use super::Window;

    /// This test doesn't check if it rendered successfully but mostly verifies bounds checks are successful.
    #[test]
    pub fn test_draw_pixels_bounds() {
        lock_for_ui_threads(|| {
            startup::start();

            let mut window = Window::new().unwrap();

            // Should be bigger than the window
            let length = 1000_i32;
            let pixels = vec![Color::WHITE; (length * length) as usize];

            // Should be smaller than the window
            let small_length = 200;
            let small_pixels = vec![Color::WHITE; (small_length * small_length) as usize];

            let mut draw = |x: i32, y: i32| {
                let mut window_mutator = window.mutate();
                window_mutator
                    .draw_pixels(&Point { x, y }, &pixels, length as usize)
                    .unwrap();
                window_mutator
                    .draw_pixels(&Point { x, y }, &small_pixels, small_length as usize)
                    .unwrap();
            };

            draw(-100, -100);
            draw(-100, 100);
            draw(100, -100);

            draw(0, 0);
            draw(length, length);

            draw(100, length + 100);
            draw(100 + length, length);
            draw(100 + length, length + 100);
        });
    }
}
