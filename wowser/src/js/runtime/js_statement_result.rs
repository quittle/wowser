use std::collections::HashMap;

use super::{JsValue, JsValueGraph, JsValueNode};

/// Represents the resulting value of evaluating a statement
#[derive(Debug, PartialEq)]
pub enum JsStatementResult {
    Value(JsValueNode),
    ReturnValue(JsValueNode),
    Void,
}

impl JsStatementResult {
    pub fn bool(node_graph: &JsValueGraph, b: bool) -> Self {
        Self::Value(JsValue::bool_rc(node_graph, b))
    }

    pub fn number<F>(node_graph: &JsValueGraph, v: F) -> Self
    where
        F: Into<f64>,
    {
        Self::Value(JsValue::number_rc(node_graph, v.into()))
    }

    pub fn nan(node_graph: &JsValueGraph) -> Self {
        Self::Value(JsValue::nan_rc(node_graph))
    }

    pub fn string<S>(node_graph: &JsValueGraph, string: S) -> Self
    where
        S: Into<String>,
    {
        Self::Value(JsValue::string_rc(node_graph, string.into()))
    }

    pub fn undefined(node_graph: &JsValueGraph) -> Self {
        Self::Value(JsValue::undefined_rc(node_graph))
    }

    pub fn null(node_graph: &JsValueGraph) -> Self {
        Self::Value(JsValue::null_rc(node_graph))
    }

    pub fn object(node_graph: &JsValueGraph, object: Vec<(&str, JsValueNode)>) -> Self {
        let mut map = HashMap::with_capacity(object.len());
        for (key, value) in object {
            map.insert(key.to_string(), value);
        }
        Self::Value(JsValue::object_rc(node_graph, map))
    }

    pub fn is_nan(&self) -> bool {
        match self {
            JsStatementResult::Value(value) => value.map_value(|value| match value {
                JsValue::Number(n) => n.is_nan(),
                _ => false,
            }),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::JsStatementResult;
    use crate::{
        garbage_collector::GcNodeGraph,
        js::{JsValue, JsValueGraph},
    };

    fn get_node_graph() -> JsValueGraph {
        let (node_graph, _root) = GcNodeGraph::new(JsValue::Undefined);
        node_graph
    }

    #[test]
    fn test_nan() {
        let node_graph = get_node_graph();
        match JsStatementResult::nan(&node_graph) {
            JsStatementResult::Value(v) => v.with_value(|v| match v {
                JsValue::Number(n) => assert!(n.is_nan()),
                v => panic!("Invalid value type: {:?}", v),
            }),
            v => panic!("Invalid result value: {:?}", v),
        }
    }

    #[test]
    fn test_undefined() {
        let node_graph = get_node_graph();
        match JsStatementResult::undefined(&node_graph) {
            JsStatementResult::Value(v) => v.with_value(|v| match v {
                JsValue::Undefined => {}
                v => panic!("Invalid value type: {:?}", v),
            }),
            v => panic!("Invalid result value: {:?}", v),
        }
    }

    #[test]
    fn test_number() {
        let node_graph = get_node_graph();
        match JsStatementResult::number(&node_graph, 123.0) {
            JsStatementResult::Value(v) => v.with_value(|v| match v {
                JsValue::Number(v) => assert_eq!(*v, 123.0),
                v => panic!("Invalid value type: {:?}", v),
            }),
            v => panic!("Invalid result value: {:?}", v),
        }
    }

    #[test]
    fn test_string() {
        let node_graph = get_node_graph();
        assert_eq!(
            JsStatementResult::string(&node_graph, "123"),
            JsStatementResult::Value(JsValue::string_rc(&node_graph, String::from("123")))
        );
        assert_eq!(
            JsStatementResult::string(&node_graph, String::from("abc")),
            JsStatementResult::Value(JsValue::string_rc(&node_graph, String::from("abc")))
        );
    }

    #[test]
    fn test_is_nan() {
        let node_graph = get_node_graph();
        assert!(JsStatementResult::nan(&node_graph,).is_nan());

        assert!(!JsStatementResult::string(&node_graph, "123").is_nan());
        assert!(!JsStatementResult::number(&node_graph, 1.0).is_nan());
        assert!(!JsStatementResult::undefined(&node_graph,).is_nan());
    }
}
