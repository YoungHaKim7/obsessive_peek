use criterion::{Criterion, criterion_group, criterion_main};
use obsessive_peek::PeekMore;

fn std_peek_test() {
    let mut iter = vec![1, 2, 3, 4, 5].into_iter().peekable();

    // Peek at the first element
    let first_peek = iter.peek().cloned();
    assert_eq!(first_peek, Some(1));

    // Consume first element
    let first = iter.next();
    assert_eq!(first, Some(1));

    // Peek at the second element
    let second_peek = iter.peek().cloned();
    assert_eq!(second_peek, Some(2));

    // Consume second element
    let second = iter.next();
    assert_eq!(second, Some(2));

    // Peek at the third element
    let third_peek = iter.peek().cloned();
    assert_eq!(third_peek, Some(3));

    // Consume third element
    let third = iter.next();
    assert_eq!(third, Some(3));
    assert_eq!(third_peek, third);

    // Continue consuming remaining elements
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(5));

    // Peeking beyond the end returns None
    assert_eq!(iter.peek(), None);
    assert_eq!(iter.next(), None);
}

fn peek_more_test() {
    let range10 = 0..11;
    let mut peekable = range10.peekmore();

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
    c.bench_function("std_peek test", |b| {
        b.iter(|| {
            std_peek_test();
        })
    });
    c.bench_function("peekmore test", |b| {
        b.iter(|| {
            peek_more_test();
        })
    });
}
criterion_group!(benches, peek_init);
criterion_main!(benches);
