use render::normalize_style_nodes;
use wowser::css::parse_css;
use wowser::html::parse_html;
use wowser::render::html_css_to_styles;
use wowser::render::style_html;
use wowser::startup;
use wowser::ui::Window;
use wowser::util::Rect;
use wowser::{font::BDFFont, font::CachingFont, render, util::Point};

use std::{borrow::Borrow, thread};

const DEFAULT_FONT_BYTES: &[u8] = include_bytes!("../../data/gohufont-11.bdf");

fn render(window: &mut Window) {
    let html = parse_html(r#"<div>hello <span>world</span></div>"#).unwrap();
    let css =
        parse_css(r#"div { background-color: #f00; margin: 10px; } span { background-color: #0f0; border-color: #0ff; border-width: 3px; }"#).unwrap();
    let css_styling = style_html(&html, &css);
    let mut example_root = html_css_to_styles(&html, &css_styling);
    normalize_style_nodes(&mut example_root);
    let style_root = render::style_to_scene(&example_root, 0_f32, window.get_bounds().width as f32);
    let mut font: CachingFont = CachingFont::wrap(Box::new(
        BDFFont::load(DEFAULT_FONT_BYTES).expect("Unable to load default font"),
    ));

    for node in style_root {
        match node {
            render::SceneNode::TextSceneNode(render::TextSceneNode {
                bounds,
                text,
                font_size: _font_size,
                text_color,
            }) => {
                let mut offset = Point { x: bounds.x, y: bounds.y };
                for text_char in text.chars() {
                    if let Some(c) = font.render_character(text_char) {
                        window
                            .draw_bitmap(
                                &(offset.borrow() + &c.offset).into(),
                                &c.bitmap,
                                c.width as u32,
                                &text_color,
                            )
                            .unwrap();
                        offset.x += c.next_char_offset;
                    }
                }
            }
            render::SceneNode::RectangleSceneNode(render::RectangleSceneNode {
                bounds,
                fill,
                border_color,
                border_width,
            }) => {
                window
                    .draw_rect(
                        &Rect {
                            x: bounds.x as i32,
                            y: bounds.y as i32,
                            width: bounds.width as i32,
                            height: bounds.height as i32,
                        },
                        &fill,
                        &border_color,
                        border_width,
                    )
                    .unwrap();
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
