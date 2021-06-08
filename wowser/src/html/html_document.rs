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
            .map(|s| format!(r#" "{}""#, s))
            .collect::<Vec<String>>()
            .join("");
        f.write_fmt(format_args!("<!DOCTYPE{}>", contents))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_html_node() {
        let node = DocumentHtmlNode {
            doctype: DoctypeHtmlNode { document_type_definition: vec![] },
            contents: vec![ElementContents::Text(TextHtmlNode { text: "text" })],
        };
        assert_eq!("<!DOCTYPE>text", node.to_string());
    }

    #[test]
    fn doctype_html_node() {
        let mut node = DoctypeHtmlNode { document_type_definition: vec![] };
        assert_eq!("<!DOCTYPE>", node.to_string());
        node.document_type_definition.push("html");
        assert_eq!("<!DOCTYPE \"html\">", node.to_string());
        node.document_type_definition.push("contains spaces");
        assert_eq!("<!DOCTYPE \"html\" \"contains spaces\">", node.to_string());
    }

    #[test]
    fn element_html_node() {
        let mut node = ElementHtmlNode { tag_name: "tag", attributes: vec![], children: vec![] };
        assert_eq!("<tag />", node.to_string());
        node.attributes.push(TagAttributeHtmlNode { name: "attr1", value: None });
        assert_eq!("<tag attr1 />", node.to_string());
        node.attributes[0].value = Some("value");
        assert_eq!("<tag attr1=\"value\" />", node.to_string());
        node.children.push(ElementContents::Text(TextHtmlNode { text: "text content" }));
        assert_eq!("<tag attr1=\"value\">text content</tag>", node.to_string());
        node.attributes.clear();
        assert_eq!("<tag>text content</tag>", node.to_string());
        node.children.push(ElementContents::Element(ElementHtmlNode {
            tag_name: "nested",
            attributes: vec![],
            children: vec![],
        }));
        assert_eq!("<tag>text content<nested /></tag>", node.to_string());
        node.children.insert(
            0,
            ElementContents::Element(ElementHtmlNode {
                tag_name: "first",
                attributes: vec![],
                children: vec![],
            }),
        );
        assert_eq!("<tag><first />text content<nested /></tag>", node.to_string());
    }

    #[test]
    fn tag_attribute_html_node() {
        let mut node = TagAttributeHtmlNode { name: "name", value: None };
        assert_eq!("name", node.to_string());
        node.value = Some("value");
        assert_eq!("name=\"value\"", node.to_string());
    }

    #[test]
    fn text_html_node() {
        let node = TextHtmlNode { text: "text in node" };
        assert_eq!("text in node", node.to_string());
    }

    #[test]
    fn element_contents() {
        let node = ElementContents::Text(TextHtmlNode { text: "text" });
        assert_eq!("text", node.to_string());

        let node = ElementContents::Element(ElementHtmlNode {
            tag_name: "tag",
            attributes: vec![],
            children: vec![],
        });
        assert_eq!("<tag />", node.to_string());
    }
}
