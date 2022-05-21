use std::{cell::RefCell, rc::Weak};

use super::{inner_node::InnerNode, GarbageCollectable};

/// Represents an opaque node that can be garbage collected
pub struct Node<T: GarbageCollectable> {
    node: Weak<RefCell<InnerNode<T>>>,
}

impl<T: GarbageCollectable> Node<T> {
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
}

impl<T: GarbageCollectable> Clone for Node<T> {
    fn clone(&self) -> Node<T> {
        Node {
            node: self.node.clone(),
        }
    }
}

/// Internal details usable by others in this module
impl<T: GarbageCollectable> Node<T> {
    pub(super) fn new(node: Weak<RefCell<InnerNode<T>>>) -> Self {
        Self { node }
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
        if let Some(ref_cell) = self.node.upgrade() {
            let value = &mut ref_cell.borrow_mut();
            func(value);
        }
    }
}
