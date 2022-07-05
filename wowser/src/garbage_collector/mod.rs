mod garbage_collectable;
mod gc_node;
mod gc_node_graph;
mod gc_sweep_id;
mod inner_node;

pub use garbage_collectable::GarbageCollectable;
pub use gc_node::GcNode;
pub use gc_node_graph::GcNodeGraph;

#[cfg(test)]
mod tests {
    use super::{GarbageCollectable, GcNode, GcNodeGraph};

    struct TestTree {
        value: u8,
        left: Option<GcNode<TestTree>>,
        right: Option<GcNode<TestTree>>,
    }

    impl GarbageCollectable for TestTree {
        fn get_referenced_nodes(&self) -> Vec<GcNode<TestTree>> {
            self.left
                .iter()
                .chain(self.right.iter())
                .map(Clone::clone)
                .collect()
        }
    }

    impl TestTree {
        pub fn new(value: u8) -> Self {
            TestTree {
                value,
                left: None,
                right: None,
            }
        }

        pub fn sum(&self) -> u8 {
            self.value
                + self
                    .left
                    .as_ref()
                    .and_then(|wrcn| wrcn.map_value(|tree| tree.value))
                    .unwrap_or(0)
                + self
                    .right
                    .as_ref()
                    .and_then(|wrcn| wrcn.map_value(|tree| tree.value))
                    .unwrap_or(0)
        }
    }

    #[test]
    fn test_empty() {
        let (graph, _root) = GcNodeGraph::<TestTree>::new(TestTree::new(u8::MAX));
        assert_eq!(graph.borrow().size(), 1);
        GcNodeGraph::gc(&graph);
        assert_eq!(graph.borrow().size(), 1);
    }

    #[test]
    fn test_single_node() {
        let (graph, mut root) = GcNodeGraph::<TestTree>::new(TestTree::new(1));
        assert_eq!(graph.borrow().size(), 1);

        let child = GcNodeGraph::create_node(&graph, TestTree::new(2));
        root.with_mut(|root| root.left = Some(child));
        assert_eq!(graph.borrow().size(), 2);
        assert_eq!(root.map_value(|root| root.sum()), Some(3));

        GcNodeGraph::gc(&graph);
        assert_eq!(graph.borrow().size(), 2);

        root.with_mut(|root| root.left = None);
        assert_eq!(graph.borrow().size(), 2);

        GcNodeGraph::gc(&graph);
        assert_eq!(graph.borrow().size(), 1);
    }

    #[test]
    fn test_cycle() {
        let (graph, mut root) = GcNodeGraph::<TestTree>::new(TestTree::new(1));
        assert_eq!(graph.borrow().size(), 1);

        let mut node1 = GcNodeGraph::create_node(&graph, TestTree::new(2));
        let mut node2 = GcNodeGraph::create_node(&graph, TestTree::new(3));

        root.with_mut(|root| root.left = Some(node1.clone()));

        // Clone outside for the borrow checker
        let node1_clone = node1.clone();
        node1.with_mut(|node| {
            node.left = Some(node2.clone());
            node.right = Some(node1_clone);
        });

        node2.with_mut(|node| node.left = Some(node1.clone()));
        assert_eq!(graph.borrow().size(), 3);

        root.with_value(|root| {
            assert_eq!(root.value, 1, "Root Node");
            root.left.as_ref().unwrap().with_value(|left| {
                assert_eq!(left.value, 2, "Node 1");
                left.left.as_ref().unwrap().with_value(|left_left| {
                    assert_eq!(left_left.value, 3, "Node 2");
                    left_left
                        .left
                        .as_ref()
                        .unwrap()
                        .with_value(|left_left_left| {
                            assert_eq!(left_left_left.value, 2, "Cyclically links back to Node 1");
                        });
                    assert!(left_left.right.is_none(), "Node 2 has no right child");
                });
                left.right.as_ref().unwrap().with_value(|left_right| {
                    assert_eq!(left_right.value, 2, "Node 1 cylically links back to Node 1");
                });
            });
            assert!(root.right.is_none(), "No right child of root");
        });

        GcNodeGraph::gc(&graph);
        assert_eq!(graph.borrow().size(), 3);

        // Remove the only reference to node 2
        node1.with_mut_node(|node| node.value.left = None);
        assert_eq!(graph.borrow().size(), 3);
        assert!(node2.exists());
        GcNodeGraph::gc(&graph);
        assert_eq!(graph.borrow().size(), 2);
        assert!(!node2.exists());

        // Remove the root reference to node 1
        root.with_mut_node(|root| root.value.left = None);
        assert_eq!(graph.borrow().size(), 2);
        assert!(node1.exists());
        GcNodeGraph::gc(&graph);
        assert_eq!(graph.borrow().size(), 1);
        assert!(!node1.exists());

        assert!(root.exists());
    }

    #[test]
    fn test_create_new_node() {
        let (graph, mut root) = GcNodeGraph::<TestTree>::new(TestTree::new(1));
        assert_eq!(
            Some(1),
            root.map_value(|tree| tree.sum()),
            "Just root value"
        );

        let child = root.create_new_node(TestTree::new(2));
        assert_eq!(
            Some(1),
            root.map_value(|tree| tree.sum()),
            "Child should not be associated with root value"
        );

        root.with_mut(|tree| tree.left = Some(child));
        // Just to prove the connection
        GcNodeGraph::gc(&graph);

        assert_eq!(Some(3), root.map_value(|tree| tree.sum()));
    }

    #[test]
    fn test_garbage_collection() {
        let (graph, mut root) = GcNodeGraph::<TestTree>::new(TestTree::new(100));
        let left_child = GcNodeGraph::create_node(&graph, TestTree::new(20));
        let right_child = GcNodeGraph::create_node(&graph, TestTree::new(3));

        root.with_mut_node(|node| {
            node.gc_sweep_id = 123;
            node.value.left = Some(left_child);
            node.value.right = Some(right_child);

            assert_eq!(node.value.sum(), 123);
            assert_eq!(graph.borrow().size(), 3);
        });

        GcNodeGraph::gc(&graph);

        root.with_mut_node(|node| {
            assert_eq!(node.value.sum(), 123);
            assert_eq!(graph.borrow().size(), 3);

            node.value.left = None;

            assert_eq!(node.value.sum(), 103);
            assert_eq!(
                graph.borrow().size(),
                3,
                "Should not be garbage collected yet"
            );
        });

        GcNodeGraph::gc(&graph);

        root.with_node(|node| {
            assert_eq!(node.value.sum(), 103);
            assert_eq!(graph.borrow().size(), 2);
        });
    }
}
