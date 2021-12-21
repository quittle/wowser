use super::JsValue;

/// Represents the resulting value of evaluating a statement
#[derive(Debug, PartialEq)]
pub enum JsStatementResult {
    Value(JsValue),
    Void,
}
