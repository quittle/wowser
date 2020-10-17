use super::color::Color;

/// Represents a DOM node after attaching all applicable CSS styles.
#[derive(Debug, PartialEq)]
pub struct StyleNode {
    // pub bounds: Rect,
    pub display: StyleNodeDisplay,
    pub border_width: f32,
    pub border_color: Color,
    pub background_color: Color,
    pub padding: f32,
    pub margin: f32,
    pub child: StyleNodeChild,
}

impl StyleNode {
    pub fn new_default(display: StyleNodeDisplay) -> Self {
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

#[derive(Eq, PartialEq, Debug)]
pub enum StyleNodeDisplay {
    Inline,
    Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextStyleNode {
    // pub bounds: Rect,
    pub text: String,
    pub font_size: f32,
    pub text_color: Color,
}

#[derive(Debug, PartialEq)]
pub enum StyleNodeChild {
    Text(TextStyleNode),
    Nodes(Vec<StyleNode>),
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
