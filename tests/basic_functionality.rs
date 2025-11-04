use obsessive_peek::PeekMore;

#[test]
fn readme_example() {
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

#[test]
fn empty() {
    let iterable: [i32; 0] = [];

    let mut iter = iterable.iter().peekmore();

    assert_eq!(iter.peek(), None);

    let none = iter.peek_next();
    assert_eq!(none, None);

    let iter = iter.advance_cursor();
    assert_eq!(iter.peek(), None);
    assert_eq!(iter.peek_next(), None);
}

#[test]
fn test_with_inherited_feature_count() {
    let iterable = [1, 2, 3];
    let mut iter = iterable.iter().peekmore();

    iter.advance_cursor();
    let second = iter.peek().unwrap();
    assert_eq!(second, &&2);

    let consume_first = iter.next().unwrap();
    assert_eq!(consume_first, &1);

    let count = iter.count();
    assert_eq!(count, 2);
}
