use super::{
    gc_sweep_id::{GcSweepId, DEFAULT_GC_SWEEP_ID},
    GarbageCollectable,
};

/// Not exposed outside the garbage collector, this holds all the metadata needed for managing the node.
pub(super) struct InnerNode<T: GarbageCollectable> {
    pub value: Box<T>,
    pub gc_sweep_id: GcSweepId,
}

impl<T: GarbageCollectable> InnerNode<T> {
    pub fn new(value: T) -> Self {
        InnerNode {
            value: Box::new(value),
            gc_sweep_id: DEFAULT_GC_SWEEP_ID,
        }
    }
}
