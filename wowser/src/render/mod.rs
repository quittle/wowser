//! Partial implementation. Not ready for any real use

mod rect;

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

static COLOR_RED: Color = Color {r: 255, g: 0, b: 0, a: 255};
static COLOR_BLUE: Color = Color {r: 0, g: 0, b: 255, a: 255};
static COLOR_BLACK: Color = Color {r: 0, g: 0, b: 0, a: 255};
static COLOR_TRANSPARENT: Color = Color {r: 0, g: 0, b: 0, a: 0};

struct StyleNode {
    // bounds: Rect,
    z_index: i32,
    border_width: f32,
    border_color: Color,
    background_color: Color,
    padding: f32,
    margin: f32,
    child: StyleNodeChild,
}

struct TextStyleNode {
    // bounds: Rect,
    text: String,
    font_size: f32,
    text_color: Color
}

enum StyleNodeChild {
    Text(TextStyleNode),
    Nodes(Vec<StyleNode>)
}

fn example_style_nodes() -> StyleNode {
    StyleNode {
        // bounds: Rect {
        //     x: 0,
        //     y: 0,
        //     width: 100,
        //     height: 100,
        // },
        z_index: 0,
        border_width: 2.0,
        border_color: COLOR_RED,
        background_color: COLOR_BLUE,
        padding: 10.0,
        margin: 5.0,
        child: StyleNodeChild::Text(TextStyleNode {
            // bounds: Rect {
            //     x: 10,
            //     y: 10,
            //     width: 50,
            //     height: 12,
            // },
            text: String::from("test text"),
            font_size: 12.0,
            text_color: COLOR_BLACK,
        })
    }
}

fn style_to_scene(style_node: &StyleNode) -> Vec<SceneNode> {
    style_to_scene_r(style_node, Point {x: 0, y: 0})
}

fn style_to_scene_r(style_node: &StyleNode, offset: Point) -> Vec<SceneNode> {

    match style_node.child {
        StyleNodeChild::Text(text) => {
            text_style_to_scene(text, offset.offset(style_node.margin, style_node.margin))
        },
        StyleNodeChild::Nodes(nodes) => {
            
        }
    }

    let root = SceneNode {
        bounds: Rect {
            x: offset.x.into() + style_node.margin,
            y: offset.y.into() + style_node.margin,
            width: 0,
            height: 0,
        },
        fill: style_node.background_color,
        border_color: style_node.border_color,
        border_width: style_node.border_width,
    };
}

fn text_style_to_scene(node: TextStyleNode, offset: Point) -> SceneNode {
    SceneNode::TextSceneNode(TextSceneNode {
        bounds: Rect {
            x: offset.x,
            y: offset.y,
            width: calculate_text_width(&node.text, node.font_size),
            height: node.font_size,
        },
        text: node.text,
        font_size: node.font_size,
        text_color: node.text_color,
    })
}

fn calculate_text_width(text: &String, font_size: f32) -> f32 {
    0
}

enum SceneNode {
    TextSceneNode(TextSceneNode),
    RectangleSceneNode(RectangleSceneNode),
}

struct TextSceneNode {
    bounds: Rect,
    text: String,
    font_size: f32,
    text_color: Color,
}

struct RectangleSceneNode {
    bounds: Rect,
    fill: Color,
    border_color: Color,
    border_width: f32,
}

pub use rect::{Point, Rect};
