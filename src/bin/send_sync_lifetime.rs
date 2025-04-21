use std::future::Future;

fn main() {
    let x = async { 10 };

    let v = 123;
    let y = async { &v };

    let z = async {
        let non_sync = std::cell::RefCell::new(10);
        let res = non_sync.borrow();
        let x = *res;
        x
    };
    let w = async {
        let non_sync = std::cell::RefCell::new(10);
        async {}.await;
        let res = non_sync.borrow();
        let x = *res;
        x
    };
    let u = async {
        let non_send = std::rc::Rc::new(10);
        async {}.await;
        let x = *non_send;
        x
    };

    accepts_static(&x);
    accepts_sync(&x);
    accepts_send(&x);

    // accepts_static(&y);
    accepts_sync(&y);
    accepts_send(&y);

    accepts_static(&z);
    accepts_sync(&z);
    accepts_send(&z);

    accepts_static(&w);
    // accepts_sync(&w);
    accepts_send(&w);

    accepts_static(&u);
    // accepts_sync(&u);
    // accepts_send(&u);
}

fn accepts_static<F: Future + 'static>(_: &F) {}
fn accepts_sync<F: Future + Sync>(_: &F) {}
fn accepts_send<F: Future + Send>(_: &F) {}
