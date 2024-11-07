#![allow(dead_code, unused_variables)]

use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
pub struct Test {
    a: String,
    b: *const String,
    /// this make the type !UnPin;
    /// this ensures this type cannot be moved
    _marker: PhantomPinned,
}

impl Test {
    pub fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    pub fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    pub fn init_pinned(self: Pin<&mut Self>) {
        let self_ref: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ref;
    }

    pub fn a(&self) -> &str {
        &self.a
    }

    pub fn a_pinned(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    pub fn b(&self) -> &String {
        assert!(!self.b.is_null());
        unsafe { &*self.b }
    }

    pub fn b_pinned(self: Pin<&Self>) -> &String {
        // this makes sure that b is not called before init funct
        assert!(!self.b.is_null());
        unsafe { &*self.b }
    }
}
