use super::parser::{ASTNode, ParserResult};
use super::rule::Rule;

/// Interprets the results of a parsing
pub trait Interpreter<'a, R: Rule> {
    /// The output of interpreting the result
    type Result;

    fn interpret(&self, ast: &ParserResult<'a, R>) -> Option<Self::Result> {
        self.on_node(&ast.node)
    }

    /// This method should be recursively called in implementations
    fn on_node(&self, ast: &ASTNode<'a, R>) -> Option<Self::Result>;
}

#[track_caller]
pub fn extract_interpreter_children<'a, R: Rule>(
    node: &'a ASTNode<'a, R>,
    expected_rule: R,
) -> &Vec<ASTNode<R>> {
    let ASTNode { rule, children, .. } = node;
    assert_eq!(
        *rule, expected_rule,
        "Expected rule of type {expected_rule}, but received {rule}",
    );
    children
}

#[track_caller]
pub fn extract_interpreter_n_children<'a, R: Rule>(
    node: &'a ASTNode<'a, R>,
    expected_rule: R,
    expected_children_length: usize,
) -> &Vec<ASTNode<R>> {
    let children = extract_interpreter_children(node, expected_rule);
    let actual_children_length = children.len();
    assert_eq!(
        actual_children_length, expected_children_length,
        "Expected {expected_rule} children for {expected_children_length} but received {actual_children_length}",
    );
    children
}

#[track_caller]
pub fn extract_interpreter_token<'a, R: Rule>(
    node: &'a ASTNode<'a, R>,
    expected_rule: R,
) -> String {
    assert_eq!(
        node.rule, expected_rule,
        "Expected token rule of type {expected_rule}, but received {}",
        node.rule,
    );

    let token_str: &str = node
        .token
        .unwrap_or_else(|| panic!("Expected token to be present on node: {}", node.rule))
        .literal;

    token_str.to_string()
}
