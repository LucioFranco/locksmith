use crate::sync::{
    atomic::{AtomicPtr, Ordering},
    CausalCell,
};
use std::ptr;

/// A lock-free stack.
pub struct Stack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    value: CausalCell<Option<T>>,
    next: *mut Node<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn push(&self, value: T) {
        let new_head = Box::new(Node {
            value: CausalCell::new(Some(value)),
            next: self.head.load(Ordering::Acquire),
        });

        let new_head = Box::into_raw(new_head);

        loop {
            match self.head.compare_exchange(
                unsafe { &*new_head }.next,
                new_head,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    break;
                }
                Err(actual) => {
                    unsafe { &mut *new_head }.next = actual;
                    #[cfg(all(test, feature = "loom-tests"))]
                    loom::thread::yield_now();
                }
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        let old_head = self.head.load(Ordering::Acquire);

        if old_head == ptr::null_mut() {
            return None;
        }

        let new_head = unsafe { &mut *old_head }.next;

        let mut old_head = old_head;
        loop {
            match self.head.compare_exchange(
                old_head,
                new_head,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    break;
                }
                Err(actual) => {
                    old_head = actual;
                    #[cfg(all(test, feature = "loom-tests"))]
                    loom::thread::yield_now();
                }
            }
        }

        let old = unsafe { Box::from_raw(old_head) };
        unsafe { old.value.with_mut(|p| (*p).take()) }
    }
}
