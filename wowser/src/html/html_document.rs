use std::fmt::{Display, Write};

/// Represents an entires HTML document
#[derive(Debug, Default)]
pub struct DocumentHtmlNode<'a> {
    pub doctype: DoctypeHtmlNode<'a>,
    pub contents: Vec<ElementContents<'a>>,
}
impl Display for DocumentHtmlNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.doctype,
            self.contents.iter().map(|child| child.to_string()).collect::<Vec<String>>().join("")
        )
    }
}

/// Represents a text node
#[derive(Debug, Default)]
pub struct TextHtmlNode<'a> {
    pub text: &'a str,
}

impl Display for TextHtmlNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.text)
    }
}

/// Represents the units that can make up the contents of an element. There can be text interspersed with other elements
#[derive(Debug)]
pub enum ElementContents<'a> {
    Text(TextHtmlNode<'a>),
    Element(ElementHtmlNode<'a>),
}

impl Display for ElementContents<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(text_node) => f.write_str(&text_node.to_string()),
            Self::Element(element_node) => f.write_str(&element_node.to_string()),
        }
    }
}

/// Represents a doctype node
#[derive(Debug, Default)]
pub struct DoctypeHtmlNode<'a> {
    pub document_type_definition: Vec<&'a str>,
}

impl Display for DoctypeHtmlNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let contents = self
            .document_type_definition
            .iter()
            .map(|s| format!(r#""{}""#, s))
            .collect::<Vec<String>>()
            .join(" ");
        f.write_fmt(format_args!("<!DOCTYPE {}>", contents))
    }
}

/// Represents an element node
#[derive(Debug, Default)]
pub struct ElementHtmlNode<'a> {
    pub tag_name: &'a str,
    pub attributes: Vec<TagAttributeHtmlNode<'a>>,
    pub children: Vec<ElementContents<'a>>,
}

impl Display for ElementHtmlNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("<{}", self.tag_name))?;
        if !self.attributes.is_empty() {
            f.write_char(' ')?;
            let attributes = self
                .attributes
                .iter()
                .map(|attribute| attribute.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            f.write_str(&attributes)?;
        }
        if self.children.is_empty() {
            f.write_str(" />")?;
        } else {
            f.write_char('>')?;
            let children = self
                .children
                .iter()
                .map(|child| child.to_string())
                .collect::<Vec<String>>()
                .join("");
            f.write_str(&children)?;
            f.write_fmt(format_args!("</{}>", self.tag_name))?;
        }
        Ok(())
    }
}

/// Represents a text node
#[derive(Debug, Default)]
pub struct TagAttributeHtmlNode<'a> {
    pub name: &'a str,
    pub value: Option<&'a str>,
}

impl Display for TagAttributeHtmlNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)?;
        if let Some(value) = self.value {
            let escaped_value = value.replace('"', "\\\"");
            f.write_fmt(format_args!("=\"{}\"", escaped_value))?;
        }
        Ok(())
    }
}
