// use html_document;
// mod html_document;

// use super::html_document;
// use self::html_document::HtmlDocument;
use super::{DoctypeElement, HtmlDocument};
// use regex::{Match, Regex};

#[allow(dead_code)]
pub struct Html<'a> {
    raw_html: Box<str>,
    html_document: HtmlDocument<'a>,
}

#[allow(dead_code, clippy::borrowed_box)]
impl<'a> Html<'a> {
    pub fn load(html_document: &'a Box<str>) -> Html<'a> {
        Html {
            raw_html: html_document.clone(),
            html_document: HtmlDocument::from(html_document),
        }
    }
    pub fn get_doctype(&self) -> &str {
        match self.html_document.doctype_element {
            DoctypeElement::Specified { full_type } => &full_type,
            DoctypeElement::Unspecified => "UNSET",
        }
    }
}
