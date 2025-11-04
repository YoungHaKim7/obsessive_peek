use obsessive_peek::PeekMore;

#[test]
fn test_with_consume() {
    let iterable = "123".chars();

    let mut iter = iterable.peekmore();
    assert_eq!(iter.peek(), Some(&core::char::from_digit(1, 10).unwrap()));
    assert_eq!(
        iter.peek_next(),
        Some(&core::char::from_digit(2, 10).unwrap())
    );
    assert_eq!(
        iter.peek_next(),
        Some(&core::char::from_digit(3, 10).unwrap())
    );
    assert_eq!(iter.peek_next(), None);
    assert_eq!(iter.next(), Some(core::char::from_digit(1, 10).unwrap()));
    assert_eq!(iter.peek(), None);
    assert_eq!(iter.peek_next(), None);
    assert_eq!(iter.next(), Some(core::char::from_digit(2, 10).unwrap()));
    assert_eq!(iter.peek(), None);
    assert_eq!(iter.peek_next(), None);
    assert_eq!(iter.next(), Some(core::char::from_digit(3, 10).unwrap()));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.peek_next(), None);
}

#[test]
fn test_with_consume_and_reset() {
    let iterable = "456".chars();

    let mut iter = iterable.peekmore();
    assert_eq!(iter.peek(), Some(&core::char::from_digit(4, 10).unwrap()));
    assert_eq!(
        iter.peek_next(),
        Some(&core::char::from_digit(5, 10).unwrap())
    );
    assert_eq!(
        iter.peek_next(),
        Some(&core::char::from_digit(6, 10).unwrap())
    );
    assert_eq!(iter.peek_next(), None);
    assert_eq!(iter.next(), Some(core::char::from_digit(4, 10).unwrap()));
    iter.reset_cursor();

    assert_eq!(iter.peek(), Some(&core::char::from_digit(5, 10).unwrap()));
    assert_eq!(
        iter.peek_next(),
        Some(&core::char::from_digit(6, 10).unwrap())
    );

    assert_eq!(iter.next(), Some(core::char::from_digit(5, 10).unwrap()));
    assert_eq!(iter.next(), Some(core::char::from_digit(6, 10).unwrap()));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.peek_next(), None);
}

#[test]
fn check_peek_window_moves_with_consume() {
    let iterable = [1, 2, 3, 4];

    let mut iter = iterable.iter().peekmore();

    let v1 = iter.peek();
    assert_eq!(v1, Some(&&1));

    let v1c = iter.next();
    assert_eq!(v1c, Some(&1));

    let v2 = iter.peek();
    assert_eq!(v2, Some(&&2));

    let v2c = iter.next();
    assert_eq!(v2c, Some(&2));

    let v3 = iter.peek();
    assert_eq!(v3, Some(&&3));

    iter.reset_cursor();

    let v3 = iter.peek();
    assert_eq!(v3, Some(&&3));

    let v3c = iter.next();
    assert_eq!(v3c, Some(&3));

    let v4c = iter.next();
    assert_eq!(v4c, Some(&4));

    let v5 = iter.peek();
    assert_eq!(v5, None);

    let v5c = iter.next();
    assert_eq!(v5c, None);
}

#[test]
fn truncate_iterator_to_cursor_is_noop_when_queue_is_empty_from_no_peeking() {
    let iterable = [1, 2, 3, 4];

    let mut iter = iterable.iter().peekmore();

    // Before truncation, peek should work normally
    assert_eq!(iter.peek(), Some(&&1));

    iter.truncate_iterator_to_cursor();

    // After truncation, peek should still work normally
    assert_eq!(iter.peek(), Some(&&1));
}

#[test]
fn truncate_iterator_to_cursor_is_noop_when_queue_is_empty_from_iteration() {
    let iterable = [1, 2, 3, 4];

    let mut iter = iterable.iter().peekmore();

    iter.peek_forward(2);
    iter.next();
    iter.next();
    iter.next();

    iter.truncate_iterator_to_cursor();

    // After truncation, peek should work normally
    assert_eq!(iter.peek(), Some(&&4));
}

#[test]
fn truncate_to_iterator_fill_queue() {
    let mut iter = [0, 1, 2, 3].iter().peekmore();
    iter.advance_cursor();
    iter.truncate_iterator_to_cursor();

    let value = **iter.peek().unwrap();

    assert_eq!(value, 1);
}

