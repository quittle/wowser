use crate::css::CssDocument;
use crate::html::DocumentHtmlNode;
use crate::html::ElementContents;

pub fn style_html(html_document: DocumentHtmlNode, css_document: CssDocument) {
    for element in &html_document.contents {
        style_elements(element, &css_document, vec![]);
    }
}

fn style_elements(
    element: &ElementContents,
    css_document: &CssDocument,
    parents: Vec<&ElementContents>,
) {
    // css_document.blocks.iter().map(|block| block.selectors[0].selectors);
}

fn does_element_match(
    element: &ElementContents,
    parents: Vec<&ElementContents>,
    matchers: Vec<String>,
) -> bool {
    true
}
