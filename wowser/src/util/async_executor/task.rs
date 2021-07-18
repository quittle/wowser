use {
    futures::{future::BoxFuture, task::ArcWake},
    std::{
        sync::mpsc::Sender,
        sync::{Arc, Mutex},
    },
};

pub type TaskToken = u64;

/// A future that can reschedule itself to be polled by an `AsyncExecutor`.
pub struct Task<T> {
    /// In-progress future that should be pushed to completion.
    pub future: Mutex<Option<BoxFuture<'static, T>>>,

    /// Handle to place the task itself back onto the task queue.
    pub task_sender: Sender<Arc<Task<T>>>,

    /// An identifier for the task, passed along as it continues to be executed
    pub token: TaskToken,
}

unsafe impl<T> Sync for Task<T> {}

unsafe impl<T> Send for Task<T> {}

impl<T> ArcWake for Task<T> {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).unwrap();
    }
}
