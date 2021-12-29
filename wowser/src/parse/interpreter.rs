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
