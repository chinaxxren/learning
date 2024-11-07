#![allow(dead_code, unused_variables)]

use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
pub struct TestHeap {
    a: String,
    b: *const String,
    /// this make the type !UnPin;
    /// this ensures this type cannot be moved
    _marker: PhantomPinned,
}

impl TestHeap {
    pub fn new(txt: &str) -> Pin<Box<TestHeap>> {
        let temp = TestHeap {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(temp);
        let self_ptr = &boxed.a;
        // Safety: no other references are given out
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };
        boxed
    }

    pub fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    pub fn b(self: Pin<&Self>) -> &String {
        unsafe { &*self.b }
    }
}
