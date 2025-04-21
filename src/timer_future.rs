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
    is_ready: Arc<AtomicBool>,
    is_started: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            is_ready: Arc::new(AtomicBool::new(false)),
            is_started: Arc::new(AtomicBool::new(false)),
            waker: Arc::new(Mutex::new(None)),
        }
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("POLL");
        let duration = self.duration;
        let is_ready_clone = Arc::clone(&self.is_ready);
        *self.waker.lock().unwrap() = Some(ctx.waker().clone());
        let waker_clone = Arc::clone(&self.waker);
        if !self.is_started.load(Ordering::Acquire) {
            self.is_started.store(true, Ordering::Release);
            thread::spawn(move || {
                thread::sleep(duration);
                is_ready_clone.store(true, Ordering::Release);
                if let Some(ref waker) = *waker_clone.lock().unwrap() {
                    waker.wake_by_ref();
                }
            });
        }
        if self.is_ready.load(Ordering::Acquire) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
