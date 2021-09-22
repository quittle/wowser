use crate::{
    html::ElementHtmlNode,
    image::Bitmap,
    net::Url,
    render::{AsyncRenderContext, NativeStyleNode, StyleNodeChild},
};

pub fn on_img_node(
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
