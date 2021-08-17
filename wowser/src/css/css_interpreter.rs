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

pub struct CssInterpreter {}

impl CssInterpreter {
    fn on_blocks(&self, blocks: &ASTNode<CssRule>) -> Vec<CssBlock> {
        let ASTNode { rule, children, .. } = blocks;
        assert_eq!(**rule, CssRule::Blocks, "Unexpected child type: {:?}", rule);

        children
            .iter()
            .map(|node| {
                let ASTNode { rule, children, .. } = node;
                assert_eq!(**rule, CssRule::Block, "Unexpected child type: {:?}", rule);
                assert_eq!(2, children.len());
                self.on_block(&children[0], &children[1])
            })
            .collect()
    }

    fn on_block(
        &self,
        selector_list: &ASTNode<CssRule>,
        block_body: &ASTNode<CssRule>,
    ) -> CssBlock {
        CssBlock {
            selectors: self.on_selector_list(selector_list),
            properties: self.on_block_body(block_body),
        }
    }

    fn on_selector_list(&self, selector_list: &ASTNode<CssRule>) -> Vec<CssSelectorChain> {
        let ASTNode { rule, children, .. } = selector_list;
        assert_eq!(
            **rule,
            CssRule::SelectorList,
            "Unexpected child type: {:?}",
            rule
        );
        let children_len = children.len();
        match children_len {
            1 => vec![self.on_selector(&children[0])],
            3 => {
                let mut list = vec![self.on_selector(&children[0])];
                list.append(&mut self.on_selector_list(&children[2]));
                list
            }
            _ => panic!(
                "Unsupported number of children for SelectorList {}",
                children_len
            ),
        }
    }

    fn on_selector(&self, selector: &ASTNode<CssRule>) -> CssSelectorChain {
        let ASTNode { rule, children, .. } = selector;
        assert_eq!(
            **rule,
            CssRule::Selector,
            "Unexpected child type: {:?}",
            rule
        );
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

    fn on_selector_item(&self, selector: &ASTNode<CssRule>) -> CssSelectorChainItem {
        let ASTNode {
            rule,
            token,
            children,
        } = selector;
        assert_eq!(
            **rule,
            CssRule::SelectorItem,
            "Unexpected child type: {:?}",
            rule
        );
        assert_eq!(0, children.len(), "Unexpected children length");

        let parsed_token = token.expect("Missing selector item contents").1;

        if let Some(class) = parsed_token.strip_prefix('.') {
            CssSelectorChainItem::Class(class.to_string())
        } else if let Some(id) = parsed_token.strip_prefix('#') {
            CssSelectorChainItem::Id(id.to_string())
        } else {
            CssSelectorChainItem::Tag(parsed_token.to_string())
        }
    }

    fn on_block_body(&self, block_body: &ASTNode<CssRule>) -> Vec<Rc<CssProperty>> {
        let ASTNode { rule, children, .. } = block_body;
        assert_eq!(
            **rule,
            CssRule::BlockBody,
            "Unexpected child type: {:?}",
            rule
        );
        assert_eq!(3, children.len());
        self.on_property_list(&children[1])
    }

    fn on_property_list(&self, property_list: &ASTNode<CssRule>) -> Vec<Rc<CssProperty>> {
        let ASTNode { rule, children, .. } = property_list;
        assert_eq!(
            **rule,
            CssRule::PropertyList,
            "Unexpected child type: {:?}",
            rule
        );
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

    fn on_strict_property_list(&self, property_list: &ASTNode<CssRule>) -> Vec<Rc<CssProperty>> {
        let ASTNode { rule, children, .. } = property_list;
        assert_eq!(
            **rule,
            CssRule::StrictPropertyList,
            "Unexpected child type: {:?}",
            rule
        );
        children
            .iter()
            .map(|child| Rc::new(self.on_property(child)))
            .collect()
    }

    fn on_property(&self, property: &ASTNode<CssRule>) -> CssProperty {
        let ASTNode { rule, children, .. } = property;
        assert_eq!(
            **rule,
            CssRule::Property,
            "Unexpected child type: {:?}",
            rule
        );
        assert_eq!(4, children.len(), "Unexpected children length");

        CssProperty {
            key: self.on_property_key(&children[0]),
            value: self.on_property_value(&children[2]),
        }
    }

    fn on_trailing_property(&self, property: &ASTNode<CssRule>) -> CssProperty {
        let ASTNode { rule, children, .. } = property;
        assert_eq!(
            **rule,
            CssRule::TrailingProperty,
            "Unexpected child type: {:?}",
            rule
        );
        assert_eq!(3, children.len(), "Unexpected children length");

        CssProperty {
            key: self.on_property_key(&children[0]),
            value: self.on_property_value(&children[2]),
        }
    }

    fn on_property_key(&self, selector: &ASTNode<CssRule>) -> String {
        let ASTNode {
            rule,
            token,
            children,
        } = selector;
        assert_eq!(
            **rule,
            CssRule::PropertyKey,
            "Unexpected child type: {:?}",
            rule
        );
        assert_eq!(0, children.len(), "Unexpected children length");

        let parsed_token = token.expect("Missing property key contents");
        (*parsed_token).1.trim().to_string()
    }

    fn on_property_value(&self, selector: &ASTNode<CssRule>) -> String {
        let ASTNode {
            rule,
            token,
            children,
        } = selector;
        assert_eq!(
            **rule,
            CssRule::PropertyValue,
            "Unexpected child type: {:?}",
            rule
        );
        assert_eq!(0, children.len(), "Unexpected children length");

        let parsed_token = token.expect("Missing property value contents");
        (*parsed_token).1.trim().to_string()
    }
}

impl Interpreter<'_> for CssInterpreter {
    type RuleType = CssRule;
    type Result = CssDocument;

    fn on_node(&self, document: &ASTNode<CssRule>) -> Option<CssDocument> {
        let ASTNode { rule, children, .. } = document;
        assert_eq!(
            **rule,
            CssRule::Document,
            "Unexpected child type: {:?}",
            rule
        );

        if children.len() == 1 && *children[0].rule == CssRule::Terminator {
            Some(CssDocument { blocks: vec![] })
        } else {
            Some(CssDocument {
                blocks: self.on_blocks(&children[0]),
            })
        }
    }
}
