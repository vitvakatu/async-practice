use std::{
    future::Future,
    task::Poll,
    time::{Duration, Instant},
};

use futures::future::BoxFuture;

#[pin_project::pin_project]
struct TimedFuture<F> {
    started: Option<Instant>,
    #[pin]
    fut: F,
}

impl<F> TimedFuture<F> {
    fn new(fut: F) -> Self {
        Self { fut, started: None }
    }
}

impl<F: Future> Future for TimedFuture<F> {
    type Output = (F::Output, Duration);

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        if this.started.is_none() {
            *this.started = Some(Instant::now());
        }
        // Pin<&mut F>
        match this.fut.poll(cx) {
            Poll::Ready(res) => Poll::Ready((res, this.started.unwrap().elapsed())),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[tokio::main]
async fn main() {
    let f = reqwest::get("https://rust-lang.org");
    let time_it = TimedFuture::new(f);
    let (_result, time_spent) = time_it.await;
    println!("Request took {} ms", time_spent.as_millis());
}
