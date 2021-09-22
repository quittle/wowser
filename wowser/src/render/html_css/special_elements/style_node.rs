use crate::{
    css,
    html::{ElementContents, ElementHtmlNode, TextHtmlNode},
    render::{AsyncRenderContext, StyleNodeChild},
};

pub fn on_style_node(
    element: &ElementHtmlNode,
    async_render_context: &mut AsyncRenderContext,
) -> StyleNodeChild {
    (|| -> Option<StyleNodeChild> {
        let type_attribute = element.get_attribute("type").unwrap_or("text/css");

        if type_attribute != "text/css" {
            return None;
        }

        if element.children.len() != 1 {
            return None;
        }

        if let ElementContents::Text(TextHtmlNode { text, .. }) = &element.children[0] {
            let css_document = css::parse_css(text).ok()?;
            async_render_context.css_documents.insert(
                element.get_id().to_string(),
                (element.document_offset, css_document),
            );
        }
        None
    })();

    StyleNodeChild::Nodes(vec![])
}
