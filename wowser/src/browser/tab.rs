use std::borrow::Borrow;

use crate::{
    css::{parse_css, CssDocument},
    font::{BDFFont, CachingFont},
    html::parse_html,
    render::{self, html_css_to_styles, normalize_style_nodes, style_html},
    ui::Window,
    util::{Point, Rect},
};

const DEFAULT_FONT_BYTES: &[u8] = include_bytes!("../../data/gohufont-11.bdf");

const USERAGENT_CSS: &[u8] = include_bytes!("../assets/useragent_stylesheet.css");

fn add_useragent_css(extertnal: &mut CssDocument) {
    let useragent_css = parse_css(std::str::from_utf8(USERAGENT_CSS).unwrap()).unwrap();
    extertnal.blocks.splice(0..0, useragent_css.blocks);
}

pub fn render(window: &mut Window, html_contents: &str, css_contents: &str) {
    // Parse the documents
    let html = parse_html(html_contents).unwrap();
    let mut css = parse_css(css_contents).unwrap();

    // Add useragent stylesheet
    add_useragent_css(&mut css);

    // Associate the CSS properties with individual HTML elements
    let css_styling = style_html(&html, &css);

    // Convert the HTML+CSS Properties to style nodes, an intermediary representation of styles nodes
    // with all styling, but not layout and placement resolved
    let mut style_root = html_css_to_styles(&html, &css_styling);

    // Simplifies the style nodes to make converting to scenes cleaner and handle cases like text wrapping
    // which otherwise would be treated as a single, rectangular block
    normalize_style_nodes(&mut style_root);

    // Flatten the hierarchy on nodes to a scene, which incorporates layout, sizing, text wrapping, etc. The
    // output should be pretty much ready to draw at this point
    let scene_nodes = render::style_to_scene(&style_root, 0_f32, window.get_bounds().width as f32);

    println!("{:?}", scene_nodes);
    // Wowser only supports one font right now. Eventually this may need to be lifted up with character
    // properties used in style_to_scene
    let mut font: CachingFont = CachingFont::wrap(Box::new(
        BDFFont::load(DEFAULT_FONT_BYTES).expect("Unable to load default font"),
    ));

    for node in scene_nodes {
        match node {
            render::SceneNode::TextSceneNode(render::TextSceneNode {
                bounds,
                text,
                font_size: _font_size,
                text_color,
            }) => {
                let mut offset = Point {
                    x: bounds.x,
                    y: bounds.y,
                };
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
