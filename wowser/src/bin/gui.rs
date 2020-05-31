use wowser::startup;
use wowser::ui::{Rect, Window};

use std::thread;

fn main() {
    startup::start();
    {
        let mut window = Window::new().expect("Unable to make ui.");
        window.draw_rect(&Rect {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        });
        thread::sleep(std::time::Duration::from_millis(2000));
        window
            .resize(&Rect {
                x: 100,
                y: 100,
                width: 200,
                height: 200,
            })
            .unwrap();
        thread::sleep(std::time::Duration::from_millis(2000));
    }
    wowser_glfw::terminate();
}
