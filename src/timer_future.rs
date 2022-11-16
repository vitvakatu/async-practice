use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

pub struct TimerFuture {}

impl TimerFuture {
    pub fn new(duration: Duration) -> impl Future<Output = ()> {
        tokio::time::sleep(duration)
    }
}
