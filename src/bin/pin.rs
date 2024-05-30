use std::{
    marker::PhantomPinned,
    pin::{pin, Pin},
};

pub struct SelfReferential {
    value: u32,
    ptr: *const u32,
    _marker: PhantomPinned,
}

fn check_unpin<T: Unpin>(_: T) {}

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
        let this = unsafe { self.get_unchecked_mut() };
        this.ptr = &this.value;
    }

    pub fn value(self: Pin<&mut Self>) -> &mut u32 {
        let this = unsafe { self.get_unchecked_mut() };
        &mut this.value
    }

    pub fn value_pinned(self: Pin<&mut Self>) -> Pin<&mut u32> {
        unsafe { self.map_unchecked_mut(|this| &mut this.value) }
    }
}

fn main() {
    let mut self_ref = unsafe { SelfReferential::new(10) };
    let mut pinned = pin!(self_ref);
    SelfReferential::init(pinned.as_mut());
    *pinned.as_mut().value() = 40;
    println!("Value address: {:p}", &pinned.value);
    println!("Ptr address: {:?}", pinned.ptr);
    println!("Value: {}", pinned.value);
    println!("Ptr: {}", pinned.ptr.is_null());
}
