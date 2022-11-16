mod executor;
mod timer_future;

use std::time::Duration;
use timer_future::TimerFuture;

#[tokio::main]
async fn main() {
    let future = TimerFuture::new(Duration::from_millis(1500));
    println!("Executing...");
    future.await;
    println!("Done");
}
