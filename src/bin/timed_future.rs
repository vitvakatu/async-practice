use std::{future::Future, time::Duration};

struct TimedFuture;

impl TimedFuture {
    pub fn new<F>(future: F) -> impl Future<Output = (F::Output, Duration)>
    where
        F: Future,
    {
        async move {
            let start = std::time::Instant::now();
            let result = future.await;
            let duration = start.elapsed();
            (result, duration)
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
