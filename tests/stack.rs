extern crate thompson_construction;

use thompson_construction::stack::Stack;

#[test]
fn just_example() {
    let mut s: Stack<i32> = Stack::new();

    assert_eq!(s.pop(), None);

    s.push(1);
    s.push(3);

    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(1));

    assert_eq!(s.pop(), None);
    println!("onegin");
}
