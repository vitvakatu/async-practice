use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    state: Arc<Mutex<SharedState>>,
    duration: Duration,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));
        Self { state, duration }
    }
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Poll");
        let duration = self.duration;
        let state_clone = self.state.clone();
        let mut state = self.state.lock().unwrap();
        let should_run = state.waker.is_none();
        if should_run {
            thread::spawn(move || {
                thread::sleep(duration);
                let mut guard = state_clone.lock().unwrap();
                guard.completed = true;
                let waker = guard.waker.take();
                if let Some(waker) = waker {
                    waker.wake();
                }
            });
        }

        if state.completed {
            Poll::Ready(())
        } else {
            state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
