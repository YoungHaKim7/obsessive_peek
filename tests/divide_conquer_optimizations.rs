//! Tests for divide and conquer optimizations in PeekMoreIterator

use obsessive_peek::PeekMore;

#[test]
fn test_fill_queue_divide_conquer_small_batch() {
    let data: Vec<i32> = (0..10).collect();
    let mut iter = data.iter().peekmore();

    // Small batch should use regular fill_queue
    let range = iter.peek_range(0, 5);
    assert_eq!(range.len(), 5);
    assert_eq!(range[0], Some(&0));
    assert_eq!(range[4], Some(&4));
}

#[test]
fn test_fill_queue_divide_conquer_large_batch() {
    let data: Vec<i32> = (0..2000).collect();
    let mut iter = data.iter().peekmore();

    // Large batch should trigger divide and conquer (threshold > 1000)
    let range = iter.peek_range(0, 1500);
    assert_eq!(range.len(), 1500);
    assert_eq!(range[0], Some(&0));
    assert_eq!(range[1499], Some(&1499));
}

#[test]
fn test_advance_cursor_by_optimized_small_jump() {
    let data: Vec<i32> = (0..100).collect();
    let mut iter = data.iter().peekmore();

    // Small jump should use regular optimization
    iter.advance_cursor_by_optimized(5);
    assert_eq!(iter.peek(), Some(&&5));
}

#[test]
fn test_advance_cursor_by_optimized_large_jump() {
    let data: Vec<i32> = (0..500).collect();
    let mut iter = data.iter().peekmore();

    // Large jump should trigger optimized path (threshold > 100)
    iter.advance_cursor_by_optimized(200);
    assert_eq!(iter.peek(), Some(&&200));
}

#[test]
fn test_peek_range_optimized_small_range() {
    let data: Vec<i32> = (0..100).collect();
    let mut iter = data.iter().peekmore();

    // Small range should use regular peek_range
    let range = iter.peek_range(10, 20);
    assert_eq!(range.len(), 10);
    assert_eq!(range[0], Some(&10));
    assert_eq!(range[9], Some(&19));
}

#[test]
fn test_peek_range_optimized_large_range() {
    let data: Vec<i32> = (0..3000).collect();
    let mut iter = data.iter().peekmore();

    // Large range should trigger optimized path (threshold > 2000)
    let range = iter.peek_range(0, 2500);
    assert_eq!(range.len(), 2500);
    assert_eq!(range[0], Some(&0));
    assert_eq!(range[2499], Some(&2499));
}

#[test]
fn test_divide_conquer_performance_consistency() {
    let data: Vec<i32> = (0..5000).collect();
    let mut iter1 = data.iter().peekmore();
    let mut iter2 = data.iter().peekmore();

    // Test that optimized methods produce same results as regular methods

    // Test advance_cursor_by vs advance_cursor_by_optimized
    iter1.advance_cursor_by(150);
    iter2.advance_cursor_by_optimized(150);
    assert_eq!(iter1.peek(), iter2.peek());

    // Test peek_range consistency
    let range1 = iter1.peek_range(0, 100);
    let range2 = iter2.peek_range(0, 100);
    assert_eq!(range1, range2);
}

#[test]
fn test_chunk_filling_efficiency() {
    let data: Vec<i32> = (0..10000).collect();
    let mut iter = data.iter().peekmore();

    // Test chunked filling for very large ranges
    let range = iter.peek_range(0, 8000);
    assert_eq!(range.len(), 8000);

    // Verify all elements are correct
    for (i, elem) in range.iter().enumerate() {
        assert_eq!(elem, &Some(&(i as i32)));
    }
}

#[test]
fn test_cursor_optimization_boundary_conditions() {
    let data: Vec<i32> = (0..300).collect();
    let mut iter = data.iter().peekmore();

    // Test boundary condition exactly at threshold
    iter.advance_cursor_by_optimized(100);
    assert_eq!(iter.peek(), Some(&&100));

    // Test just above threshold
    iter.advance_cursor_by_optimized(101);
    assert_eq!(iter.peek(), Some(&&201));
}

#[test]
fn test_zero_operations() {
    let data: Vec<i32> = (0..10).collect();
    let mut iter = data.iter().peekmore();

    // Test zero advance
    let original_cursor = iter.cursor();
    iter.advance_cursor_by_optimized(0);
    assert_eq!(iter.cursor(), original_cursor);

    // Test empty range
    let range = iter.peek_range(0, 0);
    assert_eq!(range.len(), 0);
}
