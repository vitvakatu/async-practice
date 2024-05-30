#[tokio::main]
async fn main() {
    let f = reqwest::get("https://rust-lang.org");
    let time_it = TimedFuture::new(f);
    let (_result, time_spent) = time_it.await;
    println!("Request took {} ms", time_spent.as_millis());
}
