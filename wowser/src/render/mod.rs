//! Partial implementation. Not ready for any real use

mod point;
mod rect;

use point::Point;
use rect::Rect;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

static COLOR_RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};
static COLOR_BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 255,
    a: 255,
};
static COLOR_BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};
static COLOR_TRANSPARENT: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};

pub struct StyleNode {
    // bounds: Rect,
    z_index: i32,
    border_width: f32,
    border_color: Color,
    background_color: Color,
    padding: f32,
    margin: f32,
    child: StyleNodeChild,
}

pub struct TextStyleNode {
    // bounds: Rect,
    text: String,
    font_size: f32,
    text_color: Color,
}

pub enum StyleNodeChild {
    Text(TextStyleNode),
    Nodes(Vec<StyleNode>),
}

pub fn example_style_nodes() -> StyleNode {
    StyleNode {
        z_index: 0,
        border_width: 2.0,
        border_color: COLOR_RED,
        background_color: COLOR_BLUE,
        padding: 10.0,
        margin: 5.0,
        child: StyleNodeChild::Nodes(vec![
            StyleNode {
                z_index: 0,
                border_width: 2.0,
                border_color: COLOR_RED,
                background_color: COLOR_BLUE,
                padding: 10.0,
                margin: 5.0,
                child: StyleNodeChild::Text(TextStyleNode {
                    text: String::from("test text"),
                    font_size: 12.0,
                    text_color: COLOR_BLACK,
                }),
            },
            StyleNode {
                z_index: 0,
                border_width: 2.0,
                border_color: COLOR_RED,
                background_color: COLOR_BLUE,
                padding: 30.0,
                margin: 5.0,
                child: StyleNodeChild::Nodes(vec![]),
            },
        ]),
    }
}

pub fn style_to_scene(style_node: &StyleNode) -> Vec<SceneNode> {
    let mut nodes = style_to_scene_r(style_node, &Point { x: 0_f32, y: 0_f32 });
    nodes.reverse();
    nodes
}

fn style_to_scene_r(style_node: &StyleNode, offset: &Point) -> Vec<SceneNode> {
    let root_offset = offset.offset(style_node.margin, style_node.margin);
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

    let base_child_offset = root_offset.offset(style_node.padding, style_node.padding);
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
                    child_offset =
                        child_offset.offset(child.bounds().width + node.margin * 2_f32, 0_f32);
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

fn text_style_to_scene(node: &TextStyleNode, offset: &Point) -> SceneNode {
    SceneNode::TextSceneNode(TextSceneNode {
        bounds: Rect {
            x: offset.x,
            y: offset.y,
            width: calculate_text_width(&node.text, node.font_size),
            height: node.font_size,
        },
        text: node.text.clone(),
        font_size: node.font_size,
        text_color: node.text_color,
    })
}

fn calculate_text_width(text: &String, font_size: f32) -> f32 {
    0_f32
}

pub enum SceneNode {
    TextSceneNode(TextSceneNode),
    RectangleSceneNode(RectangleSceneNode),
}

impl SceneNode {
    pub fn bounds(&self) -> &Rect {
        match self {
            Self::TextSceneNode(TextSceneNode { bounds, .. }) => bounds,
            Self::RectangleSceneNode(RectangleSceneNode { bounds, .. }) => bounds,
        }
    }

    pub fn mut_bounds(&mut self) -> &mut Rect {
        match self {
            Self::TextSceneNode(TextSceneNode { bounds, .. }) => bounds,
            Self::RectangleSceneNode(RectangleSceneNode { bounds, .. }) => bounds,
        }
    }
}

pub struct TextSceneNode {
    pub bounds: Rect,
    pub text: String,
    pub font_size: f32,
    pub text_color: Color,
}

pub struct RectangleSceneNode {
    pub bounds: Rect,
    pub fill: Color,
    pub border_color: Color,
    pub border_width: f32,
}
