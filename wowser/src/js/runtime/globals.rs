use super::{
    build_object_prototype, build_prototype, JsClosure, JsFunction, JsFunctionImplementation,
    JsValue,
};
use crate::util::Base64;
use std::rc::Rc;

pub fn add_globals(global_closure: &mut JsClosure) {
    add_global_function(global_closure, "atob", js_atob);
    add_global_function(global_closure, "btoa", js_btoa);
}

fn add_global_function(
    global_closure: &mut JsClosure,
    name: &str,
    func: impl Fn(Rc<JsValue>, &[Rc<JsValue>]) -> Rc<JsValue> + 'static,
) {
    let reference = global_closure.get_or_declare_reference_mut(name);
    reference.value = Rc::new(JsValue::Function(JsFunction::Native(
        name.to_string(),
        JsFunctionImplementation {
            func: Rc::new(func),
        },
    )));
}

fn js_atob(_this: Rc<JsValue>, args: &[Rc<JsValue>]) -> Rc<JsValue> {
    match args.get(0) {
        Some(value) => {
            if let Some(decoded) = value.to_string().base64_decode() {
                if let Ok(decoded_string) = std::str::from_utf8(&decoded) {
                    return JsValue::str_rc(decoded_string);
                }
            }
            JsValue::type_error_or_dom_exception_rc()
        }
        _ => JsValue::type_error_rc(),
    }
}

fn js_btoa(_this: Rc<JsValue>, args: &[Rc<JsValue>]) -> Rc<JsValue> {
    match args.get(0) {
        Some(value) => JsValue::string_rc(value.to_string().base64_encode()),
        _ => JsValue::type_error_rc(),
    }
}

#[derive(Debug)]
pub struct GlobalPrototypes {
    pub object: Rc<JsValue>,
    pub boolean: Rc<JsValue>,
    pub number: Rc<JsValue>,
    pub string: Rc<JsValue>,
    pub function: Rc<JsValue>,
}

impl Default for GlobalPrototypes {
    fn default() -> Self {
        let object = build_object_prototype();
        let boolean = build_prototype(object.clone(), []);
        let number = build_prototype(object.clone(), []);
        let string = build_prototype(object.clone(), []);
        let function = build_prototype(object.clone(), []);

        Self {
            object,
            boolean,
            number,
            string,
            function,
        }
    }
}
