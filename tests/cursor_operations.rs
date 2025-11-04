use obsessive_peek::PeekMore;

#[test]
fn check_advance_separately() {
    let iterable = [1, 2, 3, 4];

    let mut iter = iterable.iter().peekmore(); // j -> 1

    assert_eq!(iter.cursor(), 0);
    assert_eq!(iter.peek(), Some(&&1));

    iter.advance_cursor(); // j -> 2
    assert_eq!(iter.cursor(), 1);

    iter.advance_cursor(); // j -> 3
    assert_eq!(iter.cursor(), 2);

    iter.advance_cursor(); // j -> 4
    assert_eq!(iter.cursor(), 3);

    let v4 = iter.peek();
    assert_eq!(v4, Some(&&4));
}

#[test]
fn check_advance_chain() {
    let iterable = [1, 2, 3, 4];

    let mut iter = iterable.iter().peekmore(); // j -> 1

    assert_eq!(iter.cursor(), 0);

    iter.advance_cursor() // j -> 2
        .advance_cursor() // j -> 3
        .advance_cursor(); // j -> 4

    let v4 = iter.peek();
    assert_eq!(v4, Some(&&4));
}

#[test]
fn check_move_previous() {
    let iterable = [1, 2, 3, 4];

    let mut iter = iterable.iter().peekmore(); // j -> 1

    assert_eq!(iter.cursor(), 0);
    assert_eq!(iter.peek(), Some(&&1));

    iter.advance_cursor(); // j -> 2
    assert_eq!(iter.cursor(), 1);

    let _ = iter.move_cursor_back(); // j -> 1
    assert_eq!(iter.cursor(), 0);

    iter.advance_cursor(); // j -> 2
    assert_eq!(iter.cursor(), 1);

    let _ = iter.move_cursor_back(); // j -> 1
    assert_eq!(iter.cursor(), 0);

    iter.advance_cursor(); // j -> 2
    assert_eq!(iter.cursor(), 1);

    iter.advance_cursor() // j -> 3
        .advance_cursor(); // j -> 4

    assert_eq!(iter.cursor(), 3);

    let v4 = iter.peek();
    assert_eq!(v4, Some(&&4));

    let _ = iter.move_cursor_back().and_then(|it| {
        it.move_cursor_back() // j -> 3
            .and_then(|it| {
                it.move_cursor_back() // j -> 2
                    .and_then(|it| it.move_cursor_back())
            })
    }); // j -> 1

    let v1 = iter.peek();
    assert_eq!(v1, Some(&&1));

    let prev = iter.move_cursor_back();
    assert!(prev.is_err());

    let v1 = iter.peek();
    assert_eq!(v1, Some(&&1));
}

#[test]
fn check_move_forward() {
    let iterable = [1, 2, 3, 4];
    let mut iter = iterable.iter().peekmore();

    let _ = iter.advance_cursor_by(3);

    let peek = iter.peek();
    assert_eq!(peek, Some(&&4));
    assert_eq!(iter.cursor(), 3);

    let _ = iter.advance_cursor_by(3);
    let peek = iter.peek();
    assert_eq!(peek, None);
    assert_eq!(iter.cursor(), 6);
}

#[test]
fn check_move_backward() {
    let iterable = [1, 2, 3, 4];
    let mut iter = iterable.iter().peekmore();

    let _ = iter.advance_cursor_by(3);

    let peek = iter.peek();
    assert_eq!(peek, Some(&&4));
    assert_eq!(iter.cursor(), 3);

    let result = iter.move_cursor_back_by(2);
    assert!(result.is_ok());
    let peek = iter.peek();
    assert_eq!(peek, Some(&&2));
    assert_eq!(iter.cursor(), 1);

    let result = iter.move_cursor_back_by(1);
    assert!(result.is_ok());
    let peek = iter.peek();
    assert_eq!(peek, Some(&&1));
    assert_eq!(iter.cursor(), 0);

    let result = iter.move_cursor_back_by(1);
    assert!(result.is_err());
    let peek = iter.peek();
    assert_eq!(peek, Some(&&1));
    assert_eq!(iter.cursor(), 0);
}

#[test]
fn check_move_backward_beyond_consumed_verify_cursor_position() {
    let iterable = [1, 2, 3, 4];
    let mut iter = iterable.iter().peekmore();

    let _ = iter.advance_cursor_by(3);

    let peek = iter.peek();
    assert_eq!(peek, Some(&&4));
    assert_eq!(iter.cursor(), 3);

    let result = iter.move_cursor_back_by(5);
    assert!(result.is_err());
    let peek = iter.peek();
    assert_eq!(peek, Some(&&4));
    assert_eq!(iter.cursor(), 3);
}

