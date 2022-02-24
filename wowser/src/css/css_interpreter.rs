use std::rc::Rc;

use super::{CssAtRule, CssBlock, CssDocument, CssProperty, CssRule, CssSelectorChainItem};
use crate::parse::*;

type CssASTNode<'a> = ASTNode<'a, CssRule>;

#[allow(dead_code)]
fn on_blocks(blocks: &CssASTNode) -> Vec<CssBlock> {
    let children = extract_interpreter_children(blocks, CssRule::Blocks);

    children.iter().map(on_block).collect()
}

fn on_block(node: &CssASTNode) -> CssBlock {
    let children = extract_interpreter_n_children(node, CssRule::Block, 2);

    CssBlock {
        selectors: on_selector_list(&children[0]),
        properties: on_block_body(&children[1]),
    }
}

fn on_selector_list(selector_list: &CssASTNode) -> Vec<Vec<CssSelectorChainItem>> {
    let children = extract_interpreter_children(selector_list, CssRule::SelectorList);
    let children_len = children.len();
    match children_len {
        1 => vec![on_selector(&children[0])],
        3 => {
            let mut list = vec![on_selector(&children[0])];
            list.append(&mut on_selector_list(&children[2]));
            list
        }
        _ => panic!("Unsupported number of children for SelectorList {children_len}",),
    }
}

fn on_selector(selector: &CssASTNode) -> Vec<CssSelectorChainItem> {
    let children = extract_interpreter_children(selector, CssRule::Selector);
    assert!(!children.is_empty(), "Expected at least one child");

    children.iter().map(on_selector_item).collect()
}

fn on_selector_item(selector: &CssASTNode) -> CssSelectorChainItem {
    let token = extract_interpreter_token(selector, CssRule::SelectorItem);

    if let Some(class) = token.strip_prefix('.') {
        CssSelectorChainItem::Class(class.to_string())
    } else if let Some(id) = token.strip_prefix('#') {
        CssSelectorChainItem::Id(id.to_string())
    } else {
        CssSelectorChainItem::Tag(token.to_string())
    }
}

fn on_block_body(block_body: &CssASTNode) -> Vec<Rc<CssProperty>> {
    let children = extract_interpreter_n_children(block_body, CssRule::BlockBody, 3);
    on_property_list(&children[1])
}

fn on_property_list(property_list: &CssASTNode) -> Vec<Rc<CssProperty>> {
    let children = extract_interpreter_children(property_list, CssRule::PropertyList);
    assert!(
        children.len() == 1 || children.len() == 2,
        "Unexpected number of children"
    );
    let mut properties = on_strict_property_list(&children[0]);
    if let Some(trailing_property) = children.get(1) {
        properties.push(Rc::new(on_trailing_property(trailing_property)));
    }

    properties
}

fn on_strict_property_list(property_list: &CssASTNode) -> Vec<Rc<CssProperty>> {
    let children = extract_interpreter_children(property_list, CssRule::StrictPropertyList);
    children.iter().map(on_property).map(Rc::new).collect()
}

fn on_property(property: &CssASTNode) -> CssProperty {
    let children = extract_interpreter_n_children(property, CssRule::Property, 4);

    CssProperty {
        key: on_property_key(&children[0]),
        value: on_property_value(&children[2]),
    }
}

fn on_trailing_property(property: &CssASTNode) -> CssProperty {
    let children = extract_interpreter_n_children(property, CssRule::TrailingProperty, 3);

    CssProperty {
        key: on_property_key(&children[0]),
        value: on_property_value(&children[2]),
    }
}

fn on_property_key(selector: &CssASTNode) -> String {
    let token = extract_interpreter_token(selector, CssRule::PropertyKey);

    token.trim().to_string()
}

fn on_property_value(selector: &CssASTNode) -> String {
    let token = extract_interpreter_token(selector, CssRule::PropertyValue);
    token.trim().to_string()
}

fn on_top_level_entries(node: &CssASTNode) -> Vec<CssBlock> {
    let children = extract_interpreter_children(node, CssRule::TopLevelEntries);
    if children.is_empty() {
        return vec![];
    }

    let first_child = &children[0];
    let block = match &first_child.rule {
        CssRule::Block => on_block(first_child),
        CssRule::AtRule => {
            on_at_rule(first_child);
            panic!("Unsupported still")
        }
        rule => panic!("Unexpected rule: {rule}"),
    };
    let mut rest = on_top_level_entries(&children[1]);
    rest.insert(0, block);
    rest
}

fn on_at_rule(node: &CssASTNode) -> CssAtRule {
    let children = extract_interpreter_n_children(node, CssRule::AtRule, 3);

    let rule = on_at_keyword(&children[0]);
    let args = on_at_keyword_symbols(&children[1]);
    let blocks = on_blocks(&children[2]);
    CssAtRule { rule, args, blocks }
}

fn on_at_keyword(node: &CssASTNode) -> String {
    let token = extract_interpreter_token(node, CssRule::AtKeyword);

    token.trim().to_string()
}

fn on_at_keyword_symbols(node: &CssASTNode) -> Vec<String> {
    let children = extract_interpreter_children(node, CssRule::AtKeywordSymbols);

    children.iter().map(on_at_keyword_symbol).collect()
}

fn on_at_keyword_symbol(node: &CssASTNode) -> String {
    let token = extract_interpreter_token(node, CssRule::AtKeywordSymbol);

    token.trim().to_string()
}

pub struct CssInterpreter {}

impl Interpreter<'_, CssRule> for CssInterpreter {
    type Result = CssDocument;

    fn on_node(&self, document: &ASTNode<CssRule>) -> Option<CssDocument> {
        let children = extract_interpreter_children(document, CssRule::Document);

        if children.len() == 1 && children[0].rule == CssRule::Terminator {
            Some(CssDocument { blocks: vec![] })
        } else {
            Some(CssDocument {
                blocks: on_top_level_entries(&children[0]),
            })
        }
    }
}
