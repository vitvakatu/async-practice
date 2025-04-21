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

#[pin_project::pin_project]
struct Join<'a, F1: Future, F2: Future> {
    #[pin]
    f1: F1,
    #[pin]
    f2: F2,
    results: (Option<F1::Output>, Option<F2::Output>),
    _marker: std::marker::PhantomData<(&'a (), F1, F2)>,
}

impl<'a, F1: Future + 'a, F2: Future + 'a> Join<'a, F1, F2> {
    fn new(f1: F1, f2: F2) -> Self {
        Self {
            f1,
            f2,
            results: (None, None),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'a, F1: Future + 'a, F2: Future + 'a> Future for Join<'a, F1, F2> {
    type Output = (F1::Output, F2::Output);

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        if this.results.0.is_none() {
            match this.f1.as_mut().poll(ctx) {
                Poll::Ready(output) => {
                    this.results.0 = Some(output);
                }
                Poll::Pending => {}
            }
        }
        if this.results.1.is_none() {
            match this.f2.as_mut().poll(ctx) {
                Poll::Ready(output) => {
                    this.results.1 = Some(output);
                }
                Poll::Pending => {}
            }
        }
        if this.results.0.is_some() && this.results.1.is_some() {
            Poll::Ready((
                this.results.0.take().unwrap(),
                this.results.1.take().unwrap(),
            ))
        } else {
            Poll::Pending
        }
    }
}

async fn join<'a, F1: Future + 'a, F2: Future + 'a>(a: F1, b: F2) -> (F1::Output, F2::Output) {
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

    let (a, b) = join(
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
    )
    .await;
    println!("Results: {} {}", a, b);
}
