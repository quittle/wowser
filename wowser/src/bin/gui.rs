use wowser::render;
use wowser::startup;
use wowser::ui::Window;
use wowser::util::Rect;

use std::thread;

fn render(window: &mut Window) {
    let example_root = render::example_style_nodes();
    let style_root = render::style_to_scene(&example_root);

    for node in style_root {
        match node {
            render::SceneNode::TextSceneNode(render::TextSceneNode {
                bounds: _bounds,
                text: _text,
                font_size: _font_size,
                text_color: _text_color,
            }) => {}
            render::SceneNode::RectangleSceneNode(render::RectangleSceneNode {
                bounds,
                fill: _fill,
                border_color: _border_color,
                border_width: _border_width,
            }) => {
                println!("Rect: {:?}", bounds);
                window
                    .draw_rect(&Rect {
                        x: bounds.x as i32,
                        y: bounds.y as i32,
                        width: bounds.width as i32,
                        height: bounds.height as i32,
                    })
                    .expect("");
            }
        }
    }
}

fn main() {
    startup::start();
    {
        let mut window = Window::new().expect("Unable to make ui.");
        window.draw_rect(&Rect { x: 0, y: 0, width: 100, height: 100 }).expect("");
        render(&mut window);
        thread::sleep(std::time::Duration::from_millis(20000));
        window.resize(&Rect { x: 100, y: 100, width: 200, height: 200 }).unwrap();
        thread::sleep(std::time::Duration::from_millis(2000));
    }
    wowser_glfw::terminate();
}
