use wowser::browser::Tab;
use wowser::startup;
use wowser::ui::Window;
use wowser_glfw as glfw;

fn main() {
    startup::start();
    {
        let mut window = Window::new().unwrap();
        let html = r#"
            <html>
                <style>
                    html {
                        background-color: #00f;
                    }
                </style>
                <link rel="stylesheet" href="http://0.0.0.0:8000/wowser/data/example.css" />
                before
                <img src="http://www.w3.org/People/mimasa/test/imgformat/img/w3c_home.bmp" />
                after
                <div class="wrapper">
                    <div>abc<span>def</span>ghi</div>
                    <div class="foo">bbbbbbb</div>
                </div>
            </html>
        "#;

        let mut tab = Tab::load(&mut window, html);

        loop {
            glfw::poll_events().unwrap();
            tab.render();
        }
    }
}
