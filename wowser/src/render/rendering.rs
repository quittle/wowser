use crate::{font::BDFFont, font::CachingFont, util::Point};

use super::{
    Rect, RectangleSceneNode, SceneNode, StyleNode, StyleNodeChild, TextSceneNode, TextStyleNode,
};

pub fn style_to_scene(style_node: &StyleNode) -> Vec<SceneNode> {
    let mut nodes = style_to_scene_r(style_node, &Point { x: 0_f32, y: 0_f32 });
    nodes.reverse();
    nodes
}

fn style_to_scene_r(style_node: &StyleNode, offset: &Point<f32>) -> Vec<SceneNode> {
    let root_offset = offset + &Point { x: style_node.margin, y: style_node.margin };
    let mut root = SceneNode::RectangleSceneNode(RectangleSceneNode {
        bounds: Rect {
            x: root_offset.x,
            y: root_offset.y,
            width: style_node.padding * 2_f32,
            height: style_node.padding * 2_f32,
        },
        fill: style_node.background_color,
        border_color: style_node.border_color,
        border_width: style_node.border_width,
    });

    let base_child_offset = root_offset + Point { x: style_node.padding, y: style_node.padding };
    let mut child_offset = base_child_offset.clone();

    match &style_node.child {
        StyleNodeChild::Text(text) => {
            let child_text = text_style_to_scene(text, &child_offset);
            root.mut_bounds().width += child_text.bounds().width;
            root.mut_bounds().height += child_text.bounds().height;
            vec![root, child_text]
        }
        StyleNodeChild::Nodes(nodes) => {
            let mut max_child_height = 0_f32;

            let mut ret = vec![root];

            for node in nodes {
                let new_children = style_to_scene_r(node, &child_offset);
                if let Some(child) = new_children.first() {
                    child_offset +=
                        Point { x: child.bounds().width + node.margin * 2_f32, y: 0_f32 };
                    max_child_height =
                        max_child_height.max(child.bounds().height + node.margin * 2_f32);
                }
                ret.extend(new_children);
            }

            let mut root_bounds = ret.first_mut().expect("").mut_bounds();
            root_bounds.width += child_offset.x - base_child_offset.x;
            root_bounds.height += max_child_height;

            ret
        }
    }
}

const DEFAULT_FONT_BYTES: &[u8] = include_bytes!("../../data/gohufont-11.bdf");

fn text_style_to_scene(node: &TextStyleNode, offset: &Point<f32>) -> SceneNode {
    let mut font: CachingFont = CachingFont::wrap(Box::new(
        BDFFont::load(DEFAULT_FONT_BYTES).expect("Unable to load default font"),
    ));
    SceneNode::TextSceneNode(TextSceneNode {
        bounds: Rect {
            x: offset.x,
            y: offset.y,
            width: calculate_text_width(&node.text, node.font_size, &mut font),
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
        ret += caching_font.render_character(char).map(|c| c.width).unwrap_or(0_f32);
    }
    ret
}
