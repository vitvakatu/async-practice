use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct ReturnsNumber {
    number: i32,
}

impl Future for ReturnsNumber {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Println");
        Poll::Ready(())
    }
}

#[tokio::main]
async fn main() {
    let future = ReturnsNumber { number: 42 };
    // future.await;
    // println!("{number}");
    // assert_eq!(number, 42);
}
