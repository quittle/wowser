use std::{cell::RefCell, rc::Rc};

use crate::garbage_collector::{GcNode, GcNodeGraph};

use super::JsValue;

pub type JsValueGraph = Rc<RefCell<GcNodeGraph<JsValue>>>;
pub type JsValueNode = GcNode<JsValue>;
