use render::normalize_style_nodes;
use wowser::startup;
use wowser::ui::Window;
use wowser::util::Rect;
use wowser::{font::BDFFont, font::CachingFont, render, util::Point};

use std::{borrow::Borrow, thread};

const DEFAULT_FONT_BYTES: &[u8] = include_bytes!("../../data/gohufont-11.bdf");

fn render(window: &mut Window) {
    let mut example_root = render::example_style_nodes();
    normalize_style_nodes(&mut example_root);
    let style_root = render::style_to_scene(&example_root);

    let mut font: CachingFont = CachingFont::wrap(Box::new(
        BDFFont::load(DEFAULT_FONT_BYTES).expect("Unable to load default font"),
    ));

    for node in style_root {
        match node {
            render::SceneNode::TextSceneNode(render::TextSceneNode {
                bounds,
                text,
                font_size: _font_size,
                text_color: _text_color,
            }) => {
                let mut offset = Point { x: bounds.x, y: bounds.y };
                for text_char in text.chars() {
                    if let Some(c) = font.render_character(text_char) {
                        window
                            .draw_bitmap(
                                &(offset.borrow() + &c.offset).into(),
                                &c.bitmap,
                                c.width as u32,
                            )
                            .expect("Unable to draw bitmap");
                        offset.x += c.next_char_offset;
                    }
                }
            }
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
        render(&mut window);
        thread::sleep(std::time::Duration::from_millis(20000));
        window.resize(&Rect { x: 100, y: 100, width: 200, height: 200 }).unwrap();
        thread::sleep(std::time::Duration::from_millis(2000));
    }
    wowser_glfw::terminate();
}
