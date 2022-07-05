use std::{
    cell::RefCell,
    ops::DerefMut,
    rc::{Rc, Weak},
};

use super::{inner_node::InnerNode, GarbageCollectable, GcNodeGraph};

/// Represents an opaque node that can be garbage collected
pub struct GcNode<T: GarbageCollectable> {
    node: Weak<RefCell<InnerNode<T>>>,
    pub(super) node_graph: Weak<RefCell<GcNodeGraph<T>>>,
}

impl<T: GarbageCollectable> GcNode<T> {
    pub fn map_value<F, U>(&self, map: F) -> Option<U>
    where
        F: FnOnce(&T) -> U,
    {
        let ref_cell = self.node.upgrade()?;
        let value = &ref_cell.borrow().value;
        Some(map(value.as_ref()))
    }

    pub fn with_value<F>(&self, map: F)
    where
        F: FnOnce(&T),
    {
        self.map_value(map);
    }

    pub fn with_mut<F>(&mut self, func: F)
    where
        F: FnOnce(&mut T),
    {
        self.with_mut_node(|node| func(&mut node.value))
    }

    /// Returns `true` if the contents of this node still exists and is retrievable. This returns `false` when the underlying data has been garbage collected but this dangling reference is still hanging around.
    pub fn exists(&self) -> bool {
        self.node.strong_count() > 0
    }

    pub fn with_node_graph<F>(&mut self, func: F)
    where
        F: FnOnce(&Rc<RefCell<GcNodeGraph<T>>>),
    {
        let node_graph_rc = self.node_graph.upgrade().expect("Graph not present");
        func(&node_graph_rc);
    }

    pub fn create_new_node(&self, new_node_value: T) -> GcNode<T> {
        let node_graph_rc = self.node_graph.upgrade().expect("Graph not present");
        GcNodeGraph::create_node(&node_graph_rc, new_node_value)
    }
}

impl<T: GarbageCollectable> Clone for GcNode<T> {
    fn clone(&self) -> GcNode<T> {
        GcNode {
            node: self.node.clone(),
            node_graph: self.node_graph.clone(),
        }
    }
}

/// Internal details usable by others in this module
impl<T: GarbageCollectable> GcNode<T> {
    pub(super) fn new(
        node: Weak<RefCell<InnerNode<T>>>,
        node_graph: Weak<RefCell<GcNodeGraph<T>>>,
    ) -> Self {
        Self { node, node_graph }
    }

    #[allow(dead_code)]
    pub(super) fn with_node<F>(&self, func: F)
    where
        F: FnOnce(&InnerNode<T>),
    {
        if let Some(ref_cell) = self.node.upgrade() {
            let value = &ref_cell.borrow();
            func(value);
        }
    }

    pub(super) fn with_mut_node<F>(&mut self, func: F)
    where
        F: FnOnce(&mut InnerNode<T>),
    {
        if let Some(rc) = self.node.upgrade() {
            let mut ref_mut = (&*rc).borrow_mut();
            func(ref_mut.deref_mut());
        }
    }
}
