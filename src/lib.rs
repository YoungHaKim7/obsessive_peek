#![no_std]
#![deny(missing_docs)]
#![deny(clippy::all)]

//! **Synopsis:**
//!
//! This crate introduces a multi-peekable iterator.
//! The iterator is similar to [`Peekable`]. The main difference is that [`Peekable`] only
//! allows you to peek at the next element and no further. When using `PeekMore` however,
//! you can peek at as many elements as you want.
//!
//! **A peek at how it works:**
//!
//! To enable peeking at multiple elements ahead of consuming a next element, the iterator uses a
//! traversable queue which holds the elements which you can peek at, but have not been
//! consumed (yet).
//! The underlying data structure of this queue is a `Vec` which stores the elements.
//!
//!
//! **Illustrated example:**
//!
//! An illustrated example can be found at the [`PeekMoreIterator::peek`] documentation.
//!
//!
//! **Usage example:**
//!
//! ```rust
//! use obsessive_peek::PeekMore;
//!
//! let iterable = [1, 2, 3, 4];
//! let mut iter = iterable.iter().peekmore();
//!
//! // Peek at the first element.
//! let v1 = iter.peek();
//! assert_eq!(v1, Some(&&1));
//!
//! // Consume the first element.
//! let v1c = iter.next();
//! assert_eq!(v1c, Some(&1));
//!
//! // Peek at the second element (the element our cursor points at also moved to the second element,
//! // since the first element was consumed.)
//! let v2 = iter.peek();
//! assert_eq!(v2, Some(&&2));
//!
//! // Advance the cursor. The cursor will now point to the third element.
//! let _ = iter.advance_cursor();
//!
//! // Check that it is indeed at the third element.
//! let v3 = iter.peek();
//! assert_eq!(v3, Some(&&3));
//!
//! // Reset the position the cursor points at. The cursor will point to the first unconsumed element
//! // again.
//! iter.reset_cursor();
//!
//! // Check that we are indeed at the second element again.
//! let v2 = iter.peek();
//! assert_eq!(v2, Some(&&2));
//!
//! // Shift the position of the cursor to the right twice by chaining the advance_view method.
//! let _ = iter.advance_cursor().advance_cursor();
//!
//! // Verify that the cursor indeed points at the fourth element.
//! let v4 = iter.peek();
//! assert_eq!(v4, Some(&&4));
//!
//! // Reset the position which the cursor points at again.
//! iter.reset_cursor();
//!
//! // We can also advance the cursor and peek with a single operation.
//! let v3 = iter.peek_next();
//! assert_eq!(v3, Some(&&3));
//! ```
//!
//!
//! [`Peekable`]: https://doc.rust-lang.org/core/iter/struct.Peekable.html
//! [`PeekMoreIterator::peek`]: struct.PeekMoreIterator.html#method.peek
//! [requires]: https://github.com/servo/rust-smallvec/issues/160

/// We need to allocate elements which haven't been consumed by the PeekMore iterator.
extern crate alloc;

/// Import std only when running doc tests without errors. Std will not be included outside of
/// doctest based binaries.
///
/// See [rust#54010](https://github.com/rust-lang/rust/issues/54010) for the error thrown by `doctest`
/// if no allocator is present (e.g. with just core/alloc).
/// Note that `cfg(doctest)` requires Rust 1.40 ([tracking issue](https://github.com/rust-lang/rust/issues/62210)).
/// As a result of the above, `doctest` is disabled on the CI for Rust versions below `1.40`.
#[cfg(doctest)]
extern crate std;

/// Use the system allocator when running doc tests.
///
/// See [rust#54010](https://github.com/rust-lang/rust/issues/54010) for the error thrown by `doctest`
/// if no allocator is present (e.g. with just core/alloc).
/// Note that `cfg(doctest)` requires Rust 1.40 ([tracking issue](https://github.com/rust-lang/rust/issues/62210)).
/// As a result of the above, `doctest` is disabled on the CI for Rust versions below `1.40`.
#[cfg(doctest)]
#[global_allocator]
static A: std::alloc::System = std::alloc::System;

mod peek_iterator;
mod peekerror;
mod peekmore;

// Public exports
pub use peekmore::PeekMore;
pub use peek_iterator::PeekMoreIterator;
pub use peekerror::PeekMoreError;
