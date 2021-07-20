use std::fmt::{Display, Write};

#[derive(Debug)]
pub struct HtmlDocument {
    pub doctype: DoctypeHtmlNode,
    pub html: ElementContents,
}

impl HtmlDocument {
    pub fn from(document_node: DocumentHtmlNode) -> HtmlDocument {
        let first_html_node = document_node.contents.iter()
        .find_map(|child|
            child.find_first(|element|
                matches!(element, ElementContents::Element(ElementHtmlNode{tag_name, ..}) if tag_name == "html")));
        let html = if let Some(ElementContents::Element(element)) = first_html_node {
            ElementHtmlNode {
                tag_name: "html".into(),
                attributes: element.attributes.clone(),
                children: document_node.contents,
            }
        } else {
            ElementHtmlNode {
                tag_name: "html".into(),
                children: document_node.contents,
                ..Default::default()
            }
        };

        HtmlDocument {
            doctype: document_node.doctype,
            html: ElementContents::Element(html),
        }
    }
}

impl Display for HtmlDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.doctype, self.html.to_string())
    }
}

/// Represents an entires HTML document
#[derive(Debug, Default)]
pub struct DocumentHtmlNode {
    pub doctype: DoctypeHtmlNode,
    pub contents: Vec<ElementContents>,
}
impl Display for DocumentHtmlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.doctype,
            self.contents
                .iter()
                .map(|child| child.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

/// Represents a text node
#[derive(Debug, Default, PartialEq)]
pub struct TextHtmlNode {
    pub text: String,
}

impl Display for TextHtmlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.text)
    }
}

/// Represents the units that can make up the contents of an element. There can be text interspersed with other elements
#[derive(Debug, PartialEq)]
pub enum ElementContents {
    Text(TextHtmlNode),
    Element(ElementHtmlNode),
}

impl ElementContents {
    pub fn find_first<P>(&self, mut predicate: P) -> Option<&ElementContents>
    where
        P: FnMut(&ElementContents) -> bool,
    {
        self._find_first(&mut predicate)
    }

    fn _find_first<P>(&self, predicate: &mut P) -> Option<&ElementContents>
    where
        P: FnMut(&ElementContents) -> bool,
    {
        if predicate(self) {
            Some(self)
        } else if let Self::Element(element_node) = self {
            element_node
                .children
                .iter()
                .find_map(|child| child._find_first(predicate))
        } else {
            None
        }
    }
}

impl Display for ElementContents {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Text(text_node) => f.write_str(&text_node.to_string()),
            Self::Element(element_node) => f.write_str(&element_node.to_string()),
        }
    }
}

/// Represents a doctype node
#[derive(Debug, Default)]
pub struct DoctypeHtmlNode {
    pub document_type_definition: Vec<String>,
}

impl Display for DoctypeHtmlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
#[derive(Debug, Default, PartialEq)]
pub struct ElementHtmlNode {
    pub tag_name: String,
    pub attributes: Vec<TagAttributeHtmlNode>,
    pub children: Vec<ElementContents>,
}

impl ElementHtmlNode {
    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        let lowercase_name = name.to_ascii_lowercase();
        self.attributes
            .iter()
            .find(|attribute| attribute.name == lowercase_name)
            .map(|attribute| attribute.value.as_ref())
            .flatten()
            .map(|value| value.as_str())
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        let lowercase_name = name.to_ascii_lowercase();
        self.attributes
            .iter()
            .any(|attribute| attribute.name == lowercase_name)
    }
}

impl Display for ElementHtmlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TagAttributeHtmlNode {
    pub name: String,
    pub value: Option<String>,
}

