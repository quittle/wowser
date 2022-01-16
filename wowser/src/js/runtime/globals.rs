use super::{JsClosure, JsFunction, JsFunctionImplementation, JsValue};
use crate::util::Base64;
use std::rc::Rc;

pub fn add_globals(global_closure: &mut JsClosure) {
    add_global_function(global_closure, "atob", Rc::new(js_atob));
    add_global_function(global_closure, "btoa", Rc::new(js_btoa));
}

fn add_global_function(
    global_closure: &mut JsClosure,
    name: &str,
    func: Rc<dyn Fn(&[Rc<JsValue>]) -> Rc<JsValue>>,
) {
    let reference = global_closure.get_or_declare_reference_mut(name);
    reference.value = Rc::new(JsValue::Function(JsFunction::Native(
        name.to_string(),
        JsFunctionImplementation { func },
    )));
}

fn js_atob(args: &[Rc<JsValue>]) -> Rc<JsValue> {
    match args.get(0) {
        Some(value) => {
            if let Some(decoded) = value.to_string().base64_decode() {
                if let Ok(decoded_string) = std::str::from_utf8(&decoded) {
                    return JsValue::str_rc(decoded_string);
                }
            }
            JsValue::undefined_rc() // TODO: This should be a TypeError or DOMException when supported
        }
        _ => JsValue::undefined_rc(), // TODO: This should be a TypeError when supported
    }
}

fn js_btoa(args: &[Rc<JsValue>]) -> Rc<JsValue> {
    match args.get(0) {
        Some(value) => JsValue::string_rc(value.to_string().base64_encode()),
        _ => JsValue::undefined_rc(), // TODO: This should be a TypeError when supported
    }
}
