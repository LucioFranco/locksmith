#![cfg(feature = "loom-tests")]

use crate::Stack;
use loom::{sync::Arc, thread};

#[test]
fn loom_stack() {
    loom::model(|| {
        let stack = Arc::new(Stack::new());

        stack.push(0);

        let stack1 = stack.clone();
        thread::spawn(move || {
            stack1.push(1);
        })
        .join()
        .unwrap();

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), Some(0));
    });
}
