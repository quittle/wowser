use std::rc::Rc;

use super::CssRule;
use crate::parse::*;

#[derive(Debug)]
pub enum CssNode<'a> {
    Document(Vec<CssNode<'a>>),
    Block {
        selectors: Vec<CssNode<'a>>,
        properties: Vec<CssNode<'a>>,
    },
    Selector(Vec<CssNode<'a>>),
    SelectorItem(&'a str),
    Property {
        key: &'a str,
        value: &'a str,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub struct CssDocument {
    pub blocks: Vec<CssBlock>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct CssBlock {
    pub selectors: Vec<CssSelectorChain>,
    pub properties: Vec<Rc<CssProperty>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct CssSelectorChain {
    pub item: CssSelectorChainItem,
    pub next: Option<Box<CssSelectorChain>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum CssSelectorChainItem {
    Tag(String),
    Class(String),
    Id(String),
}

#[derive(PartialEq, Debug, Clone)]
pub struct CssProperty {
    pub key: String,
    pub value: String,
}

impl CssProperty {
    pub fn new(key: &str, value: &str) -> CssProperty {
        CssProperty {
            key: key.into(),
            value: value.into(),
        }
    }

    pub fn new_rc(key: &str, value: &str) -> Rc<CssProperty> {
        Rc::new(Self::new(key, value))
    }
}

type CssASTNode<'a> = ASTNode<'a, CssRule>;

pub struct CssInterpreter {}

impl CssInterpreter {
    #[allow(dead_code)]
    fn on_blocks(&self, blocks: &CssASTNode) -> Vec<CssBlock> {
        let children = extract_interpreter_children(blocks, CssRule::Blocks);

        children.iter().map(|child| self.on_block(child)).collect()
    }

    fn on_block(&self, node: &CssASTNode) -> CssBlock {
        let children = extract_interpreter_n_children(node, CssRule::Block, 2);

        CssBlock {
            selectors: self.on_selector_list(&children[0]),
            properties: self.on_block_body(&children[1]),
        }
    }

    fn on_selector_list(&self, selector_list: &CssASTNode) -> Vec<CssSelectorChain> {
        let children = extract_interpreter_children(selector_list, CssRule::SelectorList);
        let children_len = children.len();
        match children_len {
            1 => vec![self.on_selector(&children[0])],
            3 => {
                let mut list = vec![self.on_selector(&children[0])];
                list.append(&mut self.on_selector_list(&children[2]));
                list
            }
            _ => panic!("Unsupported number of children for SelectorList {children_len}",),
        }
    }

    fn on_selector(&self, selector: &CssASTNode) -> CssSelectorChain {
        let children = extract_interpreter_children(selector, CssRule::Selector);
        assert!(!children.is_empty(), "Expected at least one child");

        let mut ret = CssSelectorChain {
            item: self.on_selector_item(&children[0]),
            next: None,
        };

        let mut cur_node = &mut ret;
        for child in children.iter().skip(1) {
            let next_node = CssSelectorChain {
                item: self.on_selector_item(child),
                next: None,
            };
            cur_node.next = Some(Box::new(next_node));
            let boxed_node = cur_node.next.as_mut().expect("Unexpected empty next node");
            cur_node = boxed_node.as_mut();
        }
        ret
    }

    fn on_selector_item(&self, selector: &CssASTNode) -> CssSelectorChainItem {
        let token = extract_interpreter_token(selector, CssRule::SelectorItem);

        if let Some(class) = token.strip_prefix('.') {
            CssSelectorChainItem::Class(class.to_string())
        } else if let Some(id) = token.strip_prefix('#') {
            CssSelectorChainItem::Id(id.to_string())
        } else {
            CssSelectorChainItem::Tag(token.to_string())
        }
    }

    fn on_block_body(&self, block_body: &CssASTNode) -> Vec<Rc<CssProperty>> {
        let children = extract_interpreter_n_children(block_body, CssRule::BlockBody, 3);
        self.on_property_list(&children[1])
    }

    fn on_property_list(&self, property_list: &CssASTNode) -> Vec<Rc<CssProperty>> {
        let children = extract_interpreter_children(property_list, CssRule::PropertyList);
        assert!(
            children.len() == 1 || children.len() == 2,
            "Unexpected number of children"
        );
        let mut properties = self.on_strict_property_list(&children[0]);
        if let Some(trailing_property) = children.get(1) {
            properties.push(Rc::new(self.on_trailing_property(trailing_property)));
        }

        properties
    }

    fn on_strict_property_list(&self, property_list: &CssASTNode) -> Vec<Rc<CssProperty>> {
        let children = extract_interpreter_children(property_list, CssRule::StrictPropertyList);
        children
            .iter()
            .map(|child| Rc::new(self.on_property(child)))
            .collect()
    }

    fn on_property(&self, property: &CssASTNode) -> CssProperty {
        let children = extract_interpreter_n_children(property, CssRule::Property, 4);

        CssProperty {
            key: self.on_property_key(&children[0]),
            value: self.on_property_value(&children[2]),
        }
    }

    fn on_trailing_property(&self, property: &CssASTNode) -> CssProperty {
        let children = extract_interpreter_n_children(property, CssRule::TrailingProperty, 3);

        CssProperty {
            key: self.on_property_key(&children[0]),
            value: self.on_property_value(&children[2]),
        }
    }

    fn on_property_key(&self, selector: &CssASTNode) -> String {
        let token = extract_interpreter_token(selector, CssRule::PropertyKey);

        token.trim().to_string()
    }

    fn on_property_value(&self, selector: &CssASTNode) -> String {
        let token = extract_interpreter_token(selector, CssRule::PropertyValue);
        token.trim().to_string()
    }

    fn on_top_level_entries(&self, node: &CssASTNode) -> Vec<CssBlock> {
        let children = extract_interpreter_children(node, CssRule::TopLevelEntries);
        if children.is_empty() {
            return vec![];
        }

        let first_child = &children[0];
        let block = match &first_child.rule {
            CssRule::Block => self.on_block(first_child),
            CssRule::AtRule => panic!("Not supported right now"),
            rule => panic!("Unexpected rule: {rule}"),
        };
        let mut rest = self.on_top_level_entries(&children[1]);
        rest.insert(0, block);
        rest
    }
}

impl Interpreter<'_, CssRule> for CssInterpreter {
    type Result = CssDocument;

    fn on_node(&self, document: &ASTNode<CssRule>) -> Option<CssDocument> {
        let children = extract_interpreter_children(document, CssRule::Document);

        if children.len() == 1 && children[0].rule == CssRule::Terminator {
            Some(CssDocument { blocks: vec![] })
        } else {
            Some(CssDocument {
                blocks: self.on_top_level_entries(&children[0]),
            })
        }
    }
}