#[test]
fn check_move_backward_or_reset() {
    let iterable = [1, 2, 3, 4];
    let mut iter = iterable.iter().peekmore();

    let _ = iter.advance_cursor_by(3);

    let peek = iter.peek();
    assert_eq!(peek, Some(&&4));
    assert_eq!(iter.cursor(), 3);

    let _ = iter.move_cursor_back_or_reset(2);
    let peek = iter.peek();
    assert_eq!(peek, Some(&&2));
    assert_eq!(iter.cursor(), 1);

    let _ = iter.move_cursor_back_or_reset(1);
    let peek = iter.peek();
    assert_eq!(peek, Some(&&1));
    assert_eq!(iter.cursor(), 0);

    let _ = iter.move_cursor_back_or_reset(1);
    let peek = iter.peek();
    assert_eq!(peek, Some(&&1));
    assert_eq!(iter.cursor(), 0);
}

#[test]
fn check_move_backward_or_reset_beyond_consumed_verify_cursor_position() {
    let iterable = [1, 2, 3, 4];
    let mut iter = iterable.iter().peekmore();

    let _ = iter.advance_cursor_by(3);

    let peek = iter.peek();
    assert_eq!(peek, Some(&&4));
    assert_eq!(iter.cursor(), 3);

    let _ = iter.move_cursor_back_or_reset(5);
    let peek = iter.peek();
    assert_eq!(peek, Some(&&1));
    assert_eq!(iter.cursor(), 0);
}

#[test]
fn check_move_backward_or_reset_empty() {
    let iterable = "".chars();

    let mut iter = iterable.peekmore();

    assert_eq!(iter.peek(), None);
    assert_eq!(iter.cursor(), 0);

    let _ = iter.move_cursor_back_or_reset(5);

    assert_eq!(iter.peek(), None);
    assert_eq!(iter.cursor(), 0);
}

#[test]
fn check_move_forward_while() {
    let iterable = [1, 2, 3, 4];
    let mut iter = iterable.iter().peekmore();

    let _ = iter.advance_cursor_while(|i| **i.unwrap() != 3);

    let peek = iter.peek();
    assert_eq!(peek, Some(&&3));
    assert_eq!(iter.cursor(), 2);
}

#[test]
fn check_move_forward_while_empty() {
    let iterable: [i32; 0] = [];
    let mut iter = iterable.iter().peekmore();

    let _ = iter.advance_cursor_while(|i| if let Some(i) = i { **i != 3 } else { false });

    let peek = iter.peek();
    assert_eq!(peek, None);
    assert_eq!(iter.cursor(), 0);
}

#[test]
fn check_move_forward_while_some() {
    let iterable = [1, 2, 3, 4];
    let mut iter = iterable.iter().peekmore();

    let _ = iter.advance_cursor_while(|i| i.is_some());

    let peek = iter.peek();
    assert_eq!(peek, None);
    assert_eq!(iter.cursor(), 4);
}

#[test]
fn check_move_forward_while_fast_fail() {
    let iterable = [1, 2, 3, 4];
    let mut iter = iterable.iter().peekmore();

    iter.advance_cursor_by(2);

    let _ = iter.advance_cursor_while(|i| **i.unwrap() > 3);

    let peek = iter.peek();
    assert_eq!(peek, Some(&&3));
    assert_eq!(iter.cursor(), 2);
}

#[test]
fn check_move_nth() {
    let iterable = [1, 2, 3, 4];

    let mut iter = iterable.iter().peekmore();

    iter.move_nth(20);
    assert_eq!(iter.peek_nth(0), Some(&&1));
    assert_eq!(iter.cursor(), 20);
    assert_eq!(iter.peek(), None);

    iter.move_nth(0);
    assert_eq!(iter.peek(), Some(&&1));

    iter.move_nth(3);
    assert_eq!(iter.peek(), Some(&&4));
}

#[test]
fn check_move_nth_empty() {
    let iterable: [i32; 0] = [];

    let mut iter = iterable.iter().peekmore();

    iter.move_nth(0);
    assert_eq!(iter.cursor(), 0);

    iter.move_nth(10);
    assert_eq!(iter.cursor(), 10);
}
