/// Use a `Vec` to queue iterator elements
use alloc::vec::Vec;

use crate::peek_iterator::PeekMoreIterator;

/// Trait which allows you to create the multi-peek iterator.
/// It allows you to peek at any unconsumed element.
/// Elements can be consumed using the [`next`] method defined on any [`Iterator`].
///
/// [`next`]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#tymethod.next
/// [`Iterator`]: https://doc.rust-lang.org/core/iter/trait.Iterator.html
pub trait PeekMore: Iterator + Sized {
    /// Create a multi-peek iterator where we can peek forward multiple times from an existing iterator.
    fn peekmore(self) -> PeekMoreIterator<Self>;
}

impl<I: Iterator> PeekMore for I {
    fn peekmore(self) -> PeekMoreIterator<I> {
        PeekMoreIterator {
            iterator: self,
            queue: Vec::new(),
            cursor: 0usize,
        }
    }
}
