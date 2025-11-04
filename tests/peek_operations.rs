use obsessive_peek::{PeekMore, PeekMoreError};

#[test]
fn peek_forward_with_reassignment() {
    let iterable = [1, 2, 3, 4];

    let mut peek = iterable.iter().peekmore();

    assert_eq!(peek.peek(), Some(&&1));

    let peek = peek.advance_cursor();
    assert_eq!(peek.peek(), Some(&&2));

    let peek = peek.advance_cursor();
    assert_eq!(peek.peek(), Some(&&3));

    let peek = peek.advance_cursor();
    assert_eq!(peek.peek(), Some(&&4));

    let peek = peek.advance_cursor();
    assert_eq!(peek.peek(), None);
}

#[test]
fn peek_forward_without_reassignment_separately_advance_and_peek() {
    let iterable = [1, 2, 3, 4];

    let mut iter = iterable.iter().peekmore();

    assert_eq!(iter.peek(), Some(&&1));

    let v2 = iter.advance_cursor().peek();
    assert_eq!(v2, Some(&&2));

    let v3 = iter.advance_cursor().peek();
    assert_eq!(v3, Some(&&3));

    let v4 = iter.advance_cursor().peek();
    assert_eq!(v4, Some(&&4));

    let v5 = iter.advance_cursor().peek();
    assert_eq!(v5, None);
}

#[test]
fn peek_forward_without_reassignment_advance_and_peek_combined() {
    let iterable = [1, 2, 3, 4];

    let mut iter = iterable.iter().peekmore();

    let v1 = iter.peek();
    assert_eq!(v1, Some(&&1));

    let v2 = iter.peek_next();
    assert_eq!(v2, Some(&&2));

    let v3 = iter.peek_next();
    assert_eq!(v3, Some(&&3));

    let v4 = iter.peek_next();
    assert_eq!(v4, Some(&&4));

    let v5 = iter.peek_next();
    assert_eq!(v5, None);
}

#[test]
fn peek_forward_without_reassignment_advance_and_peek_combined_and_reset_view() {
    let iterable = [1, 2, 3, 4];

    let mut iter = iterable.iter().peekmore();

    let v1 = iter.peek();
    assert_eq!(v1, Some(&&1));

    let v2 = iter.peek_next();
    assert_eq!(v2, Some(&&2));

    iter.reset_cursor();
    let v1again = iter.peek();
    assert_eq!(v1again, Some(&&1));

    let v2again = iter.peek_next();
    assert_eq!(v2again, Some(&&2));

    let v3 = iter.peek_next();
    assert_eq!(v3, Some(&&3));

    let v4 = iter.peek_next();
    assert_eq!(v4, Some(&&4));

    let v5 = iter.peek_next();
    assert_eq!(v5, None);
}

#[test]
fn check_peek_forward() {
    let iterable = [1, 2, 3, 4];
    let mut iter = iterable.iter().peekmore();

    let peek = iter.peek_forward(3);

    assert_eq!(peek, Some(&&4));
    assert_eq!(iter.cursor(), 3);

    let peek = iter.peek_forward(3);
    assert_eq!(peek, None);
    assert_eq!(iter.cursor(), 6);
}

#[test]
fn check_peek_backward() {
    let iterable = [1, 2, 3, 4];
    let mut iter = iterable.iter().peekmore();

    let _ = iter.advance_cursor_by(3);

    let peek = iter.peek();
    assert_eq!(peek, Some(&&4));
    assert_eq!(iter.cursor(), 3);

    let result = iter.peek_backward(2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(&&2));
    assert_eq!(iter.cursor(), 1);

    let result = iter.peek_backward(1);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some(&&1));
    assert_eq!(iter.cursor(), 0);

    let result = iter.peek_backward(1);
    assert!(result.is_err());
    let peek = iter.peek();
    assert_eq!(peek, Some(&&1));
    assert_eq!(iter.cursor(), 0);
}

#[test]
fn check_peek_backward_beyond_consumed_verify_cursor_position() {
    let iterable = [1, 2, 3, 4];
    let mut iter = iterable.iter().peekmore();

    let _ = iter.advance_cursor_by(3);

    let peek = iter.peek();
    assert_eq!(peek, Some(&&4));
    assert_eq!(iter.cursor(), 3);

    let result = iter.peek_backward(5);
    assert!(result.is_err());
    let peek = iter.peek();
    assert_eq!(peek, Some(&&4));
    assert_eq!(iter.cursor(), 3);
}

#[test]
fn check_peek_backward_or_first_beyond_consumed_verify_cursor_position() {
    let iterable = [1, 2, 3, 4];
    let mut iter = iterable.iter().peekmore();

    let _ = iter.advance_cursor_by(3);

    let peek = iter.peek();
    assert_eq!(peek, Some(&&4));
    assert_eq!(iter.cursor(), 3);

    let peek = iter.peek_backward_or_first(5);
    assert_eq!(peek, Some(&&1));
    assert_eq!(iter.cursor(), 0);
}

