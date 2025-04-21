use std::future::Future;

use async_trait::async_trait;

#[async_trait]
trait MyTrait {
    async fn do_something(&self);
}

struct MyType;

#[async_trait]
impl MyTrait for MyType {
    async fn do_something(&self) {
        println!("Hello, world!");
    }
}

trait MyTrait2 {
    fn do_something2(&self) -> impl Future<Output = ()> + Send;
}

impl MyTrait2 for MyType {
    async fn do_something2(&self) {
        // This won’t work because of `Send` requirement:
        // let _rc = std::rc::Rc::new(self);
        // async {}.await;
        println!("Hello, world!");
    }
}

#[tokio::main]
async fn main() {
    let my_type = MyType;
    tokio::spawn(async move {
        my_type.do_something().await;
    });
    let _trait_object = Box::new(MyType) as Box<dyn MyTrait + Send>;
    tokio::spawn(async move {
        MyType.do_something2().await;
    });
}
