use std::collections::HashMap;

use crate::garbage_collector::{GarbageCollectable, GcNode, GcNodeGraph};

use super::{JsFunction, JsValueGraph, JsValueNode};

pub type JsNumberPrimitive = f64;

/// Represents any type
#[derive(Debug, PartialEq)]
pub enum JsValue {
    Boolean(bool),
    Number(JsNumberPrimitive),
    String(String),
    Function(JsFunction),
    Object(HashMap<String, JsValueNode>),
    Undefined,
    Null,
}

impl JsValue {
    pub const NAN: Self = Self::Number(JsNumberPrimitive::NAN);

    pub fn bool_rc(node_graph: &JsValueGraph, b: bool) -> GcNode<Self> {
        GcNodeGraph::create_node(node_graph, Self::Boolean(b))
    }

    pub fn nan_rc(node_graph: &JsValueGraph) -> GcNode<Self> {
        GcNodeGraph::create_node(node_graph, Self::NAN)
    }

    pub fn str(s: &str) -> Self {
        JsValue::String(s.into())
    }

    pub fn str_rc(node_graph: &JsValueGraph, s: &str) -> GcNode<Self> {
        GcNodeGraph::create_node(node_graph, Self::str(s))
    }

    pub fn string_rc(node_graph: &JsValueGraph, s: String) -> GcNode<Self> {
        GcNodeGraph::create_node(node_graph, Self::String(s))
    }

    pub fn number_rc<F>(node_graph: &JsValueGraph, value: F) -> GcNode<Self>
    where
        F: Into<JsNumberPrimitive>,
    {
        GcNodeGraph::create_node(node_graph, Self::Number(value.into()))
    }

    pub fn undefined_rc(node_graph: &JsValueGraph) -> GcNode<Self> {
        GcNodeGraph::create_node(node_graph, Self::Undefined)
    }

    pub fn null_rc(node_graph: &JsValueGraph) -> GcNode<Self> {
        GcNodeGraph::create_node(node_graph, Self::Null)
    }

    pub fn object_rc(node_graph: &JsValueGraph, map: HashMap<String, JsValueNode>) -> GcNode<Self> {
        GcNodeGraph::create_node(node_graph, Self::Object(map))
    }

    pub fn type_error_rc(node_graph: &JsValueGraph) -> GcNode<Self> {
        Self::string_rc(node_graph, "TypeError".to_string()) // TODO: These should raise exceptions when supported
    }

    pub fn type_error_or_dom_exception_rc(node_graph: &JsValueGraph) -> GcNode<Self> {
        Self::string_rc(node_graph, "DomException".to_string()) // TODO: These should raise exceptions when supported
    }

    pub fn reference_error_rc(node_graph: &JsValueGraph) -> GcNode<Self> {
        Self::string_rc(node_graph, "ReferenceError".to_string()) // TODO: These should raise exceptions when supported
    }

    pub fn stack_overflow_error_rc(node_graph: &JsValueGraph) -> GcNode<Self> {
        Self::string_rc(node_graph, "StackOverflowError".to_string()) // TODO: This should raise RangeError: Maximum call stack size exceeded when supported
    }
}

impl ToString for JsValue {
    fn to_string(&self) -> String {
        match self {
            Self::Boolean(b) => b.to_string(),
            Self::Number(v) => v.to_string(),
            Self::String(v) => v.clone(),
            Self::Undefined => "undefined".to_string(),
            Self::Null => "null".to_string(),
            Self::Function(function) => match function {
                JsFunction::Native(name, _implementation) => {
                    format!("function {name}() {{ [native code] }}")
                }
                JsFunction::UserDefined(source, _name, _args, _implementation) => {
                    source.to_string()
                }
            },
            Self::Object(_) => "[object Object]".to_string(),
        }
    }
}

impl From<JsValue> for JsNumberPrimitive {
    fn from(value: JsValue) -> JsNumberPrimitive {
        From::from(&value)
    }
}

