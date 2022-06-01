use crate::garbage_collector::GcNodeGraph;

use super::{globals::add_globals, JsClosure, JsClosureContext, JsStatement, JsValue};

#[derive(Debug)]
pub struct JsDocument {
    pub statements: Vec<JsStatement>,
    pub global_closure_context: JsClosureContext,
}

impl JsDocument {
    pub fn new(statements: Vec<JsStatement>) -> Self {
        let (nodes_graph, _node) = GcNodeGraph::new(JsValue::Null);
        let global_closure = JsClosure::new(&nodes_graph);
        let mut global_closure_context = JsClosureContext::new(global_closure);
        add_globals(&mut global_closure_context);
        Self {
            statements,
            global_closure_context,
        }
    }

    pub fn run(&mut self) {
        for statement in &self.statements {
            let result = statement.run(&mut self.global_closure_context);
            self.global_closure_context.record_new_result(result);
        }
    }
}
