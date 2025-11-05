use obsessive_peek::PeekMore;

fn main() {
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
