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

impl<T: std::fmt::Debug> Stack<T> {
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn push(&self, value: T) {
        let mut new_head = Box::new(Node {
            value: CausalCell::new(Some(value)),

            next: ptr::null_mut(),
        });

        let old_head = self.head.load(Ordering::SeqCst);

        new_head.next = old_head;

        let new_head = Box::into_raw(new_head);
        while self
            .head
            .compare_and_swap(old_head, new_head, Ordering::AcqRel)
            != old_head
        {
            #[cfg(all(test, feature = "loom-tests"))]
            loom::thread::yield_now();
        }
    }

    pub fn pop(&self) -> Option<T> {
        let old_head = self.head.load(Ordering::SeqCst);

        if old_head == ptr::null_mut() {
            return None;
        }

        let new_head = unsafe { &mut *old_head }.next;

        while self
            .head
            .compare_and_swap(old_head, new_head, Ordering::AcqRel)
            != old_head
        {
            #[cfg(all(test, feature = "loom-tests"))]
            loom::thread::yield_now();
        }

        let old = unsafe { Box::from_raw(old_head) };
        let value = unsafe { old.value.with_mut(|p| (*p).take()) };

        value
    }
}
