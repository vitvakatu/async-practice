use std::{
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {
    duration: Duration,
    ready: Arc<AtomicBool>,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        TimerFuture {
            duration,
            ready: Arc::new(AtomicBool::new(false)),
        }
    }
}

// thread

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Poll");
        if self.ready.load(Ordering::SeqCst) {
            return Poll::Ready(());
        }

        let duration = self.duration;
        let waker = cx.waker().clone();
        let ready_clone = self.ready.clone();
        std::thread::spawn(move || {
            thread::sleep(duration);
            ready_clone.store(true, Ordering::SeqCst);
            waker.wake();
        });
        Poll::Pending
    }
}
