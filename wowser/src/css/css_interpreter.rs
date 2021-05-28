use super::CssRule;
use crate::parse::*;

#[derive(Debug)]
pub enum CssNode<'a> {
    Document(Vec<CssNode<'a>>),
    Block { selectors: Vec<CssNode<'a>>, properties: Vec<CssNode<'a>> },
    Selector(Vec<CssNode<'a>>),
    SelectorItem(&'a str),
    Property { key: &'a str, value: &'a str },
}

#[derive(PartialEq, Debug, Clone)]
pub struct CssDocument {
    pub blocks: Vec<CssBlock>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct CssBlock {
    pub selectors: Vec<CssSelector>,
    pub properties: Vec<CssProperty>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct CssSelector {
    pub selectors: Vec<String>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct CssProperty {
    pub key: String,
    pub value: String,
}

pub struct CssInterpreter {}

impl CssInterpreter {
    fn on_blocks<'a>(&self, blocks: &ASTNode<'a, CssRule>) -> Vec<CssBlock> {
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

    fn on_block<'a>(
        &self,
        selector_list: &ASTNode<'a, CssRule>,
        block_body: &ASTNode<'a, CssRule>,
    ) -> CssBlock {
        CssBlock {
            selectors: self.on_selector_list(selector_list),
            properties: self.on_block_body(block_body),
        }
    }

    fn on_selector_list<'a>(&self, selector_list: &ASTNode<'a, CssRule>) -> Vec<CssSelector> {
        let ASTNode { rule, children, .. } = selector_list;
        assert_eq!(**rule, CssRule::SelectorList, "Unexpected child type: {:?}", rule);
        let children_len = children.len();
        match children_len {
            1 => vec![self.on_selector(&children[0])],
            3 => {
                let mut list = vec![self.on_selector(&children[0])];
                list.append(&mut self.on_selector_list(&children[2]));
                list
            }
            _ => panic!("Unsupported number of children for SelectorList {}", children_len),
        }
    }

    fn on_selector<'a>(&self, selector: &ASTNode<'a, CssRule>) -> CssSelector {
        let ASTNode { rule, children, .. } = selector;
        assert_eq!(**rule, CssRule::Selector, "Unexpected child type: {:?}", rule);

        CssSelector {
            selectors: children
                .iter()
                .map(|selector_item| self.on_selector_item(&selector_item))
                .collect(),
        }
    }

    fn on_selector_item<'a>(&self, selector: &ASTNode<'a, CssRule>) -> String {
        let ASTNode { rule, token, children } = selector;
        assert_eq!(**rule, CssRule::SelectorItem, "Unexpected child type: {:?}", rule);
        assert_eq!(0, children.len(), "Unexpected children length");

        let parsed_token = token.expect("Missing selector item contents");
        (*parsed_token).1.to_string()
    }

    fn on_block_body<'a>(&self, block_body: &ASTNode<'a, CssRule>) -> Vec<CssProperty> {
        let ASTNode { rule, children, .. } = block_body;
        assert_eq!(**rule, CssRule::BlockBody, "Unexpected child type: {:?}", rule);
        assert_eq!(3, children.len());
        self.on_property_list(&children[1])
    }

    fn on_property_list<'a>(&self, property_list: &ASTNode<'a, CssRule>) -> Vec<CssProperty> {
        let ASTNode { rule, children, .. } = property_list;
        assert_eq!(**rule, CssRule::PropertyList, "Unexpected child type: {:?}", rule);
        children.iter().map(|child| self.on_property(child)).collect()
    }

    fn on_property<'a>(&self, property: &ASTNode<'a, CssRule>) -> CssProperty {
        let ASTNode { rule, children, .. } = property;
        assert_eq!(**rule, CssRule::Property, "Unexpected child type: {:?}", rule);
        assert_eq!(4, children.len(), "Unexpected children length");

        CssProperty {
            key: self.on_property_key(&children[0]),
            value: self.on_property_value(&children[2]),
        }
    }

    fn on_property_key<'a>(&self, selector: &ASTNode<'a, CssRule>) -> String {
        let ASTNode { rule, token, children } = selector;
        assert_eq!(**rule, CssRule::PropertyKey, "Unexpected child type: {:?}", rule);
        assert_eq!(0, children.len(), "Unexpected children length");

        let parsed_token = token.expect("Missing property key contents");
        (*parsed_token).1.to_string()
    }

    fn on_property_value<'a>(&self, selector: &ASTNode<'a, CssRule>) -> String {
        let ASTNode { rule, token, children } = selector;
        assert_eq!(**rule, CssRule::PropertyValue, "Unexpected child type: {:?}", rule);
        assert_eq!(0, children.len(), "Unexpected children length");

        let parsed_token = token.expect("Missing property value contents");
        (*parsed_token).1.to_string()
    }
}

impl<'a> Interpreter<'a> for CssInterpreter {
    type RuleType = CssRule;
    type Result = CssDocument;

    fn on_node(&self, document: &ASTNode<'a, CssRule>) -> Option<CssDocument> {
        let ASTNode { rule, children, .. } = document;
        assert_eq!(**rule, CssRule::Document, "Unexpected child type: {:?}", rule);

        if children.len() == 1 && *children[0].rule == CssRule::Terminator {
            Some(CssDocument { blocks: vec![] })
        } else {
            Some(CssDocument { blocks: self.on_blocks(&children[0]) })
        }
    }
}
