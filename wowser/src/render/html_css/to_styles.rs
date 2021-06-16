use std::{collections::HashMap, ptr::addr_of};

use crate::{
    css::{CssColor, CssDimension, CssDisplay, CssProperty},
    html::{DocumentHtmlNode, ElementContents},
    render::{Color, StyleNode, StyleNodeChild, StyleNodeDisplay, TextStyleNode},
};

pub fn html_css_to_styles(
    html_document: &DocumentHtmlNode,
    styles: &HashMap<*const ElementContents, Vec<&CssProperty>>,
) -> StyleNode {
    let mut root = StyleNode::new_default(StyleNodeDisplay::Block);
    let inherited_styles = InheritedStyles::default();
    root.child = StyleNodeChild::Nodes(
        html_document
            .contents
            .iter()
            .map(|child| render(styles, &inherited_styles, child))
            .collect(),
    );
    root
}

/// These are styles that may be passed down from node to child
/// without actually having to be attributed to nodes themselves.
#[derive(Default, Clone)]
struct InheritedStyles {
    text_color: Color,
}

fn render(
    styles: &HashMap<*const ElementContents, Vec<&CssProperty>>,
    inherited_styles: &InheritedStyles,
    element: &ElementContents,
) -> StyleNode {
    let mut cur_inherited_styles = inherited_styles.clone();
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
            CssColor::from_raw_value,
            CssColor::Rgba(0, 0, 0, 0),
        ) {
            CssColor::Rgba(r, g, b, a) => Color { r, g, b, a },
        };

        style_node.border_width = match get_style_prop(
            props,
            "border-width",
            CssDimension::from_raw_value,
            CssDimension::Px(0_f32),
        ) {
            CssDimension::Px(px) => px,
        };

        style_node.border_color = match get_style_prop(
            props,
            "border-color",
            CssColor::from_raw_value,
            CssColor::Rgba(0, 0, 0, 0),
        ) {
            CssColor::Rgba(r, g, b, a) => Color { r, g, b, a },
        };

        style_node.margin = match get_style_prop(
            props,
            "margin",
            CssDimension::from_raw_value,
            CssDimension::Px(0.0),
        ) {
            CssDimension::Px(px) => px,
        };

        if let Some(text_color) = maybe_get_style_prop(props, "color", CssColor::from_raw_value) {
            cur_inherited_styles.text_color = match text_color {
                CssColor::Rgba(r, g, b, a) => Color { r, g, b, a },
            };
        }

        style_node
    } else {
        StyleNode::new_default(StyleNodeDisplay::Inline)
    };

    style_node.child = match element {
        ElementContents::Element(element_node) => StyleNodeChild::Nodes(
            element_node
                .children
                .iter()
                .map(|child| render(styles, &cur_inherited_styles, child))
                .collect(),
        ),

        ElementContents::Text(text_node) => StyleNodeChild::Text(TextStyleNode {
            text: text_node.text.clone(),
            text_color: cur_inherited_styles.text_color,
            font_size: 12_f32, // TODO: Parse from styles
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

fn maybe_get_style_prop<T, F: Fn(&str) -> Option<T>>(
    props: &[&CssProperty],
    property_name: &str,
    from_raw_value: F,
) -> Option<T> {
    find_prop(props, property_name)
        .iter()
        .flat_map(|property_value| from_raw_value(property_value))
        .last()
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
