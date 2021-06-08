use super::{html_document::*, html_rule::HtmlRule};
use crate::parse::*;

pub struct HtmlInterpreter {}

impl HtmlInterpreter {
    fn on_document<'a>(&self, document: &ASTNode<'a, HtmlRule>) -> DocumentHtmlNode<'a> {
        let ASTNode { rule, children, .. } = document;

        self.assert_rule_is(rule, HtmlRule::Document);

        let mut doctype = DoctypeHtmlNode::default();
        let mut contents = vec![];
        for child in children {
            let rule = &*child.rule;
            match rule {
                HtmlRule::Doctype => doctype = self.on_doctype(child),
                HtmlRule::TagContents => contents = self.on_tag_contents(child),
                HtmlRule::Terminator => {}
                _ => panic!("Invalid child rule type for Document: {}", rule),
            }
        }
        DocumentHtmlNode { doctype, contents }
    }

    fn on_doctype<'a>(&self, doctype: &ASTNode<'a, HtmlRule>) -> DoctypeHtmlNode<'a> {
        let ASTNode { rule, children, .. } = doctype;

        self.assert_rule_is(rule, HtmlRule::Doctype);
        self.assert_children_length(children, 3);

        DoctypeHtmlNode { document_type_definition: self.on_doctype_contents(&children[1]) }
    }

    fn on_doctype_contents<'a>(&self, doctype_contents: &ASTNode<'a, HtmlRule>) -> Vec<&'a str> {
        let ASTNode { rule, children, .. } = doctype_contents;

        self.assert_rule_is(rule, HtmlRule::DoctypeContents);

        children.iter().map(|child| self.on_doctype_contents_string(child)).collect()
    }

    fn on_doctype_contents_string<'a>(
        &self,
        doctype_contents_string: &ASTNode<'a, HtmlRule>,
    ) -> &'a str {
        let ASTNode { rule, children, token } = doctype_contents_string;

        self.assert_rule_is(rule, HtmlRule::DoctypeContentsString);
        self.assert_no_children(children);

        self.extract_token(token)
    }

    fn on_non_self_closing_tag<'a>(
        &self,
        non_self_closing_tag: &ASTNode<'a, HtmlRule>,
    ) -> ElementHtmlNode<'a> {
        let ASTNode { rule, children, .. } = non_self_closing_tag;

        self.assert_rule_is(rule, HtmlRule::NonSelfClosingTag);
        self.assert_children_length(children, 3);

        let (tag_name, attributes) = self.on_opening_tag(&children[0]);
        let children = self.on_tag_contents(&children[1]);
        ElementHtmlNode { tag_name, attributes, children }
    }

    fn on_self_closing_tag<'a>(
        &self,
        self_closing_tag: &ASTNode<'a, HtmlRule>,
    ) -> ElementHtmlNode<'a> {
        let ASTNode { rule, children, .. } = self_closing_tag;

        self.assert_rule_is(rule, HtmlRule::SelfClosingTag);
        self.assert_children_length(children, 2);

        let (tag_name, attributes) = self.on_opening_tag_prelude(&children[0]);
        ElementHtmlNode { tag_name, attributes, children: vec![] }
    }

    fn on_opening_tag<'a>(
        &self,
        opening_tag: &ASTNode<'a, HtmlRule>,
    ) -> (&'a str, Vec<TagAttributeHtmlNode<'a>>) {
        let ASTNode { rule, children, .. } = opening_tag;

        self.assert_rule_is(rule, HtmlRule::OpeningTag);
        self.assert_children_length(children, 2);

        self.on_opening_tag_prelude(&children[0])
    }

    fn on_opening_tag_prelude<'a>(
        &self,
        opening_tag_prelude: &ASTNode<'a, HtmlRule>,
    ) -> (&'a str, Vec<TagAttributeHtmlNode<'a>>) {
        let ASTNode { rule, children, .. } = opening_tag_prelude;

        self.assert_rule_is(rule, HtmlRule::OpeningTagPrelude);
        self.assert_children_length(children, 2);

        let tag_name = self.on_opening_tag_name(&children[0]);
        let attributes = self.on_opening_tag_attributes(&children[1]);

        (tag_name, attributes)
    }

    fn on_opening_tag_name<'a>(&self, opening_tag_name: &ASTNode<'a, HtmlRule>) -> &'a str {
        let ASTNode { rule, children, token } = opening_tag_name;

        self.assert_rule_is(rule, HtmlRule::OpeningTagName);
        self.assert_no_children(children);

        self.extract_token(token)
    }

    fn on_opening_tag_attributes<'a>(
        &self,
        opening_tag_attributes: &ASTNode<'a, HtmlRule>,
    ) -> Vec<TagAttributeHtmlNode<'a>> {
        let ASTNode { rule, children, .. } = opening_tag_attributes;

        self.assert_rule_is(rule, HtmlRule::OpeningTagAttributes);

        children.iter().map(|child| self.on_tag_attribute(child)).collect()
    }

    fn on_tag_attribute<'a>(
        &self,
        tag_attribute: &ASTNode<'a, HtmlRule>,
    ) -> TagAttributeHtmlNode<'a> {
        let ASTNode { rule, children, .. } = tag_attribute;

        self.assert_rule_is(rule, HtmlRule::TagAttribute);
        self.assert_children_length_one_of(children, vec![1, 2, 3]);

        let name = self.on_attribute_name(&children[0]);
        let value =
            if children.len() == 3 { Some(self.on_attribute_value(&children[2])) } else { None };

        TagAttributeHtmlNode { name, value }
    }

    fn on_attribute_name<'a>(&self, attribute_name: &ASTNode<'a, HtmlRule>) -> &'a str {
        let ASTNode { rule, children, token } = attribute_name;

        self.assert_rule_is(rule, HtmlRule::AttributeName);
        self.assert_no_children(children);

        self.extract_token(token)
    }

    fn on_attribute_value<'a>(&self, attribute_name: &ASTNode<'a, HtmlRule>) -> &'a str {
        let ASTNode { rule, children, token } = attribute_name;

        self.assert_rule_is(rule, HtmlRule::AttributeValue);
        self.assert_no_children(children);

        self.extract_token(token)
    }

    fn on_tag_contents<'a>(
        &self,
        tag_contents: &ASTNode<'a, HtmlRule>,
    ) -> Vec<ElementContents<'a>> {
        let ASTNode { rule, children, .. } = tag_contents;

        self.assert_rule_is(rule, HtmlRule::TagContents);
        self.assert_children_length_one_of(children, vec![1, 2]);

        let first_child = &children[0];
        let mut initial_contents = match *(first_child.rule) {
            HtmlRule::Text => vec![ElementContents::Text(self.on_text(first_child))],
            HtmlRule::TagsAndText => self.on_tags_and_text(first_child),
            _ => panic!("Unsupported child type: {:?}", first_child.rule),
        };
        if children.len() == 2 {
            let remainder = self.on_tags_and_text(&children[1]);
            initial_contents.extend(remainder);
        }

        initial_contents
    }

    fn on_tags_and_text<'a>(
        &self,
        tags_and_text: &ASTNode<'a, HtmlRule>,
    ) -> Vec<ElementContents<'a>> {
        let ASTNode { rule, children, .. } = tags_and_text;

        self.assert_rule_is(rule, HtmlRule::TagsAndText);

        children.iter().flat_map(|child| self.on_tag_and_text(child)).collect()
    }

    fn on_tag_and_text<'a>(
        &self,
        tag_and_text: &ASTNode<'a, HtmlRule>,
    ) -> Vec<ElementContents<'a>> {
        let ASTNode { rule, children, .. } = tag_and_text;

        self.assert_rule_is(rule, HtmlRule::TagAndText);
        self.assert_children_length_one_of(children, vec![1, 2]);

        let element = ElementContents::Element(self.on_tag(&children[0]));
        if children.len() == 2 {
            let text = ElementContents::Text(self.on_text(&children[1]));
            vec![element, text]
        } else {
            vec![element]
        }
    }

    fn on_tag<'a>(&self, tag: &ASTNode<'a, HtmlRule>) -> ElementHtmlNode<'a> {
        let ASTNode { rule, children, .. } = tag;

        self.assert_rule_is(rule, HtmlRule::Tag);
        self.assert_children_length(children, 1);

        let child = &children[0];
        match *child.rule {
            HtmlRule::SelfClosingTag => self.on_self_closing_tag(child),
            HtmlRule::NonSelfClosingTag => self.on_non_self_closing_tag(child),
            _ => panic!("Unsupported child type {:?}", child.rule),
        }
    }

    fn on_text<'a>(&self, text_node: &ASTNode<'a, HtmlRule>) -> TextHtmlNode<'a> {
        let ASTNode { rule, children, token } = text_node;

        self.assert_rule_is(rule, HtmlRule::Text);
        self.assert_no_children(children);

        TextHtmlNode { text: self.extract_token(token) }
    }

    fn assert_rule_is(&self, rule: &HtmlRule, expected_rule: HtmlRule) {
        assert_eq!(*rule, expected_rule, "Unexpected child type: {:?}", rule);
    }

    fn assert_children_length(&self, children: &[ASTNode<HtmlRule>], length: usize) {
        assert_eq!(children.len(), length, "Unexpected number of children");
    }

    fn assert_children_length_one_of(&self, children: &[ASTNode<HtmlRule>], lengths: Vec<usize>) {
        let children_len = children.len();
        assert!(
            lengths.iter().any(|length| &children_len == length),
            "Unexpected number of children. Found {}",
            children_len,
        );
    }

    fn assert_no_children(&self, children: &[ASTNode<HtmlRule>]) {
        assert!(children.is_empty(), "Expected no children but found {}", children.len());
    }

    fn extract_token<'a>(&self, token: &Option<&(Box<dyn Token>, &'a str)>) -> &'a str {
        (*token).expect("Missing token for required rule").1
    }
}

impl<'a> Interpreter<'a> for HtmlInterpreter {
    type RuleType = HtmlRule;
    type Result = DocumentHtmlNode<'a>;

    fn on_node(&self, ast: &ASTNode<'a, HtmlRule>) -> Option<DocumentHtmlNode<'a>> {
        return Some(self.on_document(ast));
    }
}
