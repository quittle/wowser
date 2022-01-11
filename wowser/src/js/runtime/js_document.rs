use std::rc::Rc;

use super::{
    JsClosure, JsFunction, JsFunctionImplementation, JsStatement, JsStatementResult, JsValue,
};

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

fn add_globals(global_closure: &mut JsClosure) {
    // TODO: Implement real functions
    add_global_function(global_closure, "reverse", Rc::new(js_reverse));
}

fn js_reverse(args: &[JsValue]) -> JsValue {
    match args.get(0) {
        Some(JsValue::String(arg)) => JsValue::String(arg.chars().rev().collect()),
        _ => JsValue::Undefined,
    }
}

fn add_global_function(
    global_closure: &mut JsClosure,
    name: &str,
    func: Rc<dyn Fn(&[JsValue]) -> JsValue>,
) {
    let reference = global_closure.get_or_declare_reference_mut(name);
    reference.value = JsValue::Function(JsFunction::Native(
        name.to_string(),
        JsFunctionImplementation { func },
    ));
}
