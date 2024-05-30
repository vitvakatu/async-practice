use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct ReturnsNumber {}

#[tokio::main]
async fn main() {
    let future = ReturnsNumber { number: 42 };
    let number: i32 = future.await;
    println!("{number}");
    assert_eq!(number, 42);
}
