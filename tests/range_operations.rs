use obsessive_peek::PeekMore;

#[test]
fn peek_range_from_start_smaller_than_input_len() {
    let mut peeking_queue = [0, 1, 2, 3].iter().peekmore();
    let view = peeking_queue.peek_range(0, 2);

    assert_eq!(view[0], Some(&0));
    assert_eq!(view[1], Some(&1));
    assert_eq!(view.len(), 2);
}

#[test]
fn peek_range_from_start_eq_to_input_len() {
    let mut peeking_queue = [0, 1, 2, 3].iter().peekmore();
    let view = peeking_queue.peek_range(0, 4);

    assert_eq!(view[0], Some(&0));
    assert_eq!(view[1], Some(&1));
    assert_eq!(view[2], Some(&2));
    assert_eq!(view[3], Some(&3));
    assert_eq!(view.len(), 4);
}

#[test]
fn peek_range_from_start_bigger_than_input_len() {
    let mut peeking_queue = [0, 1, 2, 3].iter().peekmore();
    let view = peeking_queue.peek_range(0, 6);

    assert_eq!(view[0], Some(&0));
    assert_eq!(view[1], Some(&1));
    assert_eq!(view[2], Some(&2));
    assert_eq!(view[3], Some(&3));
    assert_eq!(view[4], None);
    assert_eq!(view[5], None);
    assert_eq!(view.len(), 6);
}

#[test]
fn peek_range_from_middle() {
    let mut peeking_queue = [0, 1, 2, 3].iter().peekmore();
    let view = peeking_queue.peek_range(2, 5);

    assert_eq!(view[0], Some(&2));
    assert_eq!(view[1], Some(&3));
    assert_eq!(view[2], None);
    assert_eq!(view.len(), 3);
}

#[test]
fn peek_range_out_of_bounds() {
    let mut peeking_queue = [0, 1, 2, 3].iter().peekmore();
    let view = peeking_queue.peek_range(5, 6);

    assert_eq!(view[0], None);
    assert_eq!(view.len(), 1);
}

#[test]
fn peek_range_empty() {
    let mut peeking_queue = [0, 1, 2, 3].iter().peekmore();
    let view = peeking_queue.peek_range(0, 0);

    assert_eq!(view.len(), 0);
}

#[test]
fn peek_range_match() {
    let mut peeking_queue = ["call", "f", "1"].iter().peekmore();
    let view = peeking_queue.peek_range(1, 3);

    let value = match view {
        &[Some(&"f"), Some(ref arg)] => arg,
        _ => panic!("test case peek_range_match failed"),
    };

    assert_eq!(**value, "1");
    assert_eq!(view.len(), 2);
}

#[test]
#[should_panic]
fn peek_range_panic_on_invalid_range() {
    let mut peeking_queue = [0, 1, 2, 3].iter().peekmore();
    let _ = peeking_queue.peek_range(2, 1);
}

#[test]
fn peek_amount_from_start_smaller_than_input_len() {
    let mut peeking_queue = [0, 1, 2, 3].iter().peekmore();
    let view = peeking_queue.peek_amount(2);

    assert_eq!(view[0], Some(&0));
    assert_eq!(view[1], Some(&1));
    assert_eq!(view.len(), 2);
}

#[test]
fn peek_amount_from_start_eq_to_input_len() {
    let mut peeking_queue = [0, 1, 2, 3].iter().peekmore();
    let view = peeking_queue.peek_amount(4);

    assert_eq!(view[0], Some(&0));
    assert_eq!(view[1], Some(&1));
    assert_eq!(view[2], Some(&2));
    assert_eq!(view[3], Some(&3));
    assert_eq!(view.len(), 4);
}

#[test]
fn peek_amount_from_start_bigger_than_input_len() {
    let mut peeking_queue = [0, 1, 2, 3].iter().peekmore();
    let view = peeking_queue.peek_amount(6);

    assert_eq!(view[0], Some(&0));
    assert_eq!(view[1], Some(&1));
    assert_eq!(view[2], Some(&2));
    assert_eq!(view[3], Some(&3));
    assert_eq!(view[4], None);
    assert_eq!(view[5], None);
    assert_eq!(view.len(), 6);
}

#[test]
fn peek_amount_empty() {
    let empty: [u32; 0] = [];
    let mut peeking_queue = empty.iter().peekmore();
    let view = peeking_queue.peek_amount(3);

    assert_eq!(view[0], None);
    assert_eq!(view[1], None);
    assert_eq!(view[2], None);
    assert_eq!(view.len(), 3);
}

#[test]
fn peek_amount_zero() {
    let mut peeking_queue = [0, 1, 2, 3].iter().peekmore();
    let view = peeking_queue.peek_amount(0);

    assert_eq!(view.len(), 0);
}

#[test]
fn peek_amount_match() {
    let mut peeking_queue = ["call", "f", "1"].iter().peekmore();
    let view = peeking_queue.peek_amount(4);

    let value = match view {
        &[Some(&"call"), Some(&"f"), Some(ref arg), None] => arg,
        _ => panic!("test case peek_n_match failed"),
    };

    assert_eq!(**value, "1");
    assert_eq!(view.len(), 4);
}

#[test]
fn peek_amount_renewed_view() {
    let mut peeking_queue = [0, 1, 2, 3].iter().peekmore();
    let view = peeking_queue.peek_amount(2);

    assert_eq!(view[0], Some(&0));
    assert_eq!(view[1], Some(&1));

    let _removed = peeking_queue.next();

    let view = peeking_queue.peek_amount(2);

    assert_eq!(view[0], Some(&1));
    assert_eq!(view[1], Some(&2));
}