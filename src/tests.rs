#![cfg(feature = "loom-tests")]

use crate::Stack;
use loom::{sync::Arc, thread};

#[test]
fn loom_stack() {
    let mut builder = loom::model::Builder::new();
    builder.max_branches = 25_000;

    builder.check(|| {
        let stack = Arc::new(Stack::new());

        const NUM: usize = 3;

        let stack1 = stack.clone();
        let j1 = thread::spawn(move || {
            for i in 0..NUM {
                stack1.push((1, i));
            }
        });

        let stack2 = stack.clone();
        let j2 = thread::spawn(move || {
            for i in 0..NUM {
                stack2.push((2, i));
            }
        });

        j1.join().unwrap();
        j2.join().unwrap();

        let mut res = Vec::new();

        loop {
            if let Some(value) = stack.pop() {
                res.push(value);
            } else {
                if res.len() >= 2 * NUM {
                    break;
                }

                thread::yield_now();
            }
        }

        for i in 0..NUM {
            assert!(res.iter().any(|v| v == &(1, i)));
            assert!(res.iter().any(|v| v == &(2, i)));
        }
    });
}
