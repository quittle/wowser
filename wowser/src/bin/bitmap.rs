use std::{env, fs};

use wowser::{
    image::Bitmap,
    render::Color,
    startup, ui,
    util::{Point, Rect},
};

fn main() {
    startup::start();

    let args: Vec<String> = env::args().collect();
    let bitmap_path = args.get(1).expect("Bitmap file not passed in");
    let bitmap_bytes = fs::read(bitmap_path).unwrap();
    let bitmap = Bitmap::new(&bitmap_bytes).unwrap();
    let window_rc = ui::Window::new().unwrap();
    let mut window = window_rc.borrow_mut();
    let mut x = -50;
    let mut y = -50;
    loop {
        let bounds = window.get_bounds().clone();
        let mut window_mutator = window.mutate();
        window_mutator
            .draw_rect(
                &Rect {
                    x: 0,
                    y: 0,
                    width: bounds.width,
                    height: bounds.height,
                },
                &Color::BLACK,
                &Color::TRANSPARENT,
                0.0,
            )
            .unwrap();
        window_mutator
            .draw_pixels(&Point { x, y }, &bitmap.pixels, bitmap.width)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        x *= -2;
        y += 50;
    }
}
