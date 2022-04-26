use crate::{font::BDFFont, font::CachingFont, render::Color, util::Point};

use super::{
    Rect, RectangleSceneNode, SceneNode, StyleNode, StyleNodeChild, StyleNodeDimen,
    StyleNodeDisplay, TextSceneNode, TextStyleNode,
};

/// This is the default font used for rendering
const DEFAULT_FONT_BYTES: &[u8] = include_bytes!("../../data/gohufont-11.bdf");

pub fn style_to_scene(
    style_node: &StyleNode,
    parent_left: f32,
    parent_width: f32,
) -> Vec<SceneNode> {
    let mut font: CachingFont = CachingFont::wrap(Box::new(
        BDFFont::load(DEFAULT_FONT_BYTES).expect("Unable to load default font"),
    ));
    style_to_scene_r(
        style_node,
        &Point { x: 0_f32, y: 0_f32 },
        parent_left,
        parent_width,
        0_f32,
        &mut font,
    )
    .into_iter()
    .filter(|node| match node {
        SceneNode::RectangleSceneNode(rect_node) => {
            !rect_node.fill.is_transparent()
                || !rect_node.fill_pixels.is_empty()
                || !rect_node.border_color.is_transparent()
                || rect_node.bounds.width == 0_f32
                || rect_node.bounds.height == 0_f32
        }
        SceneNode::TextSceneNode(text_node) => {
            !text_node.text_color.is_transparent() || text_node.text.is_empty()
        }
    })
    .collect()
}

