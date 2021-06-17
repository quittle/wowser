use crate::{font::BDFFont, font::CachingFont, util::Point};

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
        &mut font,
    )
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
    font: &mut CachingFont,
) -> Vec<SceneNode> {
    if style_node.display == StyleNodeDisplay::None {
        return vec![];
    }

    let root_offset = offset
        + &Point {
            x: style_node.margin,
            y: style_node.margin,
        };
    let default_content_width = match style_node.width {
        StyleNodeDimen::Auto => match style_node.display {
            StyleNodeDisplay::Inline => 0_f32,
            StyleNodeDisplay::Block => parent_width,
            StyleNodeDisplay::None => panic!("Should never be reached"),
        },
        StyleNodeDimen::Pixels(width) => match style_node.display {
            StyleNodeDisplay::Inline => 0_f32,
            StyleNodeDisplay::Block => width,
            StyleNodeDisplay::None => panic!("Should never be reached"),
        },
    };
    let mut root = SceneNode::RectangleSceneNode(RectangleSceneNode {
        bounds: Rect {
            x: root_offset.x,
            y: root_offset.y,
            width: default_content_width + style_node.padding * 2_f32,
            height: style_node.padding * 2_f32,
        },
        fill: style_node.background_color,
        border_color: style_node.border_color,
        border_width: style_node.border_width,
    });

    let base_child_offset = &root_offset
        + &Point {
            x: style_node.padding,
            y: style_node.padding,
        };
    let mut child_offset = base_child_offset.clone();

    match &style_node.child {
        StyleNodeChild::Text(text) => {
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
                debug_assert!(false, "Expected TextSceneNode");
            }
            vec![root, child_text]
        }
        StyleNodeChild::Nodes(nodes) => {
            let mut max_child_height = 0_f32;

            let mut ret = vec![root];

            // Parent width to pass to children. Use parent width unless a block has explicitly set width
            let (parent_left_for_children, parent_width_for_children): (f32, f32) =
                if let StyleNodeDimen::Pixels(my_width) = style_node.width {
                    if style_node.display.is_block() {
                        (child_offset.x, my_width)
                    } else {
                        (parent_left, parent_width)
                    }
                } else {
                    (parent_left, parent_width)
                };

            for node in nodes {
                let new_children = style_to_scene_r(
                    node,
                    &child_offset,
                    parent_left_for_children,
                    parent_width_for_children,
                    font,
                );
                if let Some(child) = new_children.first() {
                    child_offset += Point {
                        x: child.bounds().width + node.margin * 2_f32,
                        y: 0_f32,
                    };
                    max_child_height =
                        max_child_height.max(child.bounds().height + node.margin * 2_f32);
                }
                ret.extend(new_children);
                let last_child = ret.last().unwrap().bounds();
                child_offset.x = last_child.right();
                child_offset.y = last_child.y;
            }

            let mut root_bounds = ret.first_mut().unwrap().mut_bounds();
            root_bounds.width += child_offset.x - base_child_offset.x;
            root_bounds.height += max_child_height;

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

fn calculate_text_width(text: &str, _font_size: f32, caching_font: &mut CachingFont) -> f32 {
    let mut ret = 0_f32;
    for char in text.chars() {
        ret += caching_font
            .render_character(char)
            .map(|c| c.width)
            .unwrap_or(0_f32);
    }
    ret
}
