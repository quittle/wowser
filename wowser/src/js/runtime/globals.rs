use super::{
    build_object_prototype, build_prototype, JsClosure, JsClosureContext, JsFunction,
    JsFunctionResult, JsNativeFunctionImplementation, JsValue, JsValueGraph, JsValueNode,
};
use crate::{garbage_collector::GcNodeGraph, util::Base64};
use std::rc::Rc;

pub fn add_globals(closure_context: &mut JsClosureContext) {
    let global_closure = closure_context.get_lastest_closure();
    add_global_function(global_closure, "atob", js_atob);
    add_global_function(global_closure, "btoa", js_btoa);
}

fn add_global_function(
    global_closure: &mut JsClosure,
    name: &str,
    func: impl Fn(JsValueNode, &[JsValueNode]) -> JsFunctionResult + 'static,
) {
    let node_graph = global_closure.node_graph.clone();
    let reference = global_closure.get_or_declare_reference_mut(name);
    let value = JsValue::Function(JsFunction::Native(
        name.to_string(),
        JsNativeFunctionImplementation {
            func: Rc::new(func),
        },
    ));
    reference.value = GcNodeGraph::create_node(&node_graph, value);
}

fn js_atob(this: JsValueNode, args: &[JsValueNode]) -> JsFunctionResult {
    let node_graph = this.get_node_graph();
    match args.get(0) {
        Some(value) => {
            if let Some(decoded) = value.map_value(|v| v.to_string().base64_decode()) {
                if let Ok(decoded_string) = std::str::from_utf8(&decoded) {
                    return Ok(this.create_new_node(JsValue::str(decoded_string)));
                }
            }
            Err(JsValue::type_error_or_dom_exception_rc(&node_graph))
        }
        _ => Err(JsValue::type_error_rc(&node_graph)),
    }
}

fn js_btoa(this: JsValueNode, args: &[JsValueNode]) -> JsFunctionResult {
    let node_graph = this.get_node_graph();
    match args.get(0) {
        Some(value) => Ok(JsValue::string_rc(
            &node_graph,
            value.map_value(|v| v.to_string().base64_encode()),
        )),
        _ => Err(JsValue::type_error_rc(&node_graph)),
    }
}

#[derive(Debug)]
pub struct GlobalPrototypes {
    pub object: JsValueNode,
    pub boolean: JsValueNode,
    pub number: JsValueNode,
    pub string: JsValueNode,
    pub function: JsValueNode,
}

impl GlobalPrototypes {
    pub fn new(node_graph: &JsValueGraph) -> Self {
        let object = build_object_prototype(node_graph);
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