// Notes:
// You can be smaller than your parent
// If block && width && wider than parent && children wider than self -> Use width
// If block && width && wider than parent && children small than self -> Use width
// If block && width && smaller than parent && children wider than self -> Use width
// If block && !width -> Match first parent block
// If inline && (width || !width) && contains inline -> min(first parent block, children-width)
// If inline && contains block -> min(first parent block, children-width)
fn style_to_scene_r(
    style_node: &StyleNode,
    offset: &Point<f32>,
    parent_left: f32,
    parent_width: f32,
    prev_node_bottom: f32,
    font: &mut CachingFont,
) -> Vec<SceneNode> {
    if style_node.display == StyleNodeDisplay::None {
        return vec![];
    }

    let margin_offset = Point {
        x: style_node.margin.left,
        y: style_node.margin.top,
    };
    let root_offset = match style_node.display {
        StyleNodeDisplay::Block => {
            Point {
                x: parent_left,
                y: prev_node_bottom,
            } + margin_offset
        }
        StyleNodeDisplay::Inline => offset + margin_offset,
        StyleNodeDisplay::None => unreachable!(),
    };
    let default_content_width = match style_node.width {
        StyleNodeDimen::Auto => match style_node.display {
            StyleNodeDisplay::Inline => 0_f32,
            StyleNodeDisplay::Block => parent_width,
            StyleNodeDisplay::None => unreachable!(),
        },
        StyleNodeDimen::Pixels(width) => match style_node.display {
            StyleNodeDisplay::Inline => 0_f32,
            StyleNodeDisplay::Block => width,
            StyleNodeDisplay::None => unreachable!(),
        },
    };
    let mut root = SceneNode::RectangleSceneNode(RectangleSceneNode {
        bounds: Rect {
            x: root_offset.x,
            y: root_offset.y,
            width: default_content_width + style_node.padding.horizontal(),
            height: style_node.padding.vertical(),
        },
        fill: style_node.background_color,
        fill_pixels: vec![],
        border_color: style_node.border_color,
        border_width: style_node.border_width,
    });

    let base_child_offset = &root_offset
        + &Point {
            x: style_node.padding.left,
            y: style_node.padding.top,
        };
    let mut child_offset = base_child_offset.clone();

    match &style_node.child {
        StyleNodeChild::Text(text) => {
            if text.affects_layout() {
                let mut child_text: SceneNode = text_style_to_scene(text, &child_offset, font);
                if let SceneNode::TextSceneNode(text_node) = &mut child_text {
                    if text_node.bounds.right() > parent_left + parent_width
                        && text_node.bounds.x != 0_f32
                    {
                        text_node.bounds.x = parent_left;
                        text_node.bounds.y += text_node.font_size;

                        root.mut_bounds().x = parent_left;
                        root.mut_bounds().y += text_node.font_size;

                        child_offset.x = root.bounds().right();
                        child_offset.y = root.bounds().y;
                    }

                    root.mut_bounds().width += text_node.bounds.width;
                    root.mut_bounds().height += text_node.font_size;
                } else {
                    unreachable!("Expected TextSceneNode");
                }
                vec![root, child_text]
            } else {
                vec![root]
            }
        }
        StyleNodeChild::Native(native) => {
            let child = SceneNode::RectangleSceneNode(RectangleSceneNode {
                bounds: Rect {
                    x: offset.x,
                    y: offset.y,
                    width: native.width as f32,
                    height: native.height as f32,
                },
                border_color: Color::TRANSPARENT,
                border_width: 0_f32,
                fill: Color::TRANSPARENT,
                fill_pixels: native.pixels.clone(),
            });

            root.mut_bounds().height += native.height as f32;
            root.mut_bounds().width += native.width as f32;

            vec![root, child]
        }
        StyleNodeChild::Nodes(nodes) => {
            let mut max_child_bottom =
                prev_node_bottom + style_node.margin.top + style_node.padding.top;

            let mut ret = vec![root];

            // Parent width to pass to children. Use parent width unless a block has explicitly set width
            let (parent_left_for_children, parent_width_for_children): (f32, f32) =
                if let StyleNodeDimen::Pixels(my_width) = style_node.width {
                    if style_node.display.is_block() {
                        (child_offset.x, my_width)
                    } else {
                        (
                            parent_left + style_node.margin.left + style_node.padding.left,
                            (parent_width - style_node.margin.horizontal())
                                - style_node.padding.horizontal(),
                        )
                    }
                } else {
                    (
                        parent_left + style_node.margin.left + style_node.padding.left,
                        (parent_width - style_node.margin.horizontal())
                            - style_node.padding.horizontal(),
                    )
                };

            for node in nodes {
                let new_children = style_to_scene_r(
                    node,
                    &child_offset,
                    parent_left_for_children,
                    parent_width_for_children,
                    max_child_bottom,
                    font,
                );
                if let Some(first_child) = new_children.first() {
                    let bounds = first_child.bounds();
                    // If the prev node doesn't take up space, don't worry about wrapping blocks
                    if bounds.area() != 0_f32 {
                        child_offset += Point {
                            x: bounds.width + node.margin.horizontal() + node.padding.horizontal(),
                            y: 0_f32,
                        };
                        max_child_bottom =
                            max_child_bottom.max(bounds.bottom() + node.margin.bottom);
                    }
                }

                ret.extend(new_children);
                let last_child = ret.last().unwrap().bounds();
                child_offset.x = last_child.right();
                child_offset.y = last_child.y;
            }

            let mut root_bounds = ret.first_mut().unwrap().mut_bounds();
            root_bounds.width += child_offset.x - base_child_offset.x;
            root_bounds.height = max_child_bottom + style_node.padding.bottom - root_bounds.top();

            ret
        }
    }
}

fn text_style_to_scene(
    node: &TextStyleNode,
    offset: &Point<f32>,
    font: &mut CachingFont,
) -> SceneNode {
    SceneNode::TextSceneNode(TextSceneNode {
        bounds: Rect {
            x: offset.x,
            y: offset.y,
            width: calculate_text_width(&node.text, node.font_size, font),
            height: node.font_size,
        },
        text: node.text.clone(),
        font_size: node.font_size,
        text_color: node.text_color,
    })
}

fn calculate_text_width(text: &str, font_size: f32, caching_font: &mut CachingFont) -> f32 {
    let mut ret = 0_f32;
    for char in text.chars() {
        ret += caching_font
            .render_character(char, font_size)
            .map(|c| c.next_char_offset)
            .unwrap_or(0_f32);
    }
    ret
}
