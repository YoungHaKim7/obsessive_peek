

<h1 align="center">obsessive_peek</h1>
<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/obsessive_peek">
    <img src="https://img.shields.io/crates/v/obsessive_peek.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/obsessive_peek">
    <img src="https://img.shields.io/crates/d/obsessive_peek.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/obsessive_peek">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>
<br/>

An iterator adapter to peek at future elements without advancing the cursor of the underlying
iterator.

Check out the [documentation](https://docs.rs/obsessive_peek) for more details.


# obsessive_peek
- original code https://github.com/foresterre/peekmore
- https://crates.io/crates/multipeek
  - https://github.com/LukeMathWalker/multipeek

# Example

```rs
use obsessive_peek;

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
```

- result

```bash


```

# ✅ Implemented Divide and Conquer Optimizations

- 1. fill_queue Optimization 
  - Original: Linear iteration for all batch sizes
  - Optimized: Uses chunked processing for batches > 1000 elements
  - Method: fill_queue_divide_conquer() processes elements in 500-element chunks
  - Benefit: Reduces function call overhead for large batches
- 2. Cursor Movement Optimization 
  - Original: Simple cursor addition
  - Optimized: advance_cursor_by_optimized() with binary search-like queue pre-allocation
  - Threshold: Activates for jumps > 100 elements
  - Benefit: Uses power-of-2 sizing and exponential growth patterns to minimize reallocations
- 3. Range Access Optimization 
  - Original: Linear range access
  - Optimized: peek_range_optimized() with chunked memory allocation
  - Threshold: Activates for ranges > 2000 elements
  - Benefit: Adaptive chunk sizing (500-2000 elements) based on range size

# 🧪 Comprehensive Testing
- Test coverage: Small/large batch processing, boundary conditions, performance consistency

# 📊 Performance Characteristics

  | Operation     | Threshold      | Optimization Strategy          |
  |---------------|----------------|--------------------------------|
  | Queue filling | >1000 elements | 500-element chunked processing |
  | Cursor jumps  | >100 elements  | Power-of-2 exponential growth  |
  | Range access  | >2000 elements | Adaptive chunking (500-2000)   |

- The implementation maintains full backward compatibility while providing significant performance improvements for large-scale iterator operations through intelligent divide and conquer strategies.
