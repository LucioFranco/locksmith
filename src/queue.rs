// https://www.cs.rochester.edu/~scott/papers/1996_PODC_queues.pdf

use crate::sync::{atomic::AtomicPtr, CausalCell};
use std::ptr;

pub struct Queue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

struct Node<T> {
    value: CausalCell<Option<T>>,
    next: *mut Node<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        let node = Box::new(Node {
            value: CausalCell::new(None),
            next: ptr::null_mut(),
        });

        let node = Box::into_raw(node);

        Self {
            head: AtomicPtr::new(node),
            tail: AtomicPtr::new(node),
        }
    }

    // pub fn
}