impl Display for TagAttributeHtmlNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.name)?;
        if let Some(value) = &self.value {
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
    fn element_contents_find() {
        let root = ElementContents::Element(ElementHtmlNode {
            tag_name: "a".into(),
            children: vec![
                ElementContents::Element(ElementHtmlNode {
                    tag_name: "aa".into(),
                    children: vec![
                        ElementContents::Text(TextHtmlNode {
                            text: "aa-text".into(),
                        }),
                        ElementContents::Element(ElementHtmlNode {
                            tag_name: "aaa".into(),
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
                ElementContents::Text(TextHtmlNode {
                    text: "a-text".into(),
                }),
                ElementContents::Element(ElementHtmlNode {
                    tag_name: "ab".into(),
                    ..Default::default()
                }),
            ],
            ..Default::default()
        });

        // Reverse the order because Vec.pop() grabs from the end. This list should be
        // written in the expected order of access
        let mut order = vec!["a", "aa", "aa-text", "aaa", "a-text", "ab"];
        order.reverse();

        let ret = root.find_first(|node| {
            match node {
                ElementContents::Element(element) => {
                    assert_eq!(order.pop().unwrap(), element.tag_name)
                }
                ElementContents::Text(text) => assert_eq!(order.pop().unwrap(), text.text),
            }
            false
        });
        assert_eq!(None, ret);

        let ret = root.find_first(|node|
            matches!(node, ElementContents::Element(ElementHtmlNode { tag_name, .. }) if tag_name == "aaa"));

        assert!(matches!(
            ret,
            Some(ElementContents::Element(ElementHtmlNode { tag_name, .. })) if tag_name == "aaa"
        ));

        let ret = root.find_first(|_node| true);

        assert!(matches!(
            ret,
            Some(ElementContents::Element(ElementHtmlNode { tag_name, .. })) if tag_name == "a"
        ));
    }

    #[test]
    fn document_html_node() {
        let node = DocumentHtmlNode {
            doctype: DoctypeHtmlNode {
                document_type_definition: vec![],
            },
            contents: vec![ElementContents::Text(TextHtmlNode {
                text: "text".into(),
            })],
        };
        assert_eq!("<!DOCTYPE>text", node.to_string());
    }

    #[test]
    fn doctype_html_node() {
        let mut node = DoctypeHtmlNode {
            document_type_definition: vec![],
        };
        assert_eq!("<!DOCTYPE>", node.to_string());
        node.document_type_definition.push("html".into());
        assert_eq!("<!DOCTYPE \"html\">", node.to_string());
        node.document_type_definition.push("contains spaces".into());
        assert_eq!("<!DOCTYPE \"html\" \"contains spaces\">", node.to_string());
    }

    #[test]
    fn element_html_node() {
        let mut node = ElementHtmlNode {
            tag_name: "tag".into(),
            attributes: vec![],
            children: vec![],
        };
        assert_eq!("<tag />", node.to_string());
        node.attributes.push(TagAttributeHtmlNode {
            name: "attr1".into(),
            value: None,
        });
        assert_eq!("<tag attr1 />", node.to_string());
        node.attributes[0].value = Some("value".into());
        assert_eq!("<tag attr1=\"value\" />", node.to_string());
        node.children.push(ElementContents::Text(TextHtmlNode {
            text: "text content".into(),
        }));
        assert_eq!("<tag attr1=\"value\">text content</tag>", node.to_string());
        node.attributes.clear();
        assert_eq!("<tag>text content</tag>", node.to_string());
        node.children
            .push(ElementContents::Element(ElementHtmlNode {
                tag_name: "nested".into(),
                attributes: vec![],
                children: vec![],
            }));
        assert_eq!("<tag>text content<nested /></tag>", node.to_string());
        node.children.insert(
            0,
            ElementContents::Element(ElementHtmlNode {
                tag_name: "first".into(),
                attributes: vec![],
                children: vec![],
            }),
        );
        assert_eq!(
            "<tag><first />text content<nested /></tag>",
            node.to_string()
        );
    }

    #[test]
    fn tag_attribute_html_node() {
        let mut node = TagAttributeHtmlNode {
            name: "name".into(),
            value: None,
        };
        assert_eq!("name", node.to_string());
        node.value = Some("value".into());
        assert_eq!("name=\"value\"", node.to_string());
    }

    #[test]
    fn text_html_node() {
        let node = TextHtmlNode {
            text: "text in node".into(),
        };
        assert_eq!("text in node", node.to_string());
    }

    #[test]
    fn element_contents() {
        let node = ElementContents::Text(TextHtmlNode {
            text: "text".into(),
        });
        assert_eq!("text", node.to_string());

        let node = ElementContents::Element(ElementHtmlNode {
            tag_name: "tag".into(),
            attributes: vec![],
            children: vec![],
        });
        assert_eq!("<tag />", node.to_string());
    }

    #[test]
    fn get_has_attribute() {
        let element = ElementHtmlNode {
            tag_name: "tag".into(),
            attributes: vec![
                TagAttributeHtmlNode {
                    name: "a".into(),
                    value: Some("a".into()),
                },
                TagAttributeHtmlNode {
                    name: "b".into(),
                    value: Some("b".into()),
                },
                TagAttributeHtmlNode {
                    name: "B".into(),
                    value: Some("B".into()),
                },
                TagAttributeHtmlNode {
                    name: "c".into(),
                    value: None,
                },
            ],
            children: vec![],
        };
        assert_eq!(Some("a"), element.get_attribute("a"));
        assert_eq!(true, element.has_attribute("a"));

        assert_eq!(Some("b"), element.get_attribute("b"));
        assert_eq!(Some("b"), element.get_attribute("B"));
        assert_eq!(true, element.has_attribute("b"));

        assert_eq!(None, element.get_attribute("c"));
        assert_eq!(true, element.has_attribute("c"));

        assert_eq!(None, element.get_attribute("d"));
        assert_eq!(false, element.has_attribute("d"));
        // element.get_attribute(name);
    }
}
