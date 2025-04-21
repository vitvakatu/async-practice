use async_practice::timer_future::TimerFuture;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let future = TimerFuture::new(Duration::from_millis(1500));
    println!("Start future");
    future.await;
    println!("After 1500 millis");
}
