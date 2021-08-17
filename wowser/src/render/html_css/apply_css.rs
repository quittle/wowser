use std::collections::HashMap;
use std::rc::Rc;

use crate::css::CssDocument;
use crate::css::CssProperty;
use crate::css::CssSelectorChain;
use crate::css::CssSelectorChainItem;
use crate::html::ElementContents;
use crate::html::ElementContentsId;
use crate::html::HtmlDocument;

/// Applies a CSS document to an HTML document, returning a mapping of the entries in
/// `html_document` to their rendered CSS properties. All nodes in html_document are
/// guaranteed to have an entry, even if it's just an empty vector. Property keys
/// in each vec may be repeated but appear in order that they appear in the document.
pub fn style_html(
    html_document: &HtmlDocument,
    css_document: &CssDocument,
) -> HashMap<ElementContentsId, Vec<Rc<CssProperty>>> {
    recurse_style_html(&html_document.html, css_document, &[])
}

fn recurse_style_html(
    element: &ElementContents,
    css_document: &CssDocument,
    parents: &[&ElementContents],
) -> HashMap<ElementContentsId, Vec<Rc<CssProperty>>> {
    let cur_styles = get_applicable_styles(element, css_document, parents);
    let mut child_styles = if let ElementContents::Element(element_node) = element {
        let mut new_parents = parents.to_vec();
        new_parents.push(element);
        element_node
            .children
            .iter()
            .flat_map(|child| recurse_style_html(child, css_document, &new_parents))
            .collect()
    } else {
        HashMap::new()
    };
    child_styles.insert((*element).get_id(), cur_styles);
    child_styles
}

fn get_applicable_styles<'a>(
    element: &ElementContents,
    css_document: &'a CssDocument,
    parents: &[&ElementContents],
) -> Vec<Rc<CssProperty>> {
    css_document
        .blocks
        .iter()
        .filter(|block| {
            block
                .selectors
                .iter()
                .any(|selector_chain| do_elements_match(element, parents, selector_chain))
        })
        .flat_map(|block| block.properties.clone())
        .collect()
}

fn do_elements_match(
    element: &ElementContents,
    parents: &[&ElementContents],
    selector_chain: &CssSelectorChain,
) -> bool {
    let mut cur_selector = selector_chain;
    // Tracks if we matched all the way down the stack already.
    // If no next selector, then we did our best to match and "succeeded",
    // otherwise we need to match within the loop.
    for node in parents {
        if let Some(next_selector) = cur_selector.next.as_ref() {
            if does_element_match(node, cur_selector) {
                cur_selector = next_selector
            }
        } else {
            // If no next selector then we reached the end and must match on the input element with this final one.
            // Break early as an optimization.
            break;
        }
    }
    cur_selector.next.is_none() && does_element_match(element, cur_selector)
}

