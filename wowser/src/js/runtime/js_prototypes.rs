use std::{collections::HashMap, rc::Rc};

use crate::garbage_collector::GcNodeGraph;

use super::{
    JsClosureContext, JsFunction, JsNativeFunctionImplementation, JsValue, JsValueGraph,
    JsValueNode,
};

const PROTOTYPE_MEMBER: &str = "__proto__";

pub fn get_member_from_prototype_chain(
    value: &JsValue,
    member: &str,
    closure_context: &JsClosureContext,
) -> JsValueNode {
    let nodes_graph = &closure_context.nodes_graph;
    match value {
        JsValue::Boolean(_) => get_member_from_prototype_chain(
            closure_context.global_prototypes.boolean.get_ref(),
            member,
            closure_context,
        ),
        JsValue::Number(_) => get_member_from_prototype_chain(
            closure_context.global_prototypes.number.get_ref(),
            member,
            closure_context,
        ),
        JsValue::String(_) => get_member_from_prototype_chain(
            closure_context.global_prototypes.string.get_ref(),
            member,
            closure_context,
        ),
        JsValue::Function(_) => get_member_from_prototype_chain(
            closure_context.global_prototypes.function.get_ref(),
            member,
            closure_context,
        ),
        o @ JsValue::Object(map) => {
            if let Some(value) = map.get(member) {
                return value.clone();
            }
            if let Some(parent) = map.get(PROTOTYPE_MEMBER) {
                return get_member_from_prototype_chain(parent.get_ref(), member, closure_context);
            }

            let object_prototype = closure_context.global_prototypes.object.get_ref();
            if o as *const JsValue != object_prototype as *const JsValue {
                return get_member_from_prototype_chain(object_prototype, member, closure_context);
            }
            JsValue::undefined_rc(nodes_graph)
        }
        JsValue::Undefined => JsValue::undefined_rc(nodes_graph), // TODO: TypeError
        JsValue::Null => JsValue::undefined_rc(nodes_graph),      // TODO: TypeError
    }
}

pub fn build_object_prototype(node_graph: &JsValueGraph) -> JsValueNode {
    build_prototype(
        JsValue::null_rc(node_graph),
        [build_function_entry(
            node_graph,
            "toString",
            object_to_string,
        )],
    )
}

pub fn build_prototype<const N: usize>(
    parent: JsValueNode,
    members: [(String, JsValueNode); N],
) -> JsValueNode {
    let node_graph = parent.get_node_graph();
    let mut map = HashMap::from(members);
    map.insert(PROTOTYPE_MEMBER.to_string(), parent);
    JsValue::object_rc(&node_graph, map)
}

fn build_function_entry(
    node_graph: &JsValueGraph,
    name: &str,
    func: impl Fn(JsValueNode, &[JsValueNode]) -> JsValueNode + 'static,
) -> (String, JsValueNode) {
    (
        name.to_string(),
        GcNodeGraph::create_node(
            node_graph,
            JsValue::Function(JsFunction::Native(
                name.to_string(),
                JsNativeFunctionImplementation {
                    func: Rc::new(func),
                },
            )),
        ),
    )
}

fn object_to_string(this: JsValueNode, _args: &[JsValueNode]) -> JsValueNode {
    JsValue::string_rc(&this.get_node_graph(), this.get_ref().to_string())
}
