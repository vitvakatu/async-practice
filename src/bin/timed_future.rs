use futures::future::BoxFuture;
use futures::FutureExt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use std::time::Instant;

struct TimedFuture<T> {
    inner: BoxFuture<'static, T>,
    start: Option<Instant>,
}

impl<T> TimedFuture<T> {
    fn new(future: impl Future<Output = T> + 'static + Send) -> Self {
        Self {
            inner: future.boxed(),
            start: None,
        }
    }
}

impl<T> Future for TimedFuture<T> {
    type Output = (T, Duration);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if self.start.is_none() {
            self.start = Some(Instant::now());
        }
        match self.inner.as_mut().poll(cx) {
            Poll::Ready(value) => {
                let elapsed = self.start.unwrap().elapsed();
                Poll::Ready((value, elapsed))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[tokio::main]
async fn main() {
    // Хранить футуру лучше в виде трейт-объекта BoxFuture
    // Сконструировать можно через future.boxed() для любого future: Future

    // Вот так измерять время:
    // let now = Instant::now();
    // let elapsed = now.elapsed();

    // let mut self = self;
    // let this = &mut *self;

    // new(f: impl Future<Output=T> + Send + 'static) -> Self

    // Как заполить футуру:
    // self.inner_future.as_mut().poll(context)

    let request = reqwest::get("https://www.rust-lang.org");
    let (_response, elapsed) = TimedFuture::new(request).await;
    println!("Request took {} ms", elapsed.as_millis(),);
}
