use std::pin::Pin;
use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::{Context, Poll},
        time::Duration,
    },
};

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let task_clone = arc_self.clone();
        arc_self
            .sender
            .send(task_clone)
            .expect("Too many async tasks")
    }
}

#[derive(Clone)]
pub struct Spawner {
    sender: SyncSender<Arc<Task>>,
}

pub fn create_executor() -> (Executor, Spawner) {
    let (sender, receiver) = sync_channel(1024);
    let executor = Executor { tasks: receiver };
    let spawner = Spawner { sender };
    (executor, spawner)
}

pub struct Executor {
    tasks: Receiver<Arc<Task>>,
}

impl Spawner {
    pub fn spawn<F: Future<Output = ()> + 'static + Send>(&self, future: F) {
        let task = Task {
            future: Mutex::new(Some(future.boxed())),
            sender: self.sender.clone(),
        };
        self.sender
            .send(Arc::new(task))
            .expect("Too many async tasks")
    }
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.tasks.recv() {
            let mut future_slot = task.future.lock().unwrap();
            let future = future_slot.take();
            if let Some(mut future) = future {
                let waker = waker_ref(&task);
                let mut context = Context::from_waker(&waker);

                match future.as_mut().poll(&mut context) {
                    Poll::Ready(_) => {}
                    Poll::Pending => {
                        *future_slot = Some(future);
                    }
                }
            }
        }
    }
}
