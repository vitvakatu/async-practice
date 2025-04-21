use std::{
    pin::{pin, Pin},
    ptr::NonNull,
};

fn main() {
    let mut args = std::env::args();
    let step = args
        .nth(1)
        .unwrap_or("5".to_string())
        .parse::<u32>()
        .unwrap();

    struct SelfReferential {
        value: String,
        pointer: NonNull<String>,
    }

    let mut x = SelfReferential {
        value: "Hello, world!".to_string(),
        pointer: NonNull::dangling(),
    };
    // SAFETY: x.value is non-null as it is already allocated.
    x.pointer = unsafe { NonNull::new_unchecked(&mut x.value as *mut _) };

    println!("Struct address:\t\t{:p}", &x);
    println!("Field address:\t\t{:p}", &x.value);
    println!("Pointer points to:\t{:p}", x.pointer);
    if step == 1 {
        return;
    }

    // Let’s move x to other variable
    let y = x;
    println!("================");
    println!("Struct address:\t\t{:p}", &y);
    println!("Field address:\t\t{:p}", &y.value);
    println!("Pointer points to:\t\x1b[31m{:p}\x1b[0m", y.pointer);
    if step == 2 {
        return;
    }

    struct SafeSelfReferential {
        value: String,
        pointer: NonNull<String>,
        _marker: std::marker::PhantomPinned,
    }

    impl SafeSelfReferential {
        fn new(value: String) -> Pin<Box<Self>> {
            let mut self_ref = Box::pin(Self {
                value,
                pointer: NonNull::dangling(),
                _marker: std::marker::PhantomPinned,
            });
            // SAFETY: we don’t move anything out of the pinned reference
            let pinned = unsafe { Pin::get_unchecked_mut(self_ref.as_mut()) };
            let pointer = unsafe { NonNull::new_unchecked(&mut pinned.value as *mut _) };
            pinned.pointer = pointer;
            self_ref
        }
    }

    let x = SafeSelfReferential::new("Hello, world!".to_string());
    println!("================");
    println!("Struct address:\t\t{:p}", &x);
    println!("Field address:\t\t{:p}", &x.value);
    println!("Pointer points to:\t{:p}", x.pointer);

    let mut y = x;
    println!("================");
    println!("Struct address:\t\t{:p}", &y);
    println!("Field address:\t\t{:p}", &y.value);
    println!("Pointer points to:\t{:p}", y.pointer);
    if step == 3 {
        return;
    }

    impl SafeSelfReferential {
        fn get_value(self: Pin<&Self>) -> &str {
            &self.get_ref().value
        }

        fn get_value_pinned(self: Pin<&Self>) -> Pin<&str> {
            // SAFETY: it is impossible to move out of the shared str slice
            unsafe { self.map_unchecked(|s| s.value.as_str()) }
        }

        fn get_value_mut(self: Pin<&mut Self>) -> &mut String {
            // SAFETY: the address of the String will stay the same, no matter how it is modified
            unsafe { &mut self.get_unchecked_mut().value }
        }

        #[allow(dead_code)]
        fn get_value_mut_pinned(self: Pin<&mut Self>) -> Pin<&mut String> {
            // SAFETY: the address of the String will stay the same
            unsafe { self.map_unchecked_mut(|s| &mut s.value) }
        }
    }

    assert_eq!(SafeSelfReferential::get_value(y.as_ref()), "Hello, world!");
    assert_eq!(
        SafeSelfReferential::get_value_pinned(y.as_ref()).get_ref(),
        "Hello, world!"
    );
    // Edit value by mut reference
    y.as_mut().get_value_mut().push_str("!!!");
    assert_eq!(
        SafeSelfReferential::get_value(y.as_ref()),
        "Hello, world!!!!"
    );
    // Replace value with empty string
    std::mem::swap(y.as_mut().get_value_mut(), &mut String::new());
    assert_eq!(SafeSelfReferential::get_value(y.as_ref()), "");
    println!("Pointer still points to:{:p}", y.pointer);

    // Imagine we saved the pointer to the first _character_ of the string instead.
    // In this case, only `get_value_mut_pinned` getter will work correctly:

    // Compilation error:
    // std::mem::swap(y.as_mut().get_value_mut_pinned(), &mut String::new());

    if step == 4 {
        return;
    }
    // It is also possible to pin on stack:
    struct SelfReferentialChooseYourPoison {
        value: String,
        pointer: NonNull<String>,
        _marker: std::marker::PhantomPinned,
    }

    impl SelfReferentialChooseYourPoison {
        /// SAFETY: the user must call `init` before using the struct.
        unsafe fn new(value: String) -> Self {
            Self {
                value,
                pointer: NonNull::dangling(),
                _marker: std::marker::PhantomPinned,
            }
        }

        fn init(self: Pin<&mut Self>) {
            let self_ptr = unsafe { self.get_unchecked_mut() };
            let pointer = unsafe { NonNull::new_unchecked(&mut self_ptr.value as *mut _) };
            self_ptr.pointer = pointer;
        }
    }

    let on_stack = unsafe { SelfReferentialChooseYourPoison::new("Hello, world!".to_string()) };
    let mut on_stack = pin!(on_stack);
    on_stack.as_mut().init();
    println!("================");
    println!("Struct address:\t\t{:p}", &on_stack);
    println!("Field address:\t\t{:p}", &on_stack.value);
    println!("Pointer points to:\t{:p}", on_stack.pointer);
    let other_location = on_stack;
    println!("================");
    println!("Struct address:\t\t{:p}", &other_location);
    println!("Field address:\t\t{:p}", &other_location.value);
    println!("Pointer points to:\t{:p}", other_location.pointer);
}
