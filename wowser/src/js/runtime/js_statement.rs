use super::{JsExpression, JsStatementResult};

#[derive(Debug)]
pub struct JsStatement {
    pub expression: Option<JsExpression>,
}

impl JsStatement {
    pub fn run(&self) -> JsStatementResult {
        if let Some(expression) = &self.expression {
            JsStatementResult::Value(expression.run())
        } else {
            JsStatementResult::Void
        }
    }
}
