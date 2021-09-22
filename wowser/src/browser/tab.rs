use std::borrow::Borrow;

use crate::{
    css::{parse_css, CssDocument},
    font::{BDFFont, CachingFont},
    html::{parse_html, HtmlDocument},
    render::{self, html_css_to_styles, normalize_style_nodes, style_html, AsyncRenderContext},
    ui::Window,
    util::{Point, Rect},
};

const DEFAULT_FONT_BYTES: &[u8] = include_bytes!("../../data/gohufont-11.bdf");

const USERAGENT_CSS: &[u8] = include_bytes!("../assets/useragent_stylesheet.css");

fn get_useragent_css() -> CssDocument {
    parse_css(std::str::from_utf8(USERAGENT_CSS).unwrap()).unwrap()
}

pub struct Tab<'w> {
    window: &'w mut Window,
    html: HtmlDocument,
    async_render_context: AsyncRenderContext,
}

impl<'w> Tab<'w> {
    pub fn load(window: &'w mut Window, html_contents: &str) -> Tab<'w> {
        // Parse the documents
        let html = parse_html(html_contents).unwrap();

        // Get useragent stylesheet
        let css = get_useragent_css();

        let mut async_render_context = AsyncRenderContext::default();
        async_render_context
            .css_documents
            .insert("USER-AGENT".to_string(), (0, css));

        Tab {
            window,
            html,
            async_render_context,
        }
    }

    pub fn render(&mut self) {
        render_once(self.window, &self.html, &mut self.async_render_context);
    }
}

fn render_once(
    window: &mut Window,
    html: &HtmlDocument,
    async_render_context: &mut AsyncRenderContext,
) {
    let mut sorted_css_documents = async_render_context
        .css_documents
        .iter()
        .collect::<Vec<_>>();
    sorted_css_documents.sort_by_key(|entry| entry.1 .0);
    let css_blocks: Vec<_> = sorted_css_documents
        .iter()
        .flat_map(|css_document| css_document.1 .1.blocks.clone())
        .collect();
    let merged_css_document = CssDocument { blocks: css_blocks };

    let css_styling = &style_html(html, &merged_css_document);

    // Convert the HTML+CSS Properties to style nodes, an intermediary representation of styles nodes
    // with all styling, but not layout and placement resolved
    let mut style_root = html_css_to_styles(html, css_styling, async_render_context);

    // Simplifies the style nodes to make converting to scenes cleaner and handle cases like text wrapping
    // which otherwise would be treated as a single, rectangular block
    normalize_style_nodes(&mut style_root);

    // Flatten the hierarchy on nodes to a scene, which incorporates layout, sizing, text wrapping, etc. The
    // output should be pretty much ready to draw at this point
    let scene_nodes = render::style_to_scene(&style_root, 0_f32, window.get_bounds().width as f32);

    // Wowser only supports one font right now. Eventually this may need to be lifted up with character
    // properties used in style_to_scene
    let mut font: CachingFont = CachingFont::wrap(Box::new(
        BDFFont::load(DEFAULT_FONT_BYTES).expect("Unable to load default font"),
    ));

    let mut window_mutator = window.mutate();

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
                        window_mutator
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
                fill_pixels,
                border_color,
                border_width,
            }) => {
                window_mutator
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
                if !fill_pixels.is_empty() {
                    window_mutator
                        .draw_pixels(
                            &Point {
                                x: bounds.x as i32,
                                y: bounds.y as i32,
                            },
                            &fill_pixels,
                            bounds.width as usize,
                        )
                        .unwrap()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ui::tests::lock_for_ui_threads;
    use crate::util::get_bool_env;
    use crate::{function_name, startup};
    use std::{
        env, fs,
        panic::{catch_unwind, UnwindSafe},
    };

    use super::*;

    fn get_test_file(function_name: &'static str) -> String {
        format!("src/browser/test_data/{}.rgb", function_name)
    }

    /// If these tests fail and you have verified the failures were expected, set the
    /// WOWSER_UPDATE_TESTS env variable to true and re-run to automatically update them expected
    /// values.
    fn screenshot_test<F>(function_name: &'static str, setup: F)
    where
        F: FnOnce(&mut Window) + UnwindSafe,
    {
        let should_update_tests = get_bool_env("WOWSER_UPDATE_TESTS", false);
        lock_for_ui_threads(|| {
            let result = catch_unwind(|| {
                startup::start();
                let mut window = Window::new().unwrap();
                window
                    .resize(&Rect {
                        x: 0,
                        y: 0,
                        width: 150,
                        height: 150,
                    })
                    .unwrap();
                setup(&mut window);
                let actual_pixels = window.get_pixels_rgb().unwrap();
                let expected_pixels_file = get_test_file(function_name);
                let expected_pixels = fs::read(&expected_pixels_file).unwrap_or_default();
                if actual_pixels != expected_pixels {
                    if should_update_tests {
                        log!(INFO: "Updating screenshot for", expected_pixels_file);
                        fs::write(expected_pixels_file, &actual_pixels).unwrap();
                    } else {
                        let actual_pixels_file =
                            env::temp_dir().join(format!("{}.rgb", function_name));
                        fs::write(&actual_pixels_file, &actual_pixels).unwrap();
                        panic!(
                        "Pixels don't line up. Compare expected pixles in {} with actual pixels in {} to see the difference",
                        &expected_pixels_file,
                        actual_pixels_file.to_str().unwrap(),
                    );
                    }
                }
            });
            startup::stop();
            result.unwrap();
        });
    }

    #[test]
    fn test_blank_render() {
        screenshot_test(function_name!(), |_window| {
            // Default test
        });
    }

    #[test]
    fn test_minimal_html() {
        screenshot_test(function_name!(), |window| {
            // Currently buggy because it doesn't render a white background by default
            Tab::load(window, "").render();
        });
    }

    #[test]
    fn test_layout() {
        let html = r#"
            <html>
                <head>
                    Content Ignored
                    <style>
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
                    </style>
                </head>
                <div class="wrapper">
                    <div>abc<span>def</span>ghi</div>
                    <div class="foo">bbbbbbb</div>
                </div>
            </html>
        "#;
        screenshot_test(function_name!(), |window| {
            window
                .resize(&Rect {
                    x: 0,
                    y: 0,
                    width: 200,
                    height: 100,
                })
                .unwrap();

            let mut tab = Tab::load(window, html);
            tab.render();
            // Second render adds support for style
            tab.render();
        });
    }
}
