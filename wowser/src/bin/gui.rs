use wowser::browser;
use wowser::startup;
use wowser::ui::Window;
use wowser_glfw as glfw;

fn main() {
    startup::start();
    {
        let mut window = Window::new().unwrap();
        let html = r#"
            <html>
                <div class="wrapper">
                    <div>abc<span>def</span>ghi</div>
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
        loop {
            glfw::wait_events().unwrap();
        }
    }
}
