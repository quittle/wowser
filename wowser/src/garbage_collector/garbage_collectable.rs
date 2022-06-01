use std::fmt::Debug;

use super::GcNode;

/// An element that can be garbage collected or reference other garbage collectable nodes.
pub trait GarbageCollectable: Sized + PartialEq + Debug {
    fn get_referenced_nodes(&self) -> Vec<GcNode<Self>>;
}
