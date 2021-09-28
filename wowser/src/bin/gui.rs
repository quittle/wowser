use wowser::browser::Tab;
use wowser::startup;
use wowser::ui::Window;
use wowser_glfw as glfw;

fn main() {
    startup::start();
    {
        let mut window = Window::new().unwrap();
        let _link_fancy_html = r#"
            <html>
                <style>
                    html {
                        background-color: #05f;
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

        let padding_based_html = r#"
            <html>
                <style>
                    html {
                        background-color: black;
                    }

                    .a {
                        background-color: red;
                        padding: 12px;
                    }
                    .b {
                        background-color: orange;
                        padding: 3px;
                        padding-top: 10px;
                        padding-right: 100px;
                    }
                    .c {
                        background-color: yellow;
                        padding: 3px;
                        margin: 3px;
                    }
                </style>
                <div class="a"><div class="b"><div class="c">text</div></div></div>
            </html>
        "#;

        let html = padding_based_html;

        let mut tab = Tab::load(&mut window, html);

        loop {
            glfw::poll_events().unwrap();
            tab.render();
        }
    }
}
