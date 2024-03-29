use super::{globals::GlobalPrototypes, JsClosure, JsReference, JsStatementResult, JsValueGraph};

#[derive(Debug)]
pub struct JsClosureContext {
    // Global closure at the beginning. Immediate closure at the end.
    // This should not be leaked to ensure there is always at least one entry in the closure
    closures: Vec<JsClosure>,
    pub expression_results: Vec<JsStatementResult>,
    pub global_prototypes: GlobalPrototypes,
    pub nodes_graph: JsValueGraph,
}

impl JsClosureContext {
    pub fn new(global: JsClosure) -> Self {
        let global_prototypes = GlobalPrototypes::new(&global.node_graph);
        let nodes_graph = global.node_graph.clone();
        Self {
            closures: vec![global],
            expression_results: vec![],
            global_prototypes,
            nodes_graph,
        }
    }

    pub fn with_new_context<T, F>(&mut self, func: F) -> T
    where
        F: Fn(&mut Self) -> T,
    {
        self.closures.push(JsClosure::new(&self.nodes_graph));
        let ret = func(self);
        self.closures.pop();
        ret
    }

    pub fn get_lastest_closure(&mut self) -> &mut JsClosure {
        self.closures.last_mut().unwrap()
    }

    pub fn has_reference(&self, variable_name: &str) -> bool {
        self.closures
            .iter()
            .any(|closure| closure.has_reference(variable_name))
    }

    pub fn get_reference(&self, variable_name: &str) -> Option<&JsReference> {
        self.closures
            .iter()
            .rev()
            .find_map(|closure| closure.get_reference(variable_name))
    }

    pub fn get_reference_mut(&mut self, variable_name: &str) -> Option<&mut JsReference> {
        self.closures
            .iter_mut()
            .rev()
            .find_map(|closure| closure.get_reference_mut(variable_name))
    }

    pub fn get_or_declare_reference_mut(&mut self, variable_name: &str) -> &mut JsReference {
        // This is only necessary because extracting it first and checking for None causes the
        // borrow checker unnecessary pains.
        if self.has_reference(variable_name) {
            self.get_reference_mut(variable_name).unwrap()
        } else {
            self.get_lastest_closure()
                .get_or_declare_reference_mut(variable_name)
        }
    }

    pub fn record_new_result(&mut self, result: JsStatementResult) {
        self.expression_results.push(result);
    }

    pub fn get_closure_depth(&self) -> usize {
        self.closures.len()
    }
}
