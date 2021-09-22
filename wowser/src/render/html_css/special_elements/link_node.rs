use std::str::from_utf8;

use crate::{
    css,
    html::ElementHtmlNode,
    net::Url,
    render::{AsyncRenderContext, StyleNodeChild},
};

pub fn on_link_node(
    element: &ElementHtmlNode,
    async_render_context: &mut AsyncRenderContext,
) -> StyleNodeChild {
    (|| -> Option<StyleNodeChild> {
        let rel = element.get_attribute("rel")?;
        if rel != "stylesheet" {
            return None;
        }
        let href = element.get_attribute("href")?;
        let url = Url::parse(href)?;
        let http_result = async_render_context.get_resource(&url)?;
        let response = http_result.as_ref().as_ref().ok()?;
        if response.status.contains_success_content()
            && !async_render_context.css_documents.contains_key(href)
        {
            let body_string = from_utf8(&response.body).ok()?;
            let css_document = css::parse_css(body_string).ok()?;
            async_render_context
                .css_documents
                .insert(href.to_string(), (element.document_offset, css_document));
        }
        None
    })();

    StyleNodeChild::Nodes(vec![])
}
