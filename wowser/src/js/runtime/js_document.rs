use super::{JsStatement, JsStatementResult};

#[derive(Debug)]
pub struct JsDocument {
    pub statements: Vec<JsStatement>,
    pub expression_results: Vec<JsStatementResult>,
}

impl JsDocument {
    pub fn run(&mut self) {
        for statement in &self.statements {
            let result = statement.run();
            self.expression_results.push(result);
        }
    }
}
