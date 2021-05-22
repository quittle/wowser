use super::html_rule::HtmlRule;
use crate::parse::*;
use std::collections::HashMap;

#[derive(Debug)]
pub enum HtmlNode<'a> {
    Document {
        doctype: Box<HtmlNode<'a>>,
        document: Box<HtmlNode<'a>>,
    },
    Doctype(Vec<&'a str>),
    Element {
        // The tag name
        tag_name: &'a str,

        /// HTML Element attributes
        attributes: HashMap<&'a str, &'a str>,

        /// The children of a node, be it more Elements or Text
        remaining_children: Vec<HtmlNode<'a>>,
    },
    // Represents a text node
    Text(&'a str),
}

pub fn stringify_node(node: &HtmlNode) -> String {
    let mut ret = String::new();
    match node {
        HtmlNode::Doctype(contents) => {
            let contents =
                contents.iter().map(|s| format!(r#""{}""#, s)).collect::<Vec<String>>().join(" ");
            ret.push_str(format!("<!DOCTYPE {}>", contents).as_str());
        }
        HtmlNode::Text(text) => {
            ret.push_str(text);
        }
        HtmlNode::Document { doctype, document } => {
            ret.push_str(stringify_node(doctype).as_str());
            if let HtmlNode::Element { remaining_children, .. } = &**document {
                for child in remaining_children {
                    ret.push_str(stringify_node(&*child).as_str());
                }
            }
        }
        HtmlNode::Element { tag_name, attributes, remaining_children } => {
            let mut attributes =
                attributes.iter().map(|(k, v)| format!("{}=\"{}\"", k, v)).collect::<Vec<String>>();
            attributes.sort();
            let attributes = attributes.join(" ");
            let remaining_children = remaining_children
                .iter()
                .map(|child| stringify_node(child))
                .collect::<Vec<String>>()
                .join("");

            ret.push('<');
            ret.push_str(tag_name);

            if !attributes.is_empty() {
                ret.push(' ');
                ret.push_str(attributes.as_str());
            }

            if remaining_children.is_empty() {
                ret.push_str(" />");
            } else {
                ret.push('>');
                ret.push_str(remaining_children.as_str());
                ret.push_str("</");
                ret.push_str(tag_name);
                ret.push('>');
            }
        }
    }

    ret
}

pub struct HtmlInterpreter {}

impl<'a> Interpreter<'a> for HtmlInterpreter {
    type RuleType = HtmlRule;
    type Result = HtmlNode<'a>;

    fn on_node(&self, ast: &ASTNode<'a, HtmlRule>) -> Option<HtmlNode<'a>> {
        let ASTNode { rule, token, children } = ast;

        match **rule {
            HtmlRule::Document => {
                let mut doctype = HtmlNode::Doctype(vec![]);
                let mut contents = HtmlNode::Element {
                    tag_name: "",
                    attributes: HashMap::new(),
                    remaining_children: vec![],
                };
                for child in children {
                    let rule = &*child.rule;
                    match rule {
                        HtmlRule::Doctype => doctype = self.on_node(child)?,
                        HtmlRule::TagContents => contents = self.on_node(child)?,
                        HtmlRule::Terminator => {}
                        _ => panic!("Invalid child rule type for Document: {}", rule),
                    }
                }
                Some(HtmlNode::Document {
                    doctype: Box::new(doctype),
                    document: Box::new(contents),
                })
            }
            HtmlRule::Doctype => {
                assert_eq!(3, children.len());

                self.on_node(children.get(1)?)
            }
            HtmlRule::DoctypeContents => {
                let mut contents = vec![];
                for child in children {
                    if let HtmlNode::Doctype(mut child_content) = self.on_node(child)? {
                        contents.append(&mut child_content);
                    } else {
                        panic!("Invalid child type of DoctypeContents");
                    }
                }
                Some(HtmlNode::Doctype(contents))
            }
            HtmlRule::DoctypeContentsString => Some(HtmlNode::Doctype(vec![(*token)?.1])),
            HtmlRule::TagContents => {
                assert!(children.len() == 1 || children.len() == 2);
                let mut child_nodes = vec![];
                for child in children {
                    match &*child.rule {
                        HtmlRule::Text => {
                            child_nodes.push(self.on_node(child)?);
                        }
                        HtmlRule::TagsAndText => {
                            if let HtmlNode::Element { remaining_children, .. } =
                                self.on_node(child)?
                            {
                                for child in remaining_children {
                                    child_nodes.push(child);
                                }
                            } else {
                                panic!("Invalid child fo TagContents");
                            }
                        }
                        _ => panic!("Invalid child of TagContents"),
                    }
                }
                Some(HtmlNode::Element {
                    tag_name: "",
                    attributes: HashMap::new(),
                    remaining_children: child_nodes,
                })
            }
            HtmlRule::Text => Some(HtmlNode::Text((*token)?.1)),
            HtmlRule::TagsAndText => {
                let mut child_nodes = vec![];
                for child in children {
                    if let HtmlNode::Element { mut remaining_children, .. } = self.on_node(child)? {
                        child_nodes.append(&mut remaining_children);
                    }
                }
                Some(HtmlNode::Element {
                    tag_name: "",
                    attributes: HashMap::new(),
                    remaining_children: child_nodes,
                })
            }
            HtmlRule::TagAndText => {
                assert!(children.len() == 1 || children.len() == 2);
                let mut child_nodes = vec![];
                for child in children {
                    child_nodes.push(self.on_node(child)?);
                }
                Some(HtmlNode::Element {
                    tag_name: "",
                    attributes: HashMap::new(),
                    remaining_children: child_nodes,
                })
            }
            HtmlRule::Tag => {
                assert_eq!(1, children.len());

                self.on_node(children.get(0)?)
            }
            HtmlRule::SelfClosingTag => {
                assert_eq!(2, children.len());

                self.on_node(children.get(0)?)
                // Ignore the closing tag
            }
            HtmlRule::NonSelfClosingTag => {
                assert_eq!(3, children.len());
                if let HtmlNode::Element { tag_name, attributes, .. } =
                    self.on_node(children.get(0)?)?
                {
                    if let HtmlNode::Element { remaining_children, .. } =
                        self.on_node(children.get(1)?)?
                    {
                        // Ignore the closing tag as a child since it contains nothing of interest
                        return Some(HtmlNode::Element {
                            tag_name,
                            attributes,
                            remaining_children,
                        });
                    }
                }

                panic!("Invalid children of NonSelfClosingTag");
            }
            HtmlRule::OpeningTag => {
                assert_eq!(2, children.len());
                self.on_node(children.get(0)?)
            }
            HtmlRule::OpeningTagPrelude => {
                assert_eq!(2, children.len());
                let tag_name = children.get(0)?.token?.1;
                if let HtmlNode::Element { attributes, .. } = self.on_node(children.get(1)?)? {
                    Some(HtmlNode::Element { tag_name, attributes, remaining_children: vec![] })
                } else {
                    panic!("Invalid child of OpeningTagPrelude");
                }
            }
            HtmlRule::OpeningTagAttributes => {
                let mut ret_attributes = HashMap::new();
                for child in children {
                    if let HtmlNode::Element { attributes, .. } = self.on_node(child)? {
                        ret_attributes.extend(attributes);
                    } else {
                        panic!("Invalid child of OpeningTagAttributes");
                    }
                }
                Some(HtmlNode::Element {
                    tag_name: "",
                    attributes: ret_attributes,
                    remaining_children: vec![],
                })
            }
            HtmlRule::TagAttribute => {
                assert!(!children.is_empty());
                assert!(children.len() < 4);

                let name = children.get(0)?.token?.1;
                let value = if children.len() == 3 { children.get(2)?.token?.1 } else { "" };

                let mut attributes = HashMap::new();
                attributes.insert(name, value);
                Some(HtmlNode::Element { tag_name: "", attributes, remaining_children: vec![] })
            }
            HtmlRule::DoctypeStart => None,   // Constant value
            HtmlRule::OpeningTagName => None, // Handled by OpeningTagPrelude
            HtmlRule::AttributeName | HtmlRule::AttributeEquals | HtmlRule::AttributeValue => None, // Handled by TagAttribute
            HtmlRule::ClosingTag
            | HtmlRule::ClosingTagStart
            | HtmlRule::TagEnd
            | HtmlRule::SelfClosingTagEnding => None, // Not interesting nodes
            HtmlRule::Terminator => None, // Final Node
        }
    }
}
