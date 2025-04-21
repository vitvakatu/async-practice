use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct ReturnsNumber {
    number: i32,
}

impl Future for ReturnsNumber {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.number)
    }
}

#[tokio::main]
async fn main() {
    let future = ReturnsNumber { number: 42 };
    let number: i32 = future.await;
    println!("{number}");
    assert_eq!(number, 42);
}
