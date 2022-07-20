use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use super::{gc_sweep_id::GcSweepId, inner_node::InnerNode, GarbageCollectable, GcNode};

/// A graph of nodes that can be garbage collected
pub struct GcNodeGraph<T: GarbageCollectable> {
    nodes: Vec<Rc<RefCell<InnerNode<T>>>>,
    root: Rc<RefCell<InnerNode<T>>>,
    gc_sweep_id: GcSweepId,
}

impl<T: GarbageCollectable> GcNodeGraph<T> {
    /// Creates a new graph
    ///
    /// `root_value` - The root value which connects to all other nodes.
    pub fn new(root_value: T) -> (Rc<RefCell<Self>>, GcNode<T>) {
        let strong_node = Rc::new(RefCell::new(InnerNode::new(root_value)));
        let mut root_node = GcNode::new(Rc::downgrade(&strong_node), Weak::new());
        let graph = Rc::new(RefCell::new(GcNodeGraph {
            nodes: vec![],
            root: strong_node,
            gc_sweep_id: 0,
        }));
        root_node.node_graph = Rc::downgrade(&graph);
        (graph, root_node)
    }

    /// The only way to create a node in this graph.
    pub fn create_node(node_graph: &Rc<RefCell<GcNodeGraph<T>>>, value: T) -> GcNode<T> {
        let node = InnerNode::new(value);
        let strong_node = Rc::new(RefCell::new(node));
        let weak_node = Rc::downgrade(&strong_node);
        node_graph.borrow_mut().nodes.push(strong_node);
        GcNode::new(weak_node, Rc::downgrade(node_graph))
    }

    /// Runs the garbage collector, freeing up non-root nodes using a mark-and-sweep algorithm.
    pub fn gc(node_graph: &Rc<RefCell<GcNodeGraph<T>>>) {
        let mut graph = node_graph.borrow_mut();
        graph.gc_sweep_id += 1;
        let gc_sweep_id = graph.gc_sweep_id;
        Self::mark_nodes(&graph.root, gc_sweep_id);
        graph.nodes = graph
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
        let mut queue: Vec<GcNode<T>> = vec![GcNode::new(Rc::downgrade(root_node), Weak::new())];

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

impl<T: GarbageCollectable> std::fmt::Debug for GcNodeGraph<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GcNodeGraph")
            .field(
                "nodes",
                &self
                    .nodes
                    .iter()
                    .map(|node| node.borrow())
                    .collect::<Vec<_>>(),
            )
            .field("root", &self.root)
            .field("gc_sweep_id", &self.gc_sweep_id)
            .finish()
    }
}
