use super::{task::TaskToken, ExecutionError};
use std::future::Future;

/// Abstraction for an async task whose result can be queried for
pub trait Executor: Send {
    /// The end result of execution by the executor
    type Output: 'static + Send;

    /// Enqueue a future to be run asynchronously in some way, returning a token that be used to
    /// check on the progress
    fn run(
        &mut self,
        future: impl Future<Output = Self::Output> + 'static + Send,
    ) -> Result<TaskToken, ExecutionError>;

    /// Checks on the result of a future executed by the executor using the token returned
    /// upon enqueing. Returns None if the result is not ready or the result of the execution. The
    /// result will no longer be stored by the executor and will return None from then on.
    fn get_result(&self, token: TaskToken) -> Option<Result<Self::Output, ExecutionError>>;
}
