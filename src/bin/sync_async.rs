use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let start = std::time::Instant::now();
    tokio::join! {
        do_async_work(),
        do_heavy_work(),
    };
    println!("Time taken: {:?}", start.elapsed());
}

fn handle_element<T>(_: T) {
    println!("Handled one element");
    std::thread::sleep(Duration::from_millis(200));
}

async fn do_heavy_work() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for elem in data {
        handle_element(elem);
        tokio::time::sleep(Duration::from_millis(0)).await;
        // tokio::task::yield_now().await; // This is slower than sleep(0)
    }
}

async fn do_async_work() {
    for i in 0..10 {
        println!("Async work {}", i);
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
