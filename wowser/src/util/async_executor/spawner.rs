use std::sync::mpsc::SendError;

use {
    super::task::{Task, TaskToken},
    futures::future::FutureExt,
    std::{
        future::Future,
        sync::{
            mpsc::{channel, Receiver, Sender},
            Arc, Mutex,
        },
    },
};

type ArcTask<T> = Arc<Task<T>>;

/// `Spawner` spawns new futures onto the task channel.
#[derive(Clone)]
pub struct Spawner<T: Send> {
    task_sender: Sender<ArcTask<T>>,
}

impl<T: Send> Spawner<T> {
    pub fn new() -> (Spawner<T>, Receiver<ArcTask<T>>) {
        let (task_sender, task_receiver) = channel();
        (Spawner { task_sender }, task_receiver)
    }

    pub fn spawn(
        &self,
        future: impl Future<Output = T> + 'static + Send,
        token: TaskToken,
    ) -> Result<(), SendError<ArcTask<T>>> {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            token,
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task)
    }
}
