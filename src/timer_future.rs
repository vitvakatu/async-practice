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
            started: false,
        }));
        Self { state, duration }
    }
}

struct SharedState {
    completed: bool,
    started: bool,
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Poll");
        let state_clone = self.state.clone();
        let mut state = self.state.lock().unwrap();
        let duration = self.duration;
        let waker = cx.waker().clone();

        if !state.started {
            thread::spawn(move || {
                thread::sleep(duration);
                let mut state = state_clone.lock().unwrap();
                state.completed = true;
                waker.wake();
            });
            state.started = true;
        }

        if state.completed {
            println!("Ready");
            Poll::Ready(())
        } else {
            println!("Pending");
            Poll::Pending
        }
    }
}
