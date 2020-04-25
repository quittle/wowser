use super::DoctypeElement;

use regex::Regex;
use std::collections::HashMap;

#[allow(dead_code)]
struct GenericTag<'a> {
    prefix_text: &'a str,
    suffix_text: &'a str,
    children: Vec<GenericTag<'a>>,
    attributes: HashMap<&'a str, &'a str>,
}

// impl GenericTag {
//     pub fn from<'a>(html_document: &'a str) -> GenericTag<'a> {
//         lazy_static! {
//             static ref TAG_REGEX: Regex =
//                 Regex::new(r"^<(\w+)\s([^>]*)>(.*)</(?i)html(?-i)>$").unwrap();
//         }
        
//     }
// }

#[allow(dead_code)]
struct HtmlElement {
    head: HeadElement,
    body: BodyElement,
    attrs: HashMap<String, String>,
}

impl HtmlElement {
    #[allow(dead_code, unused_variables)]
    pub fn from(html_document: &str) -> HtmlElement {
        lazy_static! {
            static ref HTML_ELEMENT_REGEX: Regex =
                Regex::new(r"^<(?i)html(?-i)\s([^>]*)>(.*)</(?i)html(?-i)>$").unwrap();
        }

        // let trimmed = html_document.trim();
        // if let Some(html) = HTML_ELEMENT_REGEX.captures_iter(trimmed).next() {
        //     if let Some(html_attrs) = doctype.get(1) {
        //         DoctypeElement::Specified {
        //             full_type: doctype.as_str(),
        //         }
        //     } else {
        //         DoctypeElement::Unspecified
        //     }
        // } else {
        //     DoctypeElement::Unspecified
        // }

        HtmlElement {
            head: HeadElement {},
            body: BodyElement {},
            attrs: HashMap::new()
        }
    }
}

struct HeadElement {}

struct BodyElement {}

#[allow(dead_code)]
pub struct HtmlDocument<'a> {
    pub(super) doctype_element: DoctypeElement<'a>,
    html_root: HtmlElement,
}

#[allow(clippy::needless_lifetimes, dead_code)]
impl<'b> HtmlDocument<'b> {
    pub fn from<'c>(html_document: &'c str) -> HtmlDocument<'c> {
        lazy_static! {
            static ref DOCTYPE_REGEX: Regex = Regex::new(r"<!(?i)DOCTYPE(?-i)\s([^>]*)>").unwrap();
        }

        let trimmed = html_document.trim();
        let doctype_element = if trimmed.starts_with("<!") {
            if let Some(doctype) = DOCTYPE_REGEX.captures_iter(trimmed).next() {
                if let Some(doctype) = doctype.get(1) {
                    DoctypeElement::Specified {
                        full_type: doctype.as_str(),
                    }
                } else {
                    DoctypeElement::Unspecified
                }
            } else {
                DoctypeElement::Unspecified
            }
        } else {
            DoctypeElement::Unspecified
        };

        HtmlDocument {
            doctype_element,
            html_root: HtmlElement::from(html_document),
        }
    }
}
