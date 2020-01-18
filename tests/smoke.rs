use locksmith::Stack;

#[test]
fn stack() {
    let stack = Stack::new();

    stack.push(0);
    stack.push(1);
    stack.push(2);

    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), Some(0));
    assert_eq!(stack.pop(), None);
}
