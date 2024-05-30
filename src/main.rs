#![allow(dead_code)]
#![allow(unused_imports)]

mod executor;
mod timer_future;
use futures::{join, Future};

use executor::*;
use std::time::Duration;
use timer_future::TimerFuture;

#[tokio::main]
async fn main() {
    let future = TimerFuture::new(Duration::from_millis(1500));
    println!("Start future");
    future.await;
    println!("After 1500 millis");
}

// fn main() {
//     let future = TimerFuture::new(Duration::from_millis(1500));
//     println!("Executing...");
//     let (executor, spawner) = create_executor();
//     spawner.spawn(future);

//     spawner.spawn(do_work());
//     spawner.spawn(async {
//         let y = async {
//             println!("10");
//         };
//         let x = async {
//             println!("20");
//         };
//         (x.await, y.await);
//     });

//     drop(spawner);

//     executor.run();

//     println!("Done");
// }

// fn handle_element<T>(_: T) {
//     println!("Handled one element");
//     std::thread::sleep(Duration::from_millis(200));
// }

// async fn do_work() {
//     let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     for elem in data {
//         handle_element(elem);
//     }
// }
