mod color;
mod rect;

use crate::{font::BDFFont, font::CachingFont, util::Point};
use color::*;
use rect::Rect;

#[derive(Eq, PartialEq, Debug)]
pub enum StyleNodeDisplay {
    Inline,
    Block,
}

#[derive(Debug, PartialEq)]
pub struct StyleNode {
    // bounds: Rect,
    display: StyleNodeDisplay,
    border_width: f32,
    border_color: Color,
    background_color: Color,
    padding: f32,
    margin: f32,
    child: StyleNodeChild,
}

impl StyleNode {
    fn new_default(display: StyleNodeDisplay) -> Self {
        StyleNode {
            display,
            border_width: 0_f32,
            border_color: Color::TRANSPARENT,
            background_color: Color::TRANSPARENT,
            padding: 0_f32,
            margin: 0_f32,
            child: StyleNodeChild::Nodes(vec![]),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextStyleNode {
    // bounds: Rect,
    text: String,
    font_size: f32,
    text_color: Color,
}

#[derive(Debug, PartialEq)]
pub enum StyleNodeChild {
    Text(TextStyleNode),
    Nodes(Vec<StyleNode>),
}

pub fn example_style_nodes() -> StyleNode {
    StyleNode {
        display: StyleNodeDisplay::Block,
        border_width: 2.0,
        border_color: Color::RED,
        background_color: Color::BLUE,
        padding: 10.0,
        margin: 5.0,
        child: StyleNodeChild::Nodes(vec![
            StyleNode {
                display: StyleNodeDisplay::Inline,
                border_width: 2.0,
                border_color: Color::RED,
                background_color: Color::BLUE,
                padding: 10.0,
                margin: 5.0,
                child: StyleNodeChild::Text(TextStyleNode {
                    text: String::from("test text text text text text text text text text text text text text text text text text text text text text text text text text text text text text text text text text text text text text text text"),
                    font_size: 12.0,
                    text_color: Color::BLACK,
                }),
            },
            StyleNode {
                display: StyleNodeDisplay::Inline,
                border_width: 2.0,
                border_color: Color::RED,
                background_color: Color::BLUE,
                padding: 30.0,
                margin: 5.0,
                child: StyleNodeChild::Nodes(vec![]),
            },
            StyleNode {
                display: StyleNodeDisplay::Block,
                border_width: 1.0,
                border_color: Color::RED,
                background_color: Color::BLUE,
                padding: 5.0,
                margin: 5.0,
                child: StyleNodeChild::Nodes(vec![]),
            },
        ]),
    }
}

/// Transform a StyleNode and its children recursively to be as explicit as possible
/// following html layout rules suitable for simple rendering.
pub fn normalize_style_nodes(style_node: &mut StyleNode) -> &mut StyleNode {
    let background_color = style_node.background_color;
    match &mut style_node.child {
        StyleNodeChild::Text(text_style_node) => {
            // Split text on spaces to enable word wrapping
            if text_style_node.text.contains(' ') {
                let split_text = text_style_node.text.split(' ');
                style_node.child = StyleNodeChild::Nodes(
                    split_text
                        .map(|chunk| {
                            let mut node: TextStyleNode = text_style_node.clone();
                            node.text = chunk.to_string();
                            StyleNode {
                                display: StyleNodeDisplay::Inline,
                                border_width: 0_f32,
                                border_color: Color::TRANSPARENT,
                                background_color,
                                padding: 0_f32,
                                margin: 0_f32,
                                child: StyleNodeChild::Text(node),
                            }
                        })
                        .collect(),
                );
            }
        }
        StyleNodeChild::Nodes(nodes) => {
            let mut contains_block_nodes = false;

            for node in nodes.iter_mut() {
                normalize_style_nodes(node);
                if let StyleNodeDisplay::Block = node.display {
                    contains_block_nodes = true;
                }
            }

            // Inline blocks should only contain all inline or block nodes. When mixed,
            // a pseudo, block element can be generated to wrap each stream of inline elements.
            if contains_block_nodes {
                if let StyleNodeDisplay::Inline = style_node.display {
                    let mut replaced_nodes: Vec<StyleNode> = vec![];
                    let mut inline_nodes: Vec<StyleNode> = vec![];
                    for node in nodes.drain(..) {
                        if let StyleNodeDisplay::Inline = node.display {
                            inline_nodes.push(node);
                        } else {
                            if !inline_nodes.is_empty() {
                                let mut pseudo_block =
                                    StyleNode::new_default(StyleNodeDisplay::Block);
                                pseudo_block.child = StyleNodeChild::Nodes(inline_nodes);
                                replaced_nodes.push(pseudo_block);
                                inline_nodes = vec![];
                            }
                            replaced_nodes.push(node);
                        }
                    }
                    // Make sure to include the final length of inline nodes.
                    if !inline_nodes.is_empty() {
                        let mut pseudo_block = StyleNode::new_default(StyleNodeDisplay::Block);
                        pseudo_block.child = StyleNodeChild::Nodes(inline_nodes);
                        replaced_nodes.push(pseudo_block);
                    }
                    style_node.child = StyleNodeChild::Nodes(replaced_nodes);
                }
            }
        }
    }
    style_node
}

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

#[cfg(test)]
mod tests {
    use super::*;

    mod normalize_style_nodes {
        use super::*;

        #[test]
        fn inline_children_all_inline() {
            let mut style_node = StyleNode {
                display: StyleNodeDisplay::Inline,
                border_width: 1_f32,
                border_color: Color::RED,
                background_color: Color::BLUE,
                padding: 2_f32,
                margin: 3_f32,
                child: StyleNodeChild::Nodes(vec![
                    StyleNode::new_default(StyleNodeDisplay::Inline),
                    StyleNode::new_default(StyleNodeDisplay::Inline),
                ]),
            };

            normalize_style_nodes(&mut style_node);

            assert_eq!(
                StyleNode {
                    display: StyleNodeDisplay::Inline,
                    border_width: 1_f32,
                    border_color: Color::RED,
                    background_color: Color::BLUE,
                    padding: 2_f32,
                    margin: 3_f32,
                    child: StyleNodeChild::Nodes(vec!(
                        StyleNode::new_default(StyleNodeDisplay::Inline),
                        StyleNode::new_default(StyleNodeDisplay::Inline)
                    ))
                },
                style_node
            );
        }

        #[test]
        fn inline_children_all_block() {
            let mut style_node = StyleNode {
                display: StyleNodeDisplay::Inline,
                border_width: 1_f32,
                border_color: Color::RED,
                background_color: Color::BLUE,
                padding: 2_f32,
                margin: 3_f32,
                child: StyleNodeChild::Nodes(vec![
                    StyleNode::new_default(StyleNodeDisplay::Block),
                    StyleNode::new_default(StyleNodeDisplay::Block),
                ]),
            };

            normalize_style_nodes(&mut style_node);

            assert_eq!(
                StyleNode {
                    display: StyleNodeDisplay::Inline,
                    border_width: 1_f32,
                    border_color: Color::RED,
                    background_color: Color::BLUE,
                    padding: 2_f32,
                    margin: 3_f32,
                    child: StyleNodeChild::Nodes(vec!(
                        StyleNode::new_default(StyleNodeDisplay::Block),
                        StyleNode::new_default(StyleNodeDisplay::Block)
                    ))
                },
                style_node
            );
        }

        #[test]
        fn inline_children_mixed() {
            let mut style_node = StyleNode {
                display: StyleNodeDisplay::Inline,
                border_width: 1_f32,
                border_color: Color::RED,
                background_color: Color::BLUE,
                padding: 2_f32,
                margin: 3_f32,
                child: StyleNodeChild::Nodes(vec![
                    StyleNode::new_default(StyleNodeDisplay::Inline),
                    StyleNode::new_default(StyleNodeDisplay::Block),
                    StyleNode::new_default(StyleNodeDisplay::Inline),
                ]),
            };

            normalize_style_nodes(&mut style_node);

            assert_eq!(
                StyleNode {
                    display: StyleNodeDisplay::Inline,
                    border_width: 1_f32,
                    border_color: Color::RED,
                    background_color: Color::BLUE,
                    padding: 2_f32,
                    margin: 3_f32,
                    child: StyleNodeChild::Nodes(vec!(
                        StyleNode {
                            display: StyleNodeDisplay::Block,
                            border_width: 0_f32,
                            border_color: Color::TRANSPARENT,
                            background_color: Color::TRANSPARENT,
                            padding: 0_f32,
                            margin: 0_f32,
                            child: StyleNodeChild::Nodes(vec!(StyleNode::new_default(
                                StyleNodeDisplay::Inline
                            )))
                        },
                        StyleNode::new_default(StyleNodeDisplay::Block),
                        StyleNode {
                            display: StyleNodeDisplay::Block,
                            border_width: 0_f32,
                            border_color: Color::TRANSPARENT,
                            background_color: Color::TRANSPARENT,
                            padding: 0_f32,
                            margin: 0_f32,
                            child: StyleNodeChild::Nodes(vec!(StyleNode::new_default(
                                StyleNodeDisplay::Inline
                            )))
                        },
                    ))
                },
                style_node
            );
        }

        #[test]
        fn splits_strings() {
            let mut style_node = StyleNode {
                display: StyleNodeDisplay::Inline,
                border_width: 1_f32,
                border_color: Color::RED,
                background_color: Color::BLUE,
                padding: 2_f32,
                margin: 3_f32,
                child: StyleNodeChild::Text(TextStyleNode {
                    text: "text with spaces".to_string(),
                    font_size: 4_f32,
                    text_color: Color::GREEN,
                }),
            };

            normalize_style_nodes(&mut style_node);

            assert_eq!(
                style_node,
                StyleNode {
                    display: StyleNodeDisplay::Inline,
                    border_width: 1_f32,
                    border_color: Color::RED,
                    background_color: Color::BLUE,
                    padding: 2_f32,
                    margin: 3_f32,
                    child: StyleNodeChild::Nodes(vec!(
                        StyleNode {
                            display: StyleNodeDisplay::Inline,
                            border_width: 0_f32,
                            border_color: Color::TRANSPARENT,
                            background_color: Color::BLUE,
                            padding: 0_f32,
                            margin: 0_f32,
                            child: StyleNodeChild::Text(TextStyleNode {
                                text: "text".to_string(),
                                font_size: 4_f32,
                                text_color: Color::GREEN
                            })
                        },
                        StyleNode {
                            display: StyleNodeDisplay::Inline,
                            border_width: 0_f32,
                            border_color: Color::TRANSPARENT,
                            background_color: Color::BLUE,
                            padding: 0_f32,
                            margin: 0_f32,
                            child: StyleNodeChild::Text(TextStyleNode {
                                text: "with".to_string(),
                                font_size: 4_f32,
                                text_color: Color::GREEN
                            })
                        },
                        StyleNode {
                            display: StyleNodeDisplay::Inline,
                            border_width: 0_f32,
                            border_color: Color::TRANSPARENT,
                            background_color: Color::BLUE,
                            padding: 0_f32,
                            margin: 0_f32,
                            child: StyleNodeChild::Text(TextStyleNode {
                                text: "spaces".to_string(),
                                font_size: 4_f32,
                                text_color: Color::GREEN
                            })
                        },
                    ))
                }
            );
        }
    }
}
