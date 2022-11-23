mod executor;
mod timer_future;
use futures::join;

use executor::*;
use std::time::Duration;
use timer_future::TimerFuture;

fn main() {
    let future = TimerFuture::new(Duration::from_millis(1500));
    println!("Executing...");
    let (executor, spawner) = create_executor();
    spawner.spawn(future);

    spawner.spawn(async move {
        let y = async {
            println!("10");
        };
        let x = async {
            println!("20");
        };
        (x.await, y.await);
    });

    drop(spawner);

    executor.run();

    println!("Done");
}
