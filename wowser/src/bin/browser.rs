use std::env;

use wowser::browser::Tab;
use wowser::net::{HttpRequest, Url};
use wowser::ui::Window;
use wowser::{log, startup};
use wowser_glfw as glfw;

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = args.get(1).expect("URL");

    let url = Url::parse(url).expect("Invalid URL provided");
    let request = HttpRequest::new(url);
    let response = futures::executor::block_on(request.get()).expect("Failed to load HTML page");
    if !response.status.contains_success_content() {
        log!(ERROR: "Invalid response", String::from_utf8_lossy(&response.body));
        return;
    }

    let html = std::str::from_utf8(&response.body).expect("Invalid HTML Encoding");

    startup::start();
    {
        let mut window = Window::new().unwrap();

        let mut tab = Tab::load(&mut window, html);

        loop {
            glfw::poll_events().unwrap();
            tab.render();
        }
    }
}
