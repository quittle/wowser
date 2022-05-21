use super::Node;

/// An element that can be garbage collected or reference other garbage collectable nodes.
pub trait GarbageCollectable: Sized {
    fn get_referenced_nodes(&self) -> Vec<Node<Self>>;
}
