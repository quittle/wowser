use std::{
    cell::RefCell,
    ops::DerefMut,
    rc::{Rc, Weak},
};

use super::{inner_node::InnerNode, GarbageCollectable, GcNodeGraph};

/// Represents an opaque node that can be garbage collected
#[derive(Debug)]
pub struct GcNode<T: GarbageCollectable> {
    node: Weak<RefCell<InnerNode<T>>>,
    pub(super) node_graph: Weak<RefCell<GcNodeGraph<T>>>,
}

impl<T: GarbageCollectable> GcNode<T> {
    pub fn map_value<F, U>(&self, map: F) -> U
    where
        F: FnOnce(&T) -> U,
    {
        let rc = self.node.upgrade().expect("Failed to load a node.");

        let inner_node_ref = (&*rc).borrow();
        map(&lazy_static::__Deref::deref(&inner_node_ref).value)
    }

    pub fn with_value<F>(&self, func: F)
    where
        F: FnOnce(&T),
    {
        self.map_value(func);
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

    pub fn get_node_graph(&self) -> Rc<RefCell<GcNodeGraph<T>>> {
        self.node_graph.upgrade().expect("Graph not present")
    }

    pub fn with_node_graph<F, O>(&self, func: F) -> O
    where
        F: FnOnce(&Rc<RefCell<GcNodeGraph<T>>>) -> O,
    {
        func(&self.get_node_graph())
    }

    pub fn create_new_node(&self, new_node_value: T) -> GcNode<T> {
        GcNodeGraph::create_node(&self.get_node_graph(), new_node_value)
    }

    /// This unsafely gets an immutable reference to the internal data, tied to the lifetime of the node.
    /// TODO: Revisit if there's a better way
    pub fn get_ref(&self) -> &T {
        // This block ensures that all the regular memory checks pass for getting the reference to increase safety.
        // This DOES NOT ensure it will be safe to use the return value if a mutable reference gets created later.
        {
            let rc = &self
                .node
                .upgrade()
                .expect("Attempting to get reference of garbage collected node");

            let _ignore_borrowed_value = &rc.borrow().value;
        }
        let ptr = self.node.as_ptr();
        let maybe_ref = unsafe { ptr.as_ref() };
        let reference = maybe_ref.unwrap();
        let refcell_ptr = reference.as_ptr();
        let inner_ref = unsafe { refcell_ptr.as_ref() }.unwrap();
        &inner_ref.value
    }

    pub fn is_same_ref(&self, other: &Self) -> bool {
        self.map_value(|self_inner| {
            other.map_value(|other_inner| {
                let self_ptr: *const T = self_inner;
                let other_ptr: *const T = other_inner;
                self_ptr == other_ptr
            })
        })
    }
}

impl<T: GarbageCollectable> PartialEq for GcNode<T> {
    fn eq(&self, other: &Self) -> bool {
        let my_node = self.node.upgrade();
        let other_node = other.node.upgrade();

        my_node == other_node
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
