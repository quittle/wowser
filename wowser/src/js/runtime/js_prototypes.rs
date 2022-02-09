use std::{collections::HashMap, rc::Rc};

use super::{JsClosureContext, JsFunction, JsFunctionImplementation, JsValue};

const PROTOTYPE_MEMBER: &str = "__proto__";

pub fn get_member_from_prototype_chain(
    value: &JsValue,
    member: &str,
    closure_context: &JsClosureContext,
) -> Rc<JsValue> {
    match value {
        JsValue::Boolean(_) => get_member_from_prototype_chain(
            closure_context.global_prototypes.boolean.as_ref(),
            member,
            closure_context,
        ),
        JsValue::Number(_) => get_member_from_prototype_chain(
            closure_context.global_prototypes.number.as_ref(),
            member,
            closure_context,
        ),
        JsValue::String(_) => get_member_from_prototype_chain(
            closure_context.global_prototypes.string.as_ref(),
            member,
            closure_context,
        ),
        JsValue::Function(_) => get_member_from_prototype_chain(
            closure_context.global_prototypes.function.as_ref(),
            member,
            closure_context,
        ),
        o @ JsValue::Object(map) => {
            if let Some(value) = map.get(member) {
                return value.clone();
            }
            if let Some(parent) = map.get(PROTOTYPE_MEMBER) {
                return get_member_from_prototype_chain(parent.as_ref(), member, closure_context);
            }

            let object_prototype = closure_context.global_prototypes.object.as_ref();
            if o as *const JsValue != object_prototype as *const JsValue {
                return get_member_from_prototype_chain(object_prototype, member, closure_context);
            }
            JsValue::undefined_rc()
        }
        JsValue::Undefined => JsValue::undefined_rc(), // TODO: TypeError
        JsValue::Null => JsValue::undefined_rc(),      // TODO: TypeError
    }
}

pub fn build_object_prototype() -> Rc<JsValue> {
    build_prototype(
        JsValue::null_rc(),
        [build_function_entry("toString", object_to_string)],
    )
}

pub fn build_prototype<const N: usize>(
    parent: Rc<JsValue>,
    members: [(String, Rc<JsValue>); N],
) -> Rc<JsValue> {
    let mut map = HashMap::from(members);
    map.insert(PROTOTYPE_MEMBER.to_string(), parent);
    JsValue::object_rc(map)
}

fn build_function_entry(
    name: &str,
    func: impl Fn(Rc<JsValue>, &[Rc<JsValue>]) -> Rc<JsValue> + 'static,
) -> (String, Rc<JsValue>) {
    (
        name.to_string(),
        Rc::new(JsValue::Function(JsFunction::Native(
            name.to_string(),
            JsFunctionImplementation {
                func: Rc::new(func),
            },
        ))),
    )
}

fn object_to_string(this: Rc<JsValue>, _args: &[Rc<JsValue>]) -> Rc<JsValue> {
    JsValue::string_rc(this.as_ref().to_string())
}