fn does_element_match(element_contents: &ElementContents, selector: &CssSelectorChain) -> bool {
    if let ElementContents::Element(element) = element_contents {
        match &selector.item {
            CssSelectorChainItem::Tag(tag_name) => element.tag_name == *tag_name,
            CssSelectorChainItem::Class(class) => element.attributes.iter().any(|attribute| {
                attribute.name == "class"
                    && attribute
                        .value
                        .as_ref()
                        .unwrap_or(&"".into())
                        .split(' ')
                        .any(|class_name| class_name == class)
            }),
            CssSelectorChainItem::Id(id) => element
                .attributes
                .iter()
                .any(|attribute| attribute.name == "id" && attribute.value == Some(id.into())),
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::{
        css::parse_css,
        html::{parse_html, ElementHtmlNode},
    };

    fn as_element(element: &ElementContents) -> &ElementHtmlNode {
        match element {
            ElementContents::Element(element_node) => element_node,
            _ => panic!("Unexpected element type"),
        }
    }

    fn get_style<'css>(
        styles: &'css HashMap<ElementContentsId, Vec<Rc<CssProperty>>>,
        element: &ElementContents,
    ) -> &'css Vec<Rc<CssProperty>> {
        styles.get(&(*element).get_id()).unwrap()
    }

    fn get_element(html_document: &HtmlDocument, children: Vec<usize>) -> &ElementContents {
        let mut node = &html_document.html;
        for index in children {
            node = &as_element(node).children[index];
        }
        node
    }

    #[test]
    fn test_empty_config() {
        let css_document = parse_css("").unwrap();
        let html_document = parse_html("").unwrap();
        let styling = style_html(&html_document, &css_document);

        assert_eq!(
            get_style(&styling, get_element(&html_document, vec![0])),
            &Vec::<Rc<CssProperty>>::new()
        );
    }

    #[test]
    fn test_non_matching() {
        let css_document = parse_css("foo { color: red; }").unwrap();
        let html_document = parse_html(r#"<bar id="foo" class="foo"></bar>"#).unwrap();
        let styling = style_html(&html_document, &css_document);

        assert_eq!(
            get_style(&styling, get_element(&html_document, vec![0])),
            &Vec::<Rc<CssProperty>>::new()
        );
    }

    #[test]
    fn test_tag_match() {
        let css_document = parse_css("foo { color: red; }").unwrap();
        let html_document = parse_html(r#"<foo />"#).unwrap();
        let styling = style_html(&html_document, &css_document);

        assert_eq!(
            get_style(&styling, get_element(&html_document, vec![0])),
            &vec![Rc::new(CssProperty {
                key: "color".into(),
                value: "red".into()
            })]
        );
    }

    #[test]
    fn test_class_match() {
        let css_document = parse_css(".foo { color: red; }").unwrap();
        let html_document = parse_html(r#"<bar class="some foo bar" />"#).unwrap();
        let styling = style_html(&html_document, &css_document);

        assert_eq!(
            get_style(&styling, get_element(&html_document, vec![0])),
            &vec![Rc::new(CssProperty {
                key: "color".into(),
                value: "red".into()
            })]
        );
    }

    #[test]
    fn test_id_match() {
        let css_document = parse_css("#foo { color: red; }").unwrap();
        let html_document = parse_html(r#"<bar id="foo" />"#).unwrap();
        let styling = style_html(&html_document, &css_document);

        assert_eq!(
            get_style(&styling, get_element(&html_document, vec![0])),
            &vec![Rc::new(CssProperty {
                key: "color".into(),
                value: "red".into()
            })]
        );
    }

    #[test]
    fn test_multiple_selectors() {
        let css_document = parse_css("foo, bar { color: red; }").unwrap();
        let html_document = parse_html(r#"<bar /><foo />"#).unwrap();
        let styling = style_html(&html_document, &css_document);

        assert_eq!(
            get_style(&styling, get_element(&html_document, vec![0])),
            &vec![Rc::new(CssProperty {
                key: "color".into(),
                value: "red".into()
            })]
        );
        assert_eq!(
            get_style(&styling, get_element(&html_document, vec![1])),
            &vec![Rc::new(CssProperty {
                key: "color".into(),
                value: "red".into()
            })]
        );
    }

    #[test]
    fn test_override_properties() {
        let css_document = parse_css("foo { color: red; color: blue; }").unwrap();
        let html_document = parse_html(r#"<foo>value</foo>"#).unwrap();
        let styling = style_html(&html_document, &css_document);

        assert_eq!(
            get_style(&styling, get_element(&html_document, vec![0])),
            &vec![
                Rc::new(CssProperty {
                    key: "color".into(),
                    value: "red".into()
                }),
                Rc::new(CssProperty {
                    key: "color".into(),
                    value: "blue".into()
                })
            ]
        );

        let css_document = parse_css("foo { color: red; } foo { color: blue; }").unwrap();
        let html_document = parse_html(r#"<foo>value</foo>"#).unwrap();
        let styling = style_html(&html_document, &css_document);

        assert_eq!(
            get_style(&styling, get_element(&html_document, vec![0])),
            &vec![
                Rc::new(CssProperty {
                    key: "color".into(),
                    value: "red".into()
                }),
                Rc::new(CssProperty {
                    key: "color".into(),
                    value: "blue".into()
                })
            ]
        );
    }

    #[test]
    fn test_simple_config() {
        let css_document = parse_css("foo { color: red; } foo .baz { height: 1; }").unwrap();
        let html_document = parse_html(r#"<foo><bar class="baz">text</bar></foo>"#).unwrap();
        let styling = style_html(&html_document, &css_document);

        assert_eq!(
            get_style(&styling, get_element(&html_document, vec![0])),
            &vec![Rc::new(CssProperty {
                key: "color".into(),
                value: "red".into()
            })]
        );
        assert_eq!(
            get_style(&styling, get_element(&html_document, vec![1])),
            &Vec::<Rc<CssProperty>>::new()
        );
        assert_eq!(
            get_style(&styling, get_element(&html_document, vec![0, 0])),
            &vec![Rc::new(CssProperty {
                key: "height".into(),
                value: "1".into()
            })]
        );
        assert_eq!(
            get_style(&styling, get_element(&html_document, vec![0, 0, 0])),
            &Vec::<Rc<CssProperty>>::new()
        );
    }
}
