use super::{globals::add_globals, JsClosure, JsClosureContext, JsStatement};

#[derive(Debug)]
pub struct JsDocument {
    pub statements: Vec<JsStatement>,
    pub global_closure_context: JsClosureContext,
}

impl JsDocument {
    pub fn new(statements: Vec<JsStatement>) -> Self {
        let mut global_closure = JsClosure::default();
        add_globals(&mut global_closure);
        Self {
            statements,
            global_closure_context: JsClosureContext::new(global_closure),
        }
    }

    pub fn run(&mut self) {
        for statement in &self.statements {
            let result = statement.run(&mut self.global_closure_context);
            self.global_closure_context.expression_results.push(result);
        }
    }
}
