mod sync;

#[cfg(test)]
mod tests;

use std::ptr;
use sync::{
    atomic::{AtomicPtr, Ordering},
    CausalCell,
};

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

        let old_head = self.head.load(Ordering::Relaxed);

        new_head.next = old_head;

        let new_head = Box::into_raw(new_head);
        while self
            .head
            .compare_and_swap(old_head, new_head, Ordering::AcqRel)
            != old_head
        {}
    }

    pub fn pop(&self) -> Option<T> {
        let old_head = self.head.load(Ordering::Relaxed);

        if old_head == ptr::null_mut() {
            return None;
        }

        let old = unsafe { &mut *old_head };
        let value = unsafe { old.value.with_mut(|p| (*p).take()) };

        let new_head = unsafe { &mut *old_head }.next;

        while self
            .head
            .compare_and_swap(old_head, new_head, Ordering::AcqRel)
            != old_head
        {}

        value
    }
}
