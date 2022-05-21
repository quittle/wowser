use std::{cell::RefCell, rc::Rc};

use super::{gc_sweep_id::GcSweepId, inner_node::InnerNode, GarbageCollectable, Node};

/// A graph of nodes that can be garbage collected
pub struct NodeGraph<T: GarbageCollectable> {
    nodes: Vec<Rc<RefCell<InnerNode<T>>>>,
    root: Rc<RefCell<InnerNode<T>>>,
    gc_sweep_id: GcSweepId,
}

impl<T: GarbageCollectable> NodeGraph<T> {
    /// Creates a new graph
    /// `root_value` - The root value which connects to all other nodes.
    pub fn new(root_value: T) -> (Self, Node<T>) {
        let strong_node = Rc::new(RefCell::new(InnerNode::new(root_value)));
        let root_node = Node::new(Rc::downgrade(&strong_node));
        (
            NodeGraph {
                nodes: vec![],
                root: strong_node,
                gc_sweep_id: 0,
            },
            root_node,
        )
    }

    /// The only way to create a node in this graph.
    pub fn create_node(&mut self, value: T) -> Node<T> {
        let node = InnerNode::new(value);
        let strong_node = Rc::new(RefCell::new(node));
        let weak_node = Rc::downgrade(&strong_node);
        self.nodes.push(strong_node);
        Node::new(weak_node)
    }

    /// Runs the garbage collector, freeing up non-root nodes using a mark-and-sweep algorithm.
    pub fn gc(&mut self) {
        self.gc_sweep_id += 1;
        let gc_sweep_id = self.gc_sweep_id;
        Self::mark_nodes(&self.root, gc_sweep_id);
        self.nodes = self
            .nodes
            .iter()
            .filter(|node| node.borrow().gc_sweep_id == gc_sweep_id)
            .map(Rc::clone)
            .collect();
    }

    /// Returns how many nodes are stored in the graph.
    pub fn size(&self) -> usize {
        // Add one to include the root node
        self.nodes.len() + 1
    }

    /// Recurses through the nodes, finding everything reachable from the root node and updating the
    /// sweep id.
    fn mark_nodes(root_node: &Rc<RefCell<InnerNode<T>>>, new_gc_sweep_id: GcSweepId) {
        let mut queue: Vec<Node<T>> = vec![Node::new(Rc::downgrade(root_node))];

        while let Some(mut node) = queue.pop() {
            node.with_mut_node(|node| {
                // Return early if this node was already visited to avoid cycles
                if node.gc_sweep_id == new_gc_sweep_id {
                    return;
                }

                node.gc_sweep_id = new_gc_sweep_id;

                queue.extend(node.value.get_referenced_nodes());
            });
        }
    }
}
