use super::{JsExpression, JsReference, JsStatementResult};

#[derive(Debug)]
pub enum JsStatement {
    Empty,
    Expression(JsExpression),
    VarDeclaration(JsReference),
}

impl JsStatement {
    pub fn run(&self) -> JsStatementResult {
        match self {
            Self::Empty => JsStatementResult::Void,
            Self::Expression(expression) => JsStatementResult::Value(expression.run()),
            Self::VarDeclaration(_reference) => JsStatementResult::Void,
        }
    }
}
