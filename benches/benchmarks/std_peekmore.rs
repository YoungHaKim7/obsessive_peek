use criterion::{Criterion, criterion_group, criterion_main};
use obsessive_peek::PeekMore;

fn std_peek_test() {
    let iter = vec![1, 2, 3, 4, 5].into_iter();
    let mut iter02 = vec![10, 20, 30, 40, 50].into_iter();

    // Peek at the first element.
    let first_peek = iter.peekable();

    // Advance the iterator cursor to point at the first element.
    let first = iter02.next();

    // // Peek two steps ahead, at the third element.
    let third_peek = iter.peek_nth(1).cloned();
    assert_eq!(third_peek, Some(3));

    // Advance the iterator cursor twice.
    // The iterator cursor will now point to the third element.
    iter.next();
    let third = iter.next();
    assert_eq!(third_peek, third);

    // Peeking beyond the end of the iterator returns `None`.
    let ambitious_peek = iter.peek_nth(5);
    assert!(ambitious_peek.is_none());
}

fn peek_more_test() {
    let range10 = 0..11;
    let range10_clone = 0..11;
    let mut peekable = range10.peekmore();
    let peekable_clone = range10_clone.peekmore();
    // println!("range10: {range10_clone:?}");
    println!("range10: {peekable_clone:?}");

    // Peek at the first element
    let peek_first = peekable.peek();
    assert_eq!(*peek_first.unwrap(), 0);

    let peek_first_redux = peekable.peek_nth(0);
    assert_eq!(*peek_first_redux.unwrap(), 0);

    // Peek at the 10th (index) element
    let peek_tenth = peekable.peek_nth(10);
    assert_eq!(*peek_tenth.unwrap(), 10);

    // Consume the 10th element
    let tenth = peekable.nth(10);
    assert_eq!(tenth.unwrap(), 10);

    // Show that there are no more elements
    assert_eq!(peekable.peek(), None);
    assert_eq!(peekable.next(), None);
}

pub fn peek_init(c: &mut Criterion) {
    let size = 1_000_000;

    c.bench_function("std_peek test", |b| {
        b.iter(|| {
            let std_peek_data = std_peek_test();
        })
    });
    c.bench_function("peekmore test", |b| {
        b.iter(|| {
            let std_peek_data = std_peek_test();
        })
    });
}
criterion_group!(benches, peek_init);
criterion_main!(benches);
