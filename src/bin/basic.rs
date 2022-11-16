use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct ReturnsNumber {
    number: i32,
}

#[tokio::main]
async fn main() {
    let future = ReturnsNumber { number: 42 };
    let number = future.await;
    println!("{number}");
}
