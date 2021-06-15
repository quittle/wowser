use std::{collections::HashMap, ptr::addr_of};

use crate::{
    css::{CssBackgroundColor, CssDisplay, CssProperty},
    html::{DocumentHtmlNode, ElementContents},
    render::{Color, StyleNode, StyleNodeChild, StyleNodeDisplay, TextStyleNode},
};

pub fn html_css_to_styles(
    html_document: &DocumentHtmlNode,
    styles: &HashMap<*const ElementContents, Vec<&CssProperty>>,
) -> StyleNode {
    let mut root = StyleNode::new_default(StyleNodeDisplay::Block);
    root.child = StyleNodeChild::Nodes(
        html_document.contents.iter().map(|child| render(styles, child)).collect(),
    );
    root
}

fn render(
    styles: &HashMap<*const ElementContents, Vec<&CssProperty>>,
    element: &ElementContents,
) -> StyleNode {
    let mut style_node = if let Some(props) = styles.get(&addr_of!(*element)) {
        let display = match get_style_prop(
            props,
            "display",
            CssDisplay::from_raw_value,
            CssDisplay::Inline,
        ) {
            CssDisplay::Block => StyleNodeDisplay::Block,
            CssDisplay::Inline => StyleNodeDisplay::Inline,
        };

        let mut style_node = StyleNode::new_default(display);

        style_node.background_color = match get_style_prop(
            props,
            "background-color",
            CssBackgroundColor::from_raw_value,
            CssBackgroundColor::Rgba(0, 0, 0, 0),
        ) {
            CssBackgroundColor::Rgba(r, g, b, a) => Color { r, g, b, a },
        };
        style_node
    } else {
        StyleNode::new_default(StyleNodeDisplay::Inline)
    };

    style_node.child = match element {
        ElementContents::Element(element_node) => StyleNodeChild::Nodes(
            element_node.children.iter().map(|child| render(styles, child)).collect(),
        ),

        ElementContents::Text(text_node) => StyleNodeChild::Text(TextStyleNode {
            text: text_node.text.clone(),
            text_color: Color::WHITE, // TODO: Parse from styles
            font_size: 12_f32,        // TODO: Parse from styles
        }),
    };

    style_node
}

fn find_prop<'a>(props: &[&'a CssProperty], key: &str) -> Option<&'a String> {
    props
        .iter()
        // Last property takes precedence
        .rev()
        .find(|prop| prop.key == key)
        .map(|prop| &prop.value)
}

fn get_style_prop<T, F: Fn(&str) -> Option<T>>(
    props: &[&CssProperty],
    property_name: &str,
    from_raw_value: F,
    default_value: T,
) -> T {
    if let Some(property_value) = find_prop(props, property_name) {
        if let Some(materialized_value) = from_raw_value(property_value) {
            materialized_value
        } else {
            default_value
        }
    } else {
        default_value
    }
}
