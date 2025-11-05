fn main() {
    let iter = vec![1, 2, 3, 4, 5].into_iter();
    let mut iter02 = vec![10, 20, 30, 40, 50].into_iter();

    // Peek at the first element.
    let first_peek = iter.peekable();

    println!("first_peek: {first_peek:?}");

    // Advance the iterator cursor to point at the first element.
    let first = iter02.next();
    println!("first {first:?}");
    // // Peek two steps ahead, at the third element.
    // let third_peek = iter.peek_nth(1).cloned();
    // assert_eq!(third_peek, Some(3));

    // // Advance the iterator cursor twice.
    // // The iterator cursor will now point to the third element.
    // iter.next();
    // let third = iter.next();
    // assert_eq!(third_peek, third);

    // // Peeking beyond the end of the iterator returns `None`.
    // let ambitious_peek = iter.peek_nth(5);
    // assert!(ambitious_peek.is_none());
}
