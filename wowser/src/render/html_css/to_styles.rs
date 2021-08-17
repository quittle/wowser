use std::{collections::HashMap, rc::Rc};

use crate::{
    css::{CssColor, CssDimension, CssDisplay, CssProperty},
    html::{ElementContents, ElementContentsId, ElementHtmlNode, HtmlDocument},
    image::Bitmap,
    net::Url,
    render::{
        Color, NativeStyleNode, StyleNode, StyleNodeChild, StyleNodeDisplay, StyleNodeMargin,
        TextStyleNode,
    },
};

use super::AsyncRenderContext;

pub fn html_css_to_styles(
    html_document: &HtmlDocument,
    styles: &HashMap<ElementContentsId, Vec<Rc<CssProperty>>>,
    async_render_context: &mut AsyncRenderContext,
) -> StyleNode {
    let mut root = StyleNode::new_default(StyleNodeDisplay::Block);
    let inherited_styles = InheritedStyles::default();
    let html_style_node = render(
        styles,
        &inherited_styles,
        &html_document.html,
        async_render_context,
    );
    root.background_color = html_style_node.background_color;
    // TODO: force size to fill window. Requires display: absolute support
    root.child = StyleNodeChild::Nodes(vec![html_style_node]);
    root
}

/// These are styles that may be passed down from node to child
/// without actually having to be attributed to nodes themselves.
#[derive(Default, Clone, Debug)]
struct InheritedStyles {
    text_color: Color,
}

fn render(
    styles: &HashMap<ElementContentsId, Vec<Rc<CssProperty>>>,
    inherited_styles: &InheritedStyles,
    element: &ElementContents,
    async_render_context: &mut AsyncRenderContext,
) -> StyleNode {
    let mut cur_inherited_styles = inherited_styles.clone();
    let mut style_node = if let Some(props) = styles.get(&(*element).get_id()) {
        let display = match get_style_prop(
            props,
            "display",
            CssDisplay::from_raw_value,
            CssDisplay::Inline,
        ) {
            CssDisplay::Block => StyleNodeDisplay::Block,
            CssDisplay::Inline => StyleNodeDisplay::Inline,
            CssDisplay::None => StyleNodeDisplay::None,
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

        let margin_getter = |direction: &str| match get_style_prop_overrides(
            props,
            &[direction, "margin"],
            CssDimension::from_raw_value,
            CssDimension::Px(0.0),
        ) {
            CssDimension::Px(px) => px,
        };
        style_node.margin = StyleNodeMargin {
            left: margin_getter("margin-left"),
            top: margin_getter("margin-top"),
            right: margin_getter("margin-right"),
            bottom: margin_getter("margin-bottom"),
        };

        if let Some(text_color) = maybe_get_style_prop(props, "color", CssColor::from_raw_value) {
            cur_inherited_styles.text_color = match text_color {
                CssColor::Rgba(r, g, b, a) => Color { r, g, b, a },
            };
        }

        style_node
    } else {
        println!("No styles {:?}", (*element));
        StyleNode::new_default(StyleNodeDisplay::Inline)
    };

    style_node.child = match element {
        ElementContents::Element(element_node) => {
            if element_node.tag_name == "img" {
                on_img_node(element_node, async_render_context)
            } else {
                StyleNodeChild::Nodes(
                    element_node
                        .children
                        .iter()
                        .map(|child| {
                            render(styles, &cur_inherited_styles, child, async_render_context)
                        })
                        .collect(),
                )
            }
        }
        ElementContents::Text(text_node) => StyleNodeChild::Text(TextStyleNode {
            text: text_node.text.clone(),
            text_color: cur_inherited_styles.text_color,
            font_size: 12_f32, // TODO: Parse from styles
        }),
    };

    style_node
}

fn find_first_prop(props: &[Rc<CssProperty>], key: &[&str]) -> Option<String> {
    props
        .iter()
        // Last property takes precedence
        .rev()
        .find(|prop| key.contains(&prop.key.as_str()))
        .map(|prop| prop.value.clone())
}

fn maybe_get_style_prop<T, F: Fn(&str) -> Option<T>>(
    props: &[Rc<CssProperty>],
    property_names: &str,
    from_raw_value: F,
) -> Option<T> {
    maybe_get_style_prop_overrides(props, &[property_names], from_raw_value)
}

fn maybe_get_style_prop_overrides<T, F: Fn(&str) -> Option<T>>(
    props: &[Rc<CssProperty>],
    property_names: &[&str],
    from_raw_value: F,
) -> Option<T> {
    find_first_prop(props, property_names)
        .iter()
        .flat_map(|property_value| from_raw_value(property_value))
        .last()
}

fn get_style_prop_overrides<T, F: Fn(&str) -> Option<T>>(
    props: &[Rc<CssProperty>],
    property_names: &[&str],
    from_raw_value: F,
    default_value: T,
) -> T {
    maybe_get_style_prop_overrides(props, property_names, from_raw_value).unwrap_or(default_value)
}

fn get_style_prop<T, F: Fn(&str) -> Option<T>>(
    props: &[Rc<CssProperty>],
    property_name: &str,
    from_raw_value: F,
    default_value: T,
) -> T {
    get_style_prop_overrides(props, &[property_name], from_raw_value, default_value)
}

fn on_img_node(
    element: &ElementHtmlNode,
    async_render_context: &mut AsyncRenderContext,
) -> StyleNodeChild {
    let style_node = (|| -> Option<StyleNodeChild> {
        let src = element.get_attribute("src")?;
        let url = Url::parse(src)?;
        if !url.path.ends_with(".bmp") {
            // Only bitmaps are supported for now
            return None;
        }
        let http_result = async_render_context.get_resource(&url)?;
        let response = http_result.as_ref().as_ref().ok()?;
        if response.status.contains_success_content() {
            let bitmap = Bitmap::new(&response.body).ok()?;
            return Some(StyleNodeChild::Native(NativeStyleNode {
                width: bitmap.width,
                height: bitmap.height,
                pixels: bitmap.pixels,
            }));
        }
        None
    })();

    if let Some(node) = style_node {
        node
    } else {
        StyleNodeChild::Nodes(vec![])
    }
}