#[test]
fn truncate_to_iterator_on_empty_collection() {
    let mut iter = core::iter::empty::<i32>().peekmore();
    iter.advance_cursor();
    assert_eq!(iter.cursor(), 1);

    iter.truncate_iterator_to_cursor();
    assert_eq!(iter.cursor(), 0);

    assert!(iter.peek().is_none());
}

#[test]
fn truncate_to_iterator_on_single_element_collection() {
    let mut iter = core::iter::once(0).peekmore();
    assert_eq!(*iter.peek().unwrap(), 0);
    assert_eq!(iter.cursor(), 0);

    iter.advance_cursor(); // starts at 0, so now is 1 (i.e. second element so None)
    assert_eq!(iter.cursor(), 1);
    assert!(iter.peek().is_none());

    iter.truncate_iterator_to_cursor();
    assert_eq!(iter.cursor(), 0);

    assert!(iter.peek().is_none());
}

#[test]
fn truncate_to_iterator_cursor_and_queue_equal_length() {
    let mut iter = [0, 1, 2, 3].iter().peekmore();
    iter.peek();
    iter.advance_cursor();
    iter.truncate_iterator_to_cursor();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

#[test]
fn truncate_to_iterator_cursor_less_than_queue_length() {
    let mut iter = [0, 1, 2, 3].iter().peekmore();
    iter.peek_nth(2);
    iter.truncate_iterator_to_cursor();

    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);

    let mut iter = [0, 1, 2, 3].iter().peekmore();
    iter.peek_nth(3);
    iter.advance_cursor();
    iter.truncate_iterator_to_cursor();

    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

#[test]
fn next_if_works() {
    let iterable = [1, 2, 3, 4];

    let mut iter = iterable.iter().peekmore();

    assert_eq!(iter.next_if(|&x| *x == 1), Some(&1));

    assert_eq!(iter.peek(), Some(&&2));
    assert_eq!(iter.next_if(|&x| *x < 4), Some(&2));
    assert_eq!(iter.peek(), Some(&&3));

    assert_eq!(iter.peek(), Some(&&3));
    assert_eq!(iter.next_if(|&x| *x != 3), None);
    assert_eq!(iter.peek(), Some(&&3));

    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next_if(|&_x| true), Some(&4));
}

#[test]
fn next_if_exhausted() {
    let iterable = [1, 2];

    let mut iter = iterable.iter().peekmore();

    assert_eq!(iter.next_if(|&x| *x == 1), Some(&1));
    assert_eq!(iter.next_if(|&x| *x == 2), Some(&2));

    assert_eq!(iter.next_if(|&x| *x == 2), None);
}

#[test]
fn next_if_loop() {
    let iterable = 1..15;

    let mut iter = iterable.peekmore();

    while iter.next_if(|&x| x < 10).is_some() {}
    assert_eq!(iter.next(), Some(10));
}

#[test]
fn next_if_with_advanced_cursor() {
    let iterable = [1, 2, 3, 4];

    let mut iter = iterable.iter().peekmore();

    assert_eq!(iter.peek(), Some(&&1));
    let iter = iter.advance_cursor();
    let iter = iter.advance_cursor();
    assert_eq!(iter.peek(), Some(&&3));
    assert_eq!(iter.next_if(|&x| *x == 1), Some(&1));
    assert_eq!(iter.peek(), Some(&&3));
    assert_eq!(iter.next_if(|&x| *x == 2), Some(&2));
}

#[test]
fn next_if_eq_works() {
    let iterable = [1, 2, 3, 4];

    let mut iter = iterable.iter().peekmore();

    assert_eq!(iter.next_if_eq(&&1), Some(&1));

    assert_eq!(iter.peek(), Some(&&2));
    assert_eq!(iter.next_if_eq(&&2), Some(&2));
    assert_eq!(iter.peek(), Some(&&3));

    assert_eq!(iter.peek(), Some(&&3));
    assert_eq!(iter.next_if_eq(&&0), None);
    assert_eq!(iter.peek(), Some(&&3));

    assert_eq!(iter.next_if_eq(&&3), Some(&3));
    assert_eq!(iter.next_if_eq(&&4), Some(&4));

    assert_eq!(iter.next_if_eq(&&5), None);
}
