use std::rc::Rc;

use crate::util::Base64;

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
    add_global_function(global_closure, "atob", Rc::new(js_atob));
    add_global_function(global_closure, "btoa", Rc::new(js_btoa));
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

fn js_atob(args: &[JsValue]) -> JsValue {
    match args.get(0) {
        Some(value) => {
            println!("decoding: {}", value.to_string());
            if let Some(decoded) = value.to_string().base64_decode() {
                println!("Decoded: {:?}", decoded);
                if let Ok(decoded_string) = std::str::from_utf8(&decoded) {
                    println!("utf8: {:?}", decoded_string);
                    return JsValue::str(decoded_string);
                }
            }
            JsValue::Undefined // TODO: This should be a TypeError or DOMException when supported
        }
        _ => JsValue::Undefined, // TODO: This should be a TypeError when supported
    }
}

fn js_btoa(args: &[JsValue]) -> JsValue {
    match args.get(0) {
        Some(value) => JsValue::String(value.to_string().base64_encode()),
        _ => JsValue::Undefined, // TODO: This should be a TypeError when supported
    }
}
