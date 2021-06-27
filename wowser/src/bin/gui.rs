use wowser::browser;
use wowser::startup;
use wowser::ui::Window;
use wowser::util::Rect;

use std::thread;

fn main() {
    startup::start();
    {
        let mut window = Window::new().unwrap();
        let html = r#"
            <html>
                <div class="wrapper">
                    <div>aaa<span>aa</span>aa</div>
                    <div class="foo">bbbbbbb</div>
                </div>
            </html>
        "#;
        let css = r#"
            div {
                background-color: #f00;
                color: #000;
            }

            .foo {
                background-color: #0ff;
            }

            .wrapper {
                background-color: #00f;
                color:#fff;
            }

            span {
                background-color: #0f0;
                border-color: #0ff;
                border-width: 3px;
            }
        "#;

        browser::render(&mut window, html, css);
        thread::sleep(std::time::Duration::from_millis(20000));
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
