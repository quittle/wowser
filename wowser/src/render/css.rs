use crate::css::CssDocument;
use crate::css::CssProperty;
use crate::css::CssSelectorChain;
use crate::css::CssSelectorChainItem;
use crate::html::DocumentHtmlNode;
use crate::html::ElementContents;

pub fn style_html<'html, 'css>(
    html_document: &'html DocumentHtmlNode,
    css_document: &'css CssDocument,
) -> Vec<(&'html ElementContents, Vec<&'css CssProperty>)> {
    html_document
        .contents
        .iter()
        .flat_map(|element| recurse_style_html(element, css_document, &[]))
        .collect()
}

fn recurse_style_html<'element, 'css>(
    element: &'element ElementContents,
    css_document: &'css CssDocument,
    parents: &[&ElementContents],
) -> Vec<(&'element ElementContents, Vec<&'css CssProperty>)> {
    let cur_styles = get_applicable_styles(element, &css_document, parents);
    let mut child_styles = if let ElementContents::Element(element_node) = element {
        let mut new_parents = parents.to_vec();
        new_parents.push(element);
        element_node
            .children
            .iter()
            .flat_map(|child| recurse_style_html(child, css_document, &new_parents))
            .collect()
    } else {
        vec![]
    };
    child_styles.push((element, cur_styles));
    child_styles
}

fn get_applicable_styles<'a>(
    element: &ElementContents,
    css_document: &'a CssDocument,
    parents: &[&ElementContents],
) -> Vec<&'a CssProperty> {
    css_document
        .blocks
        .iter()
        .filter(|block| {
            block
                .selectors
                .iter()
                .any(|selector_chain| do_elements_match(element, parents, selector_chain))
        })
        .flat_map(|block| &block.properties)
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
    let result = if let ElementContents::Element(element) = element_contents {
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
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        css::{CssDocument, CssInterpreter, CssRule, CssToken},
        html::{HtmlInterpreter, HtmlRule, HtmlToken},
        parse::{Interpreter, Lexer, Parser},
    };

    fn parse_css(document: &str) -> CssDocument {
        let lexer = Lexer::new(Box::new(CssToken::Document));
        let tokens = Box::new(lexer.parse(document).expect("Failed to lex"));
        let ast = Parser {}.parse(&tokens, &CssRule::Document).expect("Failed to parse");
        let css_document = CssInterpreter {}.interpret(&ast).expect("Failed to interpret");
        css_document
    }

    fn test_css_html(css_file: &str, html_file: &str) {
        let lexer = Lexer::new(Box::new(HtmlToken::Document));
        let tokens = lexer.parse(html_file).expect("Failed to lex");
        let ast = Parser {}.parse(&tokens, &HtmlRule::Document).expect("Failed to parse");
        let html_document = HtmlInterpreter {}.interpret(&ast).expect("Failed to interpret");

        let css_document = parse_css(css_file);

        let styling = style_html(&html_document, &css_document);
        println!("Styling {:?}", styling);
    }

    #[test]
    fn minimal() {
        test_css_html(
            "foo { color: red; } foo .baz { height: 1; }",
            "<foo><bar class=\"baz\">text</bar></foo>",
        );
    }
}
