use super::{globals::add_globals, JsClosure, JsStatement, JsStatementResult};

#[derive(Debug)]
pub struct JsDocument {
    pub statements: Vec<JsStatement>,
    pub expression_results: Vec<JsStatementResult>,
    global_closure: JsClosure,
}

impl JsDocument {
    pub fn new(statements: Vec<JsStatement>) -> Self {
        let mut global_closure = JsClosure::default();
        add_globals(&mut global_closure);
        Self {
            statements,
            expression_results: vec![],
            global_closure,
        }
    }

    pub fn run(&mut self) {
        for statement in &self.statements {
            let result = statement.run(&mut self.global_closure);
            self.expression_results.push(result);
        }
    }
}
