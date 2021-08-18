use std::{env, fs};

use wowser::{image::Bitmap, startup, ui, util::Point};

fn main() {
    startup::start();

    let args: Vec<String> = env::args().collect();
    let bitmap_path = args.get(1).expect("Bitmap file not passed in");
    let bitmap_bytes = fs::read(bitmap_path).unwrap();
    let bitmap = Bitmap::new(&bitmap_bytes).unwrap();
    let mut window = ui::Window::new().unwrap();
    {
        window
            .mutate()
            .draw_pixels(&Point { x: 50, y: 50 }, &bitmap.pixels, bitmap.width)
            .unwrap();
    }
    std::thread::sleep(std::time::Duration::from_secs(100));
    startup::stop();
}
