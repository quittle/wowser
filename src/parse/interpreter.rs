use super::parser::{ASTNode, ParserResult};
use super::rule::Rule;

/// Interprets the results of a parsing
pub trait Interpreter {
    /// The type of rules it can interpret
    type RuleType: Rule;

    /// The output of interpreting the result
    type Result;

    fn interpret(&self, ast: &ParserResult<Self::RuleType>) -> Option<Self::Result> {
        self.on_node(&ast.node)
    }

    /// This method should be recursively called in implementations
    fn on_node(&self, ast: &ASTNode<Self::RuleType>) -> Option<Self::Result>;
}
