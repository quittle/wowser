use std::rc::Rc;

use super::{JsClosureContext, JsStatement, JsStatementResult, JsValue, JsValueNode};

#[derive(Clone)]
#[allow(clippy::type_complexity)]
pub struct JsNativeFunctionImplementation {
    pub func: Rc<dyn Fn(JsValueNode, &[JsValueNode]) -> JsValueNode>,
}

impl Default for JsNativeFunctionImplementation {
    fn default() -> Self {
        Self {
            func: Rc::new(|this, _| JsValue::undefined_rc(&this.get_node_graph())),
        }
    }
}

impl std::fmt::Debug for JsNativeFunctionImplementation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsNativeFunctionImplementation")
            .field("func", &"[Native]".to_string())
            .finish()
    }
}

impl std::cmp::PartialEq for JsNativeFunctionImplementation {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

// Functions cannot be compared and are always considered unequal
#[derive(Debug, PartialEq)]
pub enum JsFunction {
    Native(String, JsNativeFunctionImplementation),
    UserDefined(
        // Source Text
        String,
        // Name
        String,
        // Params
        Vec<String>,
        // Implementation
        Vec<JsStatement>,
    ),
}

impl JsFunction {
    pub fn get_name(&self) -> &str {
        match self {
            Self::Native(name, _) => name,
            Self::UserDefined(_source, name, _params, _implementation) => name,
        }
    }

    pub fn run(
        &self,
        closure_context: &mut JsClosureContext,
        this: JsValueNode,
        args: &[JsValueNode],
    ) -> JsValueNode {
        match self {
            Self::Native(_, implementation) => implementation.func.as_ref()(this, args),
            Self::UserDefined(_source, _name, params, implementation) => {
                if closure_context.get_closure_depth() > 255 {
                    return JsValue::stack_overflow_error_rc(&closure_context.nodes_graph);
                }

                closure_context.with_new_context(|closure_context| {
                    let closure = closure_context.get_lastest_closure();
                    for (index, param_name) in params.iter().enumerate() {
                        // Ensure all params are declared
                        let reference = closure.get_or_declare_reference_mut(param_name);

                        // Assign args that line up with parameters are assigned
                        if let Some(arg) = args.get(index) {
                            reference.value = arg.clone();
                        }
                    }
                    for statement in implementation {
                        match statement.run(closure_context) {
                            JsStatementResult::ReturnValue(value) => return value,
                            result => closure_context.record_new_result(result),
                        }
                    }
                    JsValue::undefined_rc(&closure_context.nodes_graph) // TODO: implement return
                })
            }
        }
    }

    pub fn get_referenced_nodes(&self) -> Vec<JsValueNode> {
        match self {
            Self::Native(_, _) => vec![],
            Self::UserDefined(_source, _name, _params, statements) => statements
                .iter()
                .flat_map(|statement| statement.get_referenced_nodes())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_name() {
        assert_eq!(
            "abc",
            JsFunction::Native("abc".to_string(), JsNativeFunctionImplementation::default())
                .get_name()
        );
    }

    #[test]
    #[allow(clippy::eq_op)]
    fn test_partial_eq() {
        assert_ne!(
            JsNativeFunctionImplementation::default(),
            JsNativeFunctionImplementation::default(),
        );

        let a = JsNativeFunctionImplementation::default();
        assert_ne!(a, a);
    }
}
