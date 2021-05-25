use render::normalize_style_nodes;
use wowser::startup;
use wowser::ui::Window;
use wowser::util::Rect;
use wowser::{font::BDFFont, font::CachingFont, render, util::Point};

use std::{borrow::Borrow, thread};

const DEFAULT_FONT_BYTES: &[u8] = include_bytes!("../../data/gohufont-11.bdf");

fn example_style_nodes() -> render::StyleNode {
    render::StyleNode {
        display: render::StyleNodeDisplay::Block,
        border_width: 2.0,
        border_color: render::Color::RED,
        background_color: render::Color::BLUE,
        padding: 10.0,
        margin: 5.0,
        width: render::StyleNodeDimen::Pixels(300_f32),
        child: render::StyleNodeChild::Nodes(vec![
            render::StyleNode {
                display: render::StyleNodeDisplay::Inline,
                border_width: 2.0,
                border_color: render::Color::RED,
                background_color: render::Color::BLUE,
                padding: 10.0,
                margin: 5.0,
                width: render::StyleNodeDimen::Auto,
                child: render::StyleNodeChild::Text(render::TextStyleNode {
                    text: String::from(concat!(
                        "testa textb textc textd texte textf textg texth texti ",
                        "text text text text text text text text text text text ",
                        "text text text text text text text text text text text ",
                        "text text text text text text text text text"
                    )),
                    font_size: 12.0,
                    text_color: render::Color::WHITE,
                }),
            },
            render::StyleNode {
                display: render::StyleNodeDisplay::Inline,
                border_width: 2.0,
                border_color: render::Color::RED,
                background_color: render::Color::BLUE,
                padding: 30.0,
                margin: 5.0,
                width: render::StyleNodeDimen::Auto,
                child: render::StyleNodeChild::Nodes(vec![]),
            },
            render::StyleNode {
                display: render::StyleNodeDisplay::Block,
                border_width: 1.0,
                border_color: render::Color::RED,
                background_color: render::Color::BLUE,
                padding: 5.0,
                margin: 5.0,
                width: render::StyleNodeDimen::Auto,
                child: render::StyleNodeChild::Nodes(vec![]),
            },
        ]),
    }
}

fn render(window: &mut Window) {
    let mut example_root = example_style_nodes();
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
                            .expect("Unable to draw bitmap");
                        offset.x += c.next_char_offset;
                    }
                }
            }
            render::SceneNode::RectangleSceneNode(render::RectangleSceneNode {
                bounds,
                fill: _fill,
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
                        &border_color,
                        border_width,
                    )
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
