use std::panic::{catch_unwind, AssertUnwindSafe};

use super::{
    execution_error::ExecutionError,
    executor::Executor,
    spawner::Spawner,
    task::{Task, TaskToken},
};

use {
    futures::task::waker_ref,
    std::{
        collections::HashMap,
        future::Future,
        sync::mpsc::Receiver,
        sync::{Arc, Mutex},
        task::{Context, Poll},
    },
};

type ResultMap<T> = Arc<Mutex<HashMap<TaskToken, Result<T, ExecutionError>>>>;

fn spawn_thread_and_run<T>(task_receiver: Receiver<Arc<Task<T>>>, result_map: ResultMap<T>)
where
    T: 'static + Send,
{
    std::thread::Builder::new()
        .name("DedicatedThreadExecutor".into())
        .spawn(move || {
            while let Ok(task) = task_receiver.recv() {
                // Take the future, and if it has not yet completed (is still Some),
                // poll it in an attempt to complete it.
                if let Ok(mut future_slot) = task.future.lock() {
                    if let Some(mut future) = future_slot.take() {
                        // Create a `LocalWaker` from the task itself
                        let waker = waker_ref(&task);
                        let context = &mut Context::from_waker(&waker);
                        // `BoxFuture<T>` is a type alias for
                        // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
                        // We can get a `Pin<&mut dyn Future + Send + 'static>`
                        // from it by calling the `Pin::as_mut` method.
                        let poll = catch_unwind(AssertUnwindSafe(|| future.as_mut().poll(context)));
                        match poll {
                            Err(err) => {
                                let execution_error = ExecutionError(format!("{:?}", err));
                                result_map
                                    .lock()
                                    .unwrap()
                                    .insert(task.token, Err(execution_error));
                            }
                            Ok(Poll::Pending) => {
                                // We're not done processing the future, so put it
                                // back in its task to be run again in the future.
                                // It's unclear how this works properly.
                                *future_slot = Some(future);
                            }
                            Ok(Poll::Ready(result)) => {
                                result_map.lock().unwrap().insert(task.token, Ok(result));
                            }
                        }
                    }
                }
            }
        })
        .unwrap();
}

pub struct DedicatedThreadExecutor<T: Send> {
    spawner: Spawner<T>,
    last_token: TaskToken,
    result_map: ResultMap<T>,
}

impl<T> Executor for DedicatedThreadExecutor<T>
where
    T: 'static + Send,
{
    type Output = T;

    fn run(
        &mut self,
        future: impl Future<Output = T> + 'static + Send,
    ) -> Result<TaskToken, ExecutionError> {
        let token = self.last_token;
        self.last_token += 1;
        self.spawner
            .spawn(future, token)
            .map_err(|err| ExecutionError(err.to_string()))?;
        Ok(token)
    }

    fn get_result(&self, token: TaskToken) -> Option<Result<T, ExecutionError>> {
        self.result_map.lock().unwrap().remove(&token)
    }
}

impl<T> Default for DedicatedThreadExecutor<T>
where
    T: 'static + Send,
{
    fn default() -> DedicatedThreadExecutor<T> {
        let (spawner, task_receiver) = Spawner::new();
        let result_map = Arc::new(Mutex::new(HashMap::new()));
        spawn_thread_and_run(task_receiver, result_map.clone());
        DedicatedThreadExecutor {
            spawner,
            last_token: 0,
            result_map,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    const SLEEP_DURATION: Duration = Duration::from_millis(10);

    #[test]
    pub fn test_async_method() {
        let mut executor: DedicatedThreadExecutor<u8> = DedicatedThreadExecutor::default();
        let token = executor
            .run(async {
                thread::sleep(SLEEP_DURATION);
                123
            })
            .unwrap();
        assert!(executor.get_result(token).is_none());
        thread::sleep(SLEEP_DURATION * 2);
        assert_eq!(123, executor.get_result(token).unwrap().unwrap());
        assert!(executor.get_result(token).is_none());
    }

    #[test]
    pub fn test_panic_in_thread() {
        let mut executor: DedicatedThreadExecutor<u8> = DedicatedThreadExecutor::default();
        let token = executor
            .run(async {
                thread::sleep(SLEEP_DURATION);

                panic!("Panic during execution");
            })
            .unwrap();
        assert!(executor.get_result(token).is_none());
        // This needs to be elongated due to the slowness of panic'ing on slow machines (like in GitHub Actions)
        thread::sleep(SLEEP_DURATION * 50);
        let result = executor.get_result(token);
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
        assert!(executor.get_result(token).is_none());
    }
}
