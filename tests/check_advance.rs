use obsessive_peek::PeekMore;

#[test]
fn test_advance_cursor_functionality() {
    let iterable = [1, 2, 3, 4];
    let mut iter = iterable.iter().peekmore();

    // Test basic cursor advance
    assert_eq!(iter.peek(), Some(&&1));
    assert_eq!(iter.cursor(), 0);

    iter.advance_cursor();
    assert_eq!(iter.peek(), Some(&&2));
    assert_eq!(iter.cursor(), 1);

    iter.advance_cursor();
    assert_eq!(iter.peek(), Some(&&3));
    assert_eq!(iter.cursor(), 2);

    // Test cursor reset
    iter.reset_cursor();
    assert_eq!(iter.peek(), Some(&&1));
    assert_eq!(iter.cursor(), 0);

    // Test cursor advance by multiple steps
    iter.advance_cursor_by(3);
    assert_eq!(iter.peek(), Some(&&4));
    assert_eq!(iter.cursor(), 3);
}
