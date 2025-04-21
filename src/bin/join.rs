#![allow(dead_code)]
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

fn random_numbers(seed: u32) -> impl Iterator<Item = u32> {
    let mut random = seed;
    std::iter::repeat_with(move || {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        random
    })
}

struct Join<F1: Future, F2: Future> {
    f1: Box<dyn Future<Output = F1::Output> + 'static>,
    f2: Box<dyn Future<Output = F2::Output> + 'static>,
    _marker: std::marker::PhantomData<(F1, F2)>,
}

impl<F1: Future + 'static, F2: Future + 'static> Join<F1, F2> {
    fn new(f1: F1, f2: F2) -> Self {
        Self {
            f1: Box::new(f1),
            f2: Box::new(f2),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<F1: Future, F2: Future> Future for Join<F1, F2> {
    type Output = (F1::Output, F2::Output);

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

async fn join<F1: Future + 'static, F2: Future + 'static>(
    a: F1,
    b: F2,
) -> (F1::Output, F2::Output) {
    Join::new(a, b).await
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let rng = random_numbers(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32,
    );
    let numbers = rng.filter(|n| *n < 100).take(2).collect::<Vec<_>>();
    let [delay_a, delay_b] = numbers.as_slice() else {
        panic!("Failed to get random numbers");
    };

    let (a, b) = tokio::join!(
        async {
            tokio::time::sleep(std::time::Duration::from_millis(*delay_a as u64)).await;
            println!("A");
            delay_a
        },
        async {
            tokio::time::sleep(std::time::Duration::from_millis(*delay_b as u64)).await;
            println!("B");
            delay_b
        },
    );
    println!("Results: {} {}", a, b);
}
