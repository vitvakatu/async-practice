use futures::future::BoxFuture;
use futures::FutureExt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[tokio::main]
async fn main() {
    let request = reqwest::get("https://www.rust-lang.org");
    let (_response, elapsed) = TimedFuture::new(request).await;
    println!("Request took {} ms", elapsed.as_millis(),);
}