#[test]
fn check_peek_backward_or_first_empty() {
    let iterable = "".chars();

    let mut iter = iterable.peekmore();

    assert_eq!(iter.peek(), None);
    assert_eq!(iter.cursor(), 0);

    let peek = iter.peek_backward_or_first(5);

    assert_eq!(peek, None);
    assert_eq!(iter.cursor(), 0);
}

#[test]
fn peek_previous() {
    let iterable = [1, 2, 3];
    let mut iter = iterable.iter().peekmore(); // j = 1

    iter.advance_cursor(); // j = 2
    iter.advance_cursor(); // j = 3
    let value = iter.peek().unwrap(); // 3
    assert_eq!(value, &&3);

    let peek = iter.peek_previous(); // 2
    assert_eq!(peek.unwrap(), Some(&&2));
    assert_eq!(iter.cursor(), 1);

    let peek = iter.peek_previous(); // 1
    assert_eq!(peek.unwrap(), Some(&&1));
    assert_eq!(iter.cursor(), 0);

    let peek = iter.peek_previous();
    assert_eq!(peek, Err(PeekMoreError::ElementHasBeenConsumed));
    assert_eq!(iter.cursor(), 0);
}

#[test]
fn peek_previous_beyond_none() {
    let iterable = [1];
    let mut iter = iterable.iter().peekmore(); // j = 1
    assert_eq!(iter.cursor(), 0);

    iter.advance_cursor(); // j = None (1)
    let peek = iter.peek();
    assert_eq!(peek, None);
    assert_eq!(iter.cursor(), 1);

    iter.advance_cursor(); // j = None (2)
    let peek = iter.peek();
    assert_eq!(peek, None);
    assert_eq!(iter.cursor(), 2);

    iter.advance_cursor(); // j = None (3)
    let peek = iter.peek(); // current
    assert_eq!(peek, None);
    assert_eq!(iter.cursor(), 3);

    let peek = iter.peek_previous(); // None (2)
    assert_eq!(peek.unwrap(), None);
    assert_eq!(iter.cursor(), 2);

    let peek = iter.peek_previous(); // None (1)
    assert_eq!(peek.unwrap(), None);
    assert_eq!(iter.cursor(), 1);

    let peek = iter.peek_previous(); // 1
    assert_eq!(peek.unwrap(), Some(&&1));
    assert_eq!(iter.cursor(), 0);

    let peek = iter.peek_previous();
    assert_eq!(peek, Err(PeekMoreError::ElementHasBeenConsumed));
    assert_eq!(iter.cursor(), 0);

    let peek = iter.peek_previous();
    assert_eq!(peek, Err(PeekMoreError::ElementHasBeenConsumed));
    assert_eq!(iter.cursor(), 0);
}

#[test]
fn check_peek_nth() {
    let iterable = [1, 2, 3, 4];

    let mut iter = iterable.iter().peekmore();

    assert_eq!(iter.peek_nth(0), Some(&&1));
    assert_eq!(iter.cursor(), 0);
    assert_eq!(iter.peek_nth(1), Some(&&2));
    assert_eq!(iter.cursor(), 0);
    assert_eq!(iter.peek_nth(2), Some(&&3));
    assert_eq!(iter.cursor(), 0);
    assert_eq!(iter.peek_nth(3), Some(&&4));
    assert_eq!(iter.cursor(), 0);
    assert_eq!(iter.peek_nth(4), None);
    assert_eq!(iter.cursor(), 0);
}

#[test]
fn check_peek_first() {
    let iterable = [1, 2, 3, 4];
    let mut iter = iterable.iter().peekmore();

    // testing to make sure no matter where the cursor is, we always point
    // to the initial first element.
    assert_eq!(iter.peek_first(), Some(&&1));
    assert_eq!(iter.cursor(), 0);
    iter.advance_cursor();
    assert_eq!(iter.peek_first(), Some(&&1));
    assert_eq!(iter.cursor(), 1);
    iter.advance_cursor();
    assert_eq!(iter.peek_first(), Some(&&1));
    assert_eq!(iter.cursor(), 2);
    iter.advance_cursor();
    assert_eq!(iter.peek_first(), Some(&&1));
    assert_eq!(iter.cursor(), 3);
    iter.advance_cursor(); // try moving past the end too
    assert_eq!(iter.peek_first(), Some(&&1));

    // testing to ensure that it's the first *unconsumed* element of the iterator
    // and not the first of the iterable.
    iter.next();
    assert_eq!(iter.peek_first(), Some(&&2));
    iter.advance_cursor();
    assert_eq!(iter.peek_first(), Some(&&2));

    // testing at the end boundary of the iterable.
    iter.next(); // consume 2
    iter.next(); // consume 3
    assert_eq!(iter.peek_first(), Some(&&4));

    // test that if there's no unconsumed elements, it reports None.
    iter.next();
    assert_eq!(iter.peek_first(), None);
}

#[test]
fn check_peek_nth_empty() {
    let iterable: [i32; 0] = [];

    let mut iter = iterable.iter().peekmore();

    assert_eq!(iter.peek_nth(0), None);
    assert_eq!(iter.cursor(), 0);
    assert_eq!(iter.peek_nth(1), None);
    assert_eq!(iter.cursor(), 0);
}
