use futures::future::BoxFuture;
use futures::FutureExt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use std::time::Instant;

#[tokio::main]
async fn main() {
    // Хранить футуру лучше в виде трейт-объекта BoxFuture
    // Сконструировать можно через future.boxed() для любого future: Future

    // Вот так измерять время:
    let now = Instant::now();
    let elapsed = now.elapsed();

    // let mut self = self;
    // let this = &mut *self;

    // Как заполить футуру:
    // self.inner_future.as_mut().poll(context)

    let request = reqwest::get("https://www.rust-lang.org");
    let (_response, elapsed) = TimedFuture::new(request).await;
    println!("Request took {} ms", elapsed.as_millis(),);
}
