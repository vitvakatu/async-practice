#![feature(pin_macro)]

use std::{
    marker::PhantomPinned,
    pin::{pin, Pin},
};

// #[pin_project::pin_project]
pub struct SelfReferential {
    value: u32,
    #[pin]
    ptr: *const u32,
    _marker: PhantomPinned,
}

fn accepts_unpin<T: Unpin>(x: T) {}

impl SelfReferential {
    /// Safety: The caller is responsible for pinning the value and setting `ptr` to a valid address.
    pub unsafe fn new(value: u32) -> Self {
        Self {
            value,
            ptr: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    pub fn init(self: Pin<&mut Self>) {
        let mut this = self.project();
        *(this.ptr) = &*this.value;
    }

    pub fn value(self: Pin<&Self>) -> &mut u32 {
        let this = unsafe { Pin::get_unchecked_mut(self) };
        &mut this.value
    }

    // pub fn ptr(self: Pin<&mut Self>) -> Pin<&mut u32> {}
}

fn main() {
    let mut self_ref = unsafe { SelfReferential::new(10) };
    // let pinned = pin!(self_ref);
    // let mut pinned = Pin::new(&mut self_ref);
    let mut pinned = Box::pin(self_ref);
    SelfReferential::init(pinned.as_mut());
    println!("Value address: {:p}", &pinned.value);
    println!("Ptr address: {:?}", pinned.ptr);
    println!("Value: {}", pinned.value);
    println!("Ptr: {}", pinned.ptr.is_null());
    let _: () = pinned.as_mut().project().value;
}