impl From<&JsValue> for JsNumberPrimitive {
    fn from(value: &JsValue) -> JsNumberPrimitive {
        match value {
            JsValue::Boolean(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            JsValue::Number(v) => *v,
            JsValue::String(v) => {
                let trimmed = v.trim();
                // Strings with just whitespace convert to 0
                if trimmed.is_empty() {
                    0.0
                } else {
                    trimmed
                        .parse::<JsNumberPrimitive>()
                        .unwrap_or(JsNumberPrimitive::NAN)
                }
            }
            JsValue::Undefined => JsNumberPrimitive::NAN,
            JsValue::Null => 0.0,
            JsValue::Function(_) => JsNumberPrimitive::NAN,
            JsValue::Object(_) => JsNumberPrimitive::NAN,
        }
    }
}

impl From<&JsValue> for bool {
    fn from(value: &JsValue) -> bool {
        match value {
            JsValue::Boolean(v) => *v,
            JsValue::Number(v) => !v.is_nan() && *v != 0.0,
            JsValue::String(v) => !v.is_empty(),
            JsValue::Undefined => false,
            JsValue::Null => false,
            JsValue::Function(_) => true,
            JsValue::Object(_) => true,
        }
    }
}

impl GarbageCollectable for JsValue {
    fn get_referenced_nodes(&self) -> Vec<GcNode<Self>> {
        match self {
            JsValue::Boolean(_) => vec![],
            JsValue::Number(_) => vec![],
            JsValue::String(_) => vec![],
            JsValue::Undefined => vec![],
            JsValue::Null => vec![],
            JsValue::Function(function) => function.get_referenced_nodes(),
            JsValue::Object(map) => map.values().map(Clone::clone).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::js::JsNativeFunctionImplementation;

    use super::*;

    #[test]
    fn test_nan() {
        match JsValue::NAN {
            JsValue::Number(v) => assert!(v.is_nan()),
            v => panic!("Invalid value: {:?}", v),
        }
    }

    #[test]
    fn test_to_string() {
        let (node_graph, _root) = GcNodeGraph::new(JsValue::Undefined);
        assert_eq!(JsValue::NAN.to_string(), "NaN");
        assert_eq!(JsValue::Number(1.0).to_string(), "1");
        assert_eq!(JsValue::Number(1.2).to_string(), "1.2");
        assert_eq!(JsValue::Number(-1.2).to_string(), "-1.2");
        assert_eq!(JsValue::str("").to_string(), "");
        assert_eq!(JsValue::str("abc").to_string(), "abc");
        assert_eq!(JsValue::Undefined.to_string(), "undefined");
        assert_eq!(JsValue::Null.to_string(), "null");
        assert_eq!(
            JsValue::Function(JsFunction::Native(
                "abc".to_string(),
                JsNativeFunctionImplementation::default()
            ))
            .to_string(),
            "function abc() { [native code] }"
        );
        assert_eq!(
            JsValue::Function(JsFunction::UserDefined(
                "function abc(param) {return param;}".to_string(),
                Default::default(),
                Default::default(),
                Default::default()
            ))
            .to_string(),
            "function abc(param) {return param;}"
        );
        assert_eq!(
            JsValue::Object(HashMap::from([(
                "key".to_string(),
                JsValue::str_rc(&node_graph, "value")
            )]))
            .to_string(),
            "[object Object]"
        );
    }

    #[test]
    fn test_from_jsvalue_to_f64() {
        assert!(f64::from(JsValue::NAN).is_nan());
        assert_eq!(f64::from(JsValue::Number(1.0)), 1.0);
        assert_eq!(f64::from(JsValue::Number(1.2)), 1.2);
        assert_eq!(f64::from(JsValue::Number(-1.2)), -1.2);

        assert_eq!(f64::from(JsValue::str("")), 0.0);
        assert!(f64::from(JsValue::str("abc")).is_nan());
        assert_eq!(f64::from(JsValue::str("1")), (1.0));
        assert_eq!(f64::from(JsValue::str("-1")), (-1.0));
        assert_eq!(f64::from(JsValue::str("-1.2")), (-1.2));

        assert!(f64::from(JsValue::Undefined).is_nan());
        assert_eq!(f64::from(JsValue::Null), 0.0);

        assert!(f64::from(JsValue::Function(JsFunction::Native(
            "abc".to_string(),
            JsNativeFunctionImplementation::default(),
        )))
        .is_nan());
        assert!(f64::from(JsValue::Object(HashMap::new())).is_nan());
    }
}
