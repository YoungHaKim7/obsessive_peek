use core::iter::FusedIterator;

/// Use a `Vec` to queue iterator elements
use alloc::vec::Vec;

use crate::peekerror::PeekMoreError;

/// This iterator makes it possible to peek multiple times without consuming a value.
/// In reality the underlying iterator will be consumed, but the values will be stored in a queue.
/// This queue allows us to peek at unconsumed elements (as far as the multi-peek iterator is concerned).
/// When the iterator [consumes] an element, the element at the front of the queue will be dequeued,
/// and will no longer be peekable.
///
/// [consumes]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#tymethod.next
#[derive(Clone, Debug)]
pub struct PeekMoreIterator<I: Iterator> {
    /// The underlying iterator. Consumption of this inner iterator does not represent consumption of the
    /// `PeekMoreIterator`.
    pub iterator: I,

    /// The queue represents the items of our iterator which have not been consumed, but can be peeked
    /// at without consuming them. Once an element has been consumed by the iterator, the element will
    /// be dequeued and it will no longer be possible to peek at this element.
    pub queue: Vec<Option<I::Item>>,

    /// The cursor points to the element we are currently peeking at.
    ///
    /// The cursor will point to the first unconsumed element if the value is `0`, the second if it is
    /// `1`, and so forth. Peeking at the 0th cursor element is equivalent to peeking with
    /// [`core::iter::Peekable::peek`].
    ///
    /// [`core::iter::Peekable::peek`]: https://doc.rust-lang.org/core/iter/struct.Peekable.html#method.peek
    pub cursor: usize,
}

impl<I: Iterator> PeekMoreIterator<I> {
    /// Get a reference to the element where the cursor currently points to. If no such element exists,
    /// return `None` will be returned.
    ///
    /// If we haven't advanced our cursor, it will point to the same element as `Iterator::next()` would
    /// return.
    /// Note that the cursor can't point to an element before the first unconsumed element within
    /// the iterator. In a sense the cursor moves independently within the iterator.
    /// But it can only point to unconsumed elements.
    ///
    /// The following illustration aims to show how `peek()` behaves. `i` represents the position
    /// of the iterator (i.e. the next value that will be returned if `next()` is called) and `j`
    /// represents the position of the cursor (i.e. the current element referenced if
    /// `peek()` is called).
    /// In example code next to the illustrations, the first element `1` is analogous to `A`,
    /// `2` to `B`, etc.
    ///
    /// The example below primarily uses `advance_cursor()` to move the cursor and `peek()` to
    /// peek at the element the cursor points to, but many often more convenient methods exist to
    /// change the element cursor points at, or to peek at those elements.
    ///
    /// * Let's start:
    ///
    /// ```rust
    /// use obsessive_peek::PeekMore;
    ///
    /// // Initialize our iterator.
    /// let iterable = [1, 2, 3, 4];
    /// let mut iterator = iterable.iter().peekmore();
    /// ```
    ///
    /// ```txt
    /// -----     -----      -----     -----
    /// | A | --> | B |  --> | C | --> | D | --> None --> None --> ...
    /// -----     -----      -----     -----
    ///   ^
    ///   i, j
    /// ```
    ///
    /// * Call `peek()`:
    ///
    /// ```rust
    /// # use obsessive_peek::PeekMore;
    /// # let iterable = [1, 2, 3, 4];
    /// # let mut iterator = iterable.iter().peekmore();
    /// let j = iterator.peek();
    /// assert_eq!(j, Some(&&1));
    /// ```
    ///
    /// ```txt
    /// -----     -----      -----     -----
    /// | A | --> | B |  --> | C | --> | D | --> None --> None --> ...
    /// -----     -----      -----     -----
    ///   ^
    ///   i, j
    ///      returns Some(&A)
    ///
    /// ```
    ///
    /// * Call `advance_cursor()`
    ///
    /// ```rust
    /// # use obsessive_peek::PeekMore;
    /// # let iterable = [1, 2, 3, 4];
    /// # let mut iterator = iterable.iter().peekmore();
    /// let iter = iterator.advance_cursor();
    /// ```
    ///
    /// ```txt
    /// -----     -----      -----     -----
    /// | A | --> | B |  --> | C | --> | D | --> None --> None --> ...
    /// -----     -----      -----     -----
    ///   ^         ^
    ///   i         j
    /// ```
    ///
    /// * Call `peek()`
    ///
    /// The reference returned by `peek()` will not change, similar to the behaviour of
    /// [`core::iter::Peekable::peek`]. In order to move to the next peekable element, we need to
    /// advance the cursor.
    ///
    /// ```rust
    /// # use obsessive_peek::PeekMore;
    /// # let iterable = [1, 2, 3, 4];
    /// # let mut iterator = iterable.iter().peekmore();
    /// # let iter = iterator.advance_cursor();
    /// let j = iterator.peek();
    /// assert_eq!(j, Some(&&2));
    ///
    /// // Calling `peek()` multiple times doesn't shift the position of the cursor;
    /// // a reference to the same element will be returned each call.
    /// assert_eq!(iterator.peek(), Some(&&2));
    /// assert_eq!(iterator.peek(), Some(&&2));
    /// ```
    ///
    /// ```txt
    /// -----     -----      -----     -----
    /// | A | --> | B |  --> | C | --> | D | --> None --> None --> ...
    /// -----     -----      -----     -----
    ///   ^         ^
    ///   i         j
    ///             returns Some(&B)
    /// ```
    ///
    ///
    /// * Call `next()`
    ///
    /// By calling next, the underlying iterator will be advanced andthe element represented by `A`
    /// will be consumed. It won't be possible to peek at `A` anymore.
    ///
    /// ```rust
    /// # use obsessive_peek::PeekMore;
    /// # let iterable = [1, 2, 3, 4];
    /// # let mut iterator = iterable.iter().peekmore();
    /// # let iter = iterator.advance_cursor();
    /// let i = iterator.next();
    /// assert_eq!(i, Some(&1));
    /// ```
    ///
    /// ```txt
    /// -----     -----      -----     -----
    /// | A |     | B |  --> | C | --> | D | --> None --> None --> ...
    /// -----     -----      -----     -----
    ///             ^
    ///             i, j
    ///  returns Some(A)
    /// ```
    ///
    /// * Call `next()`.
    ///
    /// The underlying iterator is advanced again.
    /// As a result, the cursor position also shifts to the next iterator position, which happens if
    /// the underlying iterator consumed an element where our cursor pointed at (that is if `j < i`).
    ///
    ///
    /// ```rust
    /// # use obsessive_peek::PeekMore;
    /// # let iterable = [1, 2, 3, 4];
    /// # let mut iterator = iterable.iter().peekmore();
    /// # let iter = iterator.advance_cursor();
    /// # let _ = iterator.next();
    /// // Show that the cursor still points at the second element.
    /// let j = iterator.peek();
    /// assert_eq!(j, Some(&&2));
    ///
    /// // Consume the second element.
    /// let i = iterator.next();
    /// assert_eq!(i, Some(&2));
    ///
    /// // Our cursor previously pointed at the element represented by B. Since that element has
    /// // been consumed, the cursor shifts to the next unconsumed element: C.
    /// let j = iterator.peek();
    /// assert_eq!(j, Some(&&3));
    ///
    ///
    /// ```
    ///
    /// ```txt
    /// -----     -----      -----     -----
    /// | A |     | B |      | C | --> | D | --> None --> None --> ...
    /// -----     -----      -----     -----
    ///                        ^
    ///                        i, j
    ///           returns Some(B)
    /// ```
    ///
    /// * Consume more elements by calling `next()` until we reach `None`:
    ///
    /// ```rust
    /// # use obsessive_peek::PeekMore;
    /// # let iterable = [1, 2, 3, 4];
    /// # let mut iterator = iterable.iter().peekmore();
    /// # let iter = iterator.advance_cursor();
    /// # let _ = iterator.next();
    /// # let j = iterator.peek();
    /// # assert_eq!(j, Some(&&2));
    /// # let i = iterator.next();
    /// # assert_eq!(i, Some(&2));
    /// # let j = iterator.peek();
    /// # assert_eq!(j, Some(&&3));
    /// let i = iterator.next();
    /// assert_eq!(i, Some(&3));
    ///
    /// let j = iterator.peek();
    /// assert_eq!(j, Some(&&4));
    ///
    /// let i = iterator.next();
    /// assert_eq!(i, Some(&4));
    ///
    /// let j = iterator.peek();
    /// assert_eq!(j, None);
    ///
    /// let i = iterator.next();
    /// assert_eq!(i, None);
    /// ```
    /// [`core::iter::Peekable::peek`]: https://doc.rust-lang.org/core/iter/struct.Peekable.html#method.peek
    #[inline]
    pub fn peek(&mut self) -> Option<&I::Item> {
        self.fill_queue(self.cursor);
        self.queue.get(self.cursor).and_then(|v| v.as_ref())
    }

    /// Peeks at the first unconsumed element, regardless of where the cursor currently is.
    #[inline]
    pub fn peek_first(&mut self) -> Option<&I::Item> {
        self.peek_nth(0)
    }

    // Convenient as we don't have to re-assign our mutable borrow on the 'user' side.
    /// Advance the cursor to the next element and return a reference to that value.
    #[inline]
    pub fn peek_next(&mut self) -> Option<&I::Item> {
        let this = self.advance_cursor();
        this.peek()
    }

    /// Try to peek at a previous element. If no such element exists, an `Err` result containing a
    /// [`PeekMoreError::ElementHasBeenConsumed`] will be returned.
    ///
    /// If a previous element does exist, an option wrapped in an `Ok` result will be returned.
    ///
    /// [`PeekMoreError::ElementHasBeenConsumed`]: enum.PeekMoreError.html#variant.ElementHasBeenConsumed
    #[inline]
    pub fn peek_previous(&mut self) -> Result<Option<&I::Item>, PeekMoreError> {
        if self.cursor >= 1 {
            self.move_cursor_back().map(|iter| iter.peek())
        } else {
            Err(PeekMoreError::ElementHasBeenConsumed)
        }
    }

    /// Move the cursor `n` steps forward and peek at the element the cursor then points to.
    #[inline]
    pub fn peek_forward(&mut self, n: usize) -> Option<&I::Item> {
        let this = self.advance_cursor_by(n);
        this.peek()
    }

    /// Move the cursor `n` steps backward and peek at the element the cursor then points to.
    ///
    /// If there aren't `n` elements prior to the element the cursor currently points at, a
    /// [`PeekMoreError::ElementHasBeenConsumed`] is returned instead.
    /// The cursor will then stay at the position it was prior to calling this method.
    ///
    /// If you want to peek at the first unconsumed element instead of returning with an error, you
    /// can use the [`peek_backward_or_first`] method instead.
    ///
    /// [`PeekMoreError::ElementHasBeenConsumed`]: enum.PeekMoreError.html#variant.ElementHasBeenConsumed
    /// [`peek_backward_or_first`]: struct.PeekMoreIterator.html#method.peek_backward_or_first
    #[inline]
    pub fn peek_backward(&mut self, n: usize) -> Result<Option<&I::Item>, PeekMoreError> {
        let _ = self.move_cursor_back_by(n)?;

        Ok(self.peek())
    }

    /// Move the cursor `n` steps backward and peek at the element the cursor then points to, or
    /// if there aren't `n` elements prior to the element the cursor currently points to, peek at
    /// the first unconsumed element instead.
    #[inline]
    pub fn peek_backward_or_first(&mut self, n: usize) -> Option<&I::Item> {
        if self.move_cursor_back_by(n).is_err() {
            self.reset_cursor();
        }

        self.peek()
    }

    /// Peek at the nth element without moving the cursor.
    #[inline]
    pub fn peek_nth(&mut self, n: usize) -> Option<&I::Item> {
        self.fill_queue(n);
        self.queue.get(n).and_then(|v| v.as_ref())
    }

    /// Advance the cursor to the next peekable element.
    ///
    /// This method does not advance the iterator itself. To advance the iterator, call [`next()`]
    /// instead.
    ///
    /// A mutable reference to the iterator is returned, which allows the operation to be chained.
    ///
    /// [`next()`]: struct.PeekMoreIterator.html#impl-Iterator
    #[inline]
    pub fn advance_cursor(&mut self) -> &mut PeekMoreIterator<I> {
        self.increment_cursor();
        self
    }

    /// Advance the cursor `n` elements forward.
    ///
    /// This does not advance the iterator itself. To advance the iterator, call [`next()`] instead.
    ///
    /// [`next()`]: struct.PeekMoreIterator.html#impl-Iterator
    #[inline]
    pub fn advance_cursor_by(&mut self, n: usize) -> &mut PeekMoreIterator<I> {
        if n > 0 {
            self.cursor += n;
            self
        } else {
            self
        }
    }

    /// Advance the cursor `n` elements forward with optimization for large jumps.
    /// Uses divide and conquer strategy to ensure the queue has sufficient capacity.
    ///
    /// This method is optimized for large jumps and will pre-allocate queue space more efficiently.
    ///
    /// This does not advance the iterator itself. To advance the iterator, call [`next()`] instead.
    ///
    /// [`next()`]: struct.PeekMoreIterator.html#impl-Iterator
    pub fn advance_cursor_by_optimized(&mut self, n: usize) -> &mut PeekMoreIterator<I> {
        if n == 0 {
            return self;
        }

        let new_cursor = self.cursor + n;

        // For large jumps, use binary search-like approach to determine optimal queue size
        if n > 100 {
            self.optimize_queue_for_cursor(new_cursor);
        } else {
            self.fill_queue(new_cursor);
        }

        self.cursor = new_cursor;
        self
    }

    /// Optimize queue size for a target cursor position using divide and conquer.
    /// This method pre-calculates the optimal queue size to minimize reallocations.
    fn optimize_queue_for_cursor(&mut self, target_cursor: usize) {
        let current_len = self.queue.len();

        if current_len <= target_cursor {
            // Estimate required capacity using exponential growth pattern
            let _estimated_capacity = if target_cursor > current_len * 2 {
                // Large jump - use power of 2 sizing for efficiency
                target_cursor.next_power_of_two()
            } else {
                // Moderate jump - use target + buffer
                target_cursor + (target_cursor / 4).min(1000)
            };

            // Fill queue to meet the target cursor position
            self.fill_queue(target_cursor);
        }
    }

    /// Moves the cursor forward until the predicate is no longer `true`.
    ///
    /// After this method returns, the cursor points to the first element that fails `predicate`. If no peeked elements
    /// pass `predicate` then the cursor will remain unchanged.
    ///
    /// This does not advance the iterator itself. To advance the iterator, call [`next()`] instead.
    ///
    /// [`next()`]: struct.PeekMoreIterator.html#impl-Iterator
    #[inline]
    pub fn advance_cursor_while<P: Fn(Option<&I::Item>) -> bool>(
        &mut self,
        predicate: P,
    ) -> &mut PeekMoreIterator<I> {
        let view = self.peek();

        if predicate(view) {
            self.increment_cursor();
            self.advance_cursor_while(predicate)
        } else {
            self
        }
    }

    /// Move the cursor to the previous peekable element.
    /// If such an element doesn't exist, a [`PeekMoreError::ElementHasBeenConsumed`] will be
    /// returned.
    ///
    /// If we can move to a previous element, a mutable reference to the iterator,
    /// wrapped in the `Ok` variant of `Result` will be returned.
    ///
    /// [`PeekMoreError::ElementHasBeenConsumed`]: enum.PeekMoreError.html#variant.ElementHasBeenConsumed
    #[inline]
    pub fn move_cursor_back(&mut self) -> Result<&mut PeekMoreIterator<I>, PeekMoreError> {
        if self.cursor >= 1 {
            self.decrement_cursor();
            Ok(self)
        } else {
            Err(PeekMoreError::ElementHasBeenConsumed)
        }
    }

    /// Move the cursor `n` elements backward. If there aren't `n` unconsumed elements prior to the
    /// cursor, an error will be returned instead. In case of an error, the cursor will stay at the position
    /// it pointed at prior to calling this method.
    ///
    /// If you want to reset the cursor to the first unconsumed element even if there aren't `n`
    /// unconsumed elements before the cursor position, the [`move_backward_or_reset`] method can be
    /// used.
    ///
    /// [`move_backward_or_reset`]: struct.PeekMoreIterator.html#method.move_backward_or_reset
    #[inline]
    pub fn move_cursor_back_by(
        &mut self,
        n: usize,
    ) -> Result<&mut PeekMoreIterator<I>, PeekMoreError> {
        if self.cursor < n {
            Err(PeekMoreError::ElementHasBeenConsumed)
        } else {
            self.cursor -= n;
            Ok(self)
        }
    }

    /// Move the cursor `n` elements backward, or reset its position to the first non-consumed element.
    /// The latter happens when the cursor position is smaller than the elements it has to move
    /// backwards by.
    #[inline]
    pub fn move_cursor_back_or_reset(&mut self, n: usize) -> &mut PeekMoreIterator<I> {
        if self.cursor < n {
            self.reset_cursor();
        } else {
            self.cursor -= n;
        }

        self
    }

    /// Move the cursor to the n-th element of the queue.
    #[inline]
    pub fn move_nth(&mut self, n: usize) -> &mut PeekMoreIterator<I> {
        self.cursor = n;
        self
    }

    /// Deprecated: use [`reset_cursor`] instead.
    ///
    /// [`reset_cursor`]: struct.PeekMoreIterator.html#method.reset_cursor
    #[deprecated]
    #[inline]
    pub fn reset_view(&mut self) {
        self.reset_cursor()
    }

    /// Reset the position of the cursor.
    ///
    /// If [`peek`] is called just after a reset, it will return a reference to the first element.
    ///
    /// [`peek`]: struct.PeekMoreIterator.html#method.peek
    #[inline]
    pub fn reset_cursor(&mut self) {
        self.cursor = 0;
    }

    /// Return the current cursor position.
    /// This is intended for use by code that more finely controls where the iterator resets to.
    #[inline]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Fills the queue up to (including) the cursor.
    #[inline]
    fn fill_queue(&mut self, required_elements: usize) {
        let stored_elements = self.queue.len();

        if stored_elements <= required_elements {
            // Use divide and conquer for large batches
            let elements_needed = required_elements - stored_elements + 1;

            if elements_needed > 1000 {
                self.fill_queue_divide_conquer(required_elements);
            } else {
                for _ in stored_elements..=required_elements {
                    self.push_next_to_queue()
                }
            }
        }
    }

    /// Fill queue using divide and conquer strategy for large batches.
    /// This method reduces the overhead of repeated function calls for large numbers of elements.
    fn fill_queue_divide_conquer(&mut self, required_elements: usize) {
        let current_len = self.queue.len();
        let remaining = required_elements - current_len + 1;

        // For very large batches, use chunked processing
        const CHUNK_SIZE: usize = 500;

        if remaining > CHUNK_SIZE {
            let chunks = remaining / CHUNK_SIZE;
            let remainder = remaining % CHUNK_SIZE;

            // Process full chunks
            for _ in 0..chunks {
                for _ in 0..CHUNK_SIZE {
                    self.push_next_to_queue();
                }
            }

            // Process remaining elements
            for _ in 0..remainder {
                self.push_next_to_queue();
            }
        } else {
            // For smaller batches, use the original approach
            for _ in current_len..=required_elements {
                self.push_next_to_queue();
            }
        }
    }

    /// Consume the underlying iterator and push an element to the queue.
    #[inline]
    fn push_next_to_queue(&mut self) {
        let item = self.iterator.next();
        self.queue.push(item);
    }

    /// Increment the cursor which points to the current peekable item.
    /// Note: if the cursor is [core::usize::MAX], it will not increment any further.
    ///
    /// [core::usize::MAX]: https://doc.rust-lang.org/core/usize/constant.MAX.html
    #[inline]
    fn increment_cursor(&mut self) {
        // do not overflow
        self.cursor = self.cursor.saturating_add(1);
    }

    /// Decrement the cursor which points to the current peekable item.
    /// Note: if the cursor is [core::usize::MIN], it will not decrement any further.
    ///
    /// [core::usize::MIN]: https://doc.rust-lang.org/core/usize/constant.MIN.html
    #[inline]
    fn decrement_cursor(&mut self) {
        if self.cursor > usize::MIN {
            self.cursor -= 1;
        }
    }

    /// Remove all elements from the start of the iterator until reaching the same
    /// position as the cursor by calling `Iterator::next()`.
    ///
    /// After calling this method, `iter.peek() == iter.next().as_ref()`.
    ///
    ///```rust
    /// use obsessive_peek::PeekMore;
    ///
    /// let iterable = [1, 2, 3, 4];
    /// let mut iter = iterable.iter().peekmore();
    ///
    /// iter.advance_cursor_by(2);
    /// assert_eq!(iter.peek(), Some(&&3));
    /// assert_eq!(iter.next(), Some(&1));
    /// iter.truncate_iterator_to_cursor();
    /// assert_eq!(iter.peek(), Some(&&3));
    /// assert_eq!(iter.next(), Some(&3));
    ///```
    pub fn truncate_iterator_to_cursor(&mut self) {
        if self.cursor < self.queue.len() {
            self.queue.drain(0..self.cursor);
        } else {
            // if the cursor is greater than the queue length,
            // we want to remove the overflow from the iterator
            for _ in 0..self.cursor.saturating_sub(self.queue.len()) {
                let _ = self.iterator.next();
            }
            self.queue.clear();
        }

        self.cursor = 0;
    }

    /// Returns a view into the next `start` (inclusive) to `end` (exclusive) elements.
    ///
    /// **Note:** `start` and `end` represent indices and start at `0`. These indices always start
    /// at the beginning of the queue (the unconsumed iterator) and don't take the position of the cursor
    /// into account.
    ///
    /// # Panics
    ///
    /// **Panics** if `start > end`, in which case the range would be negative.
    ///
    /// ```
    /// use obsessive_peek::PeekMore;
    ///
    /// let iterable = [1, 2, 3, 4];
    /// let mut iter = iterable.iter().peekmore();
    ///
    /// match iter.peek_range(1, 3) {
    ///     [Some(2), Some(p)] => println!("Yay! we found number {} after number 2", p),
    ///     _ => println!("Oh noes!"),
    /// }
    /// ```
    // implementation choice:
    // why not `core::ops::RangeBound<T>`? it adds unnecessary complexity since we would need to define what
    // unbounded bounds mean (e.g. for end whether it would be the end of the queue or the unconsumed iterator
    // elements until None or that it won't be allowed, or some other definition), we would need to map
    // the range Inclusive and Exclusive and Unbound-ed elements to usize, and we would need to verify
    // that T would be an unsigned integer. Using RangeBound would not be all negative though since we
    // could then use the standard Rust range syntax options such as 0..4 or 0..=3, which clearly
    // tell a user what kind of bounds are used (inclusive, exclusive, etc.)
    // For now however, for the reason of not adding unnecessary complexity, I've decided
    // that the simplicity of concrete start and end types is the better choice.
    pub fn peek_range(&mut self, start: usize, end: usize) -> &[Option<I::Item>] {
        assert!(
            start <= end,
            "range of the peeked view [start, end] should be positive (i.e. start <= end)"
        );

        // For large ranges, use divide and conquer optimization
        let range_size = end - start;
        if range_size > 2000 {
            self.peek_range_optimized(start, end)
        } else {
            // Original approach for smaller ranges
            if end > self.queue.len() {
                self.fill_queue(end);
            }
            &self.queue.as_slice()[start..end]
        }
    }

    /// Optimized peek_range implementation for large ranges using divide and conquer.
    /// This method pre-allocates memory in chunks to reduce reallocation overhead.
    fn peek_range_optimized(&mut self, start: usize, end: usize) -> &[Option<I::Item>] {
        let current_len = self.queue.len();

        if end > current_len {
            // Calculate optimal chunk size based on range size
            let range_size = end - current_len;
            let chunk_size = if range_size > 10000 {
                // Very large range - use larger chunks
                2000
            } else if range_size > 5000 {
                // Large range - medium chunks
                1000
            } else {
                // Medium range - smaller chunks
                500
            };

            // Fill queue in chunks using divide and conquer
            self.fill_queue_in_chunks(current_len, end, chunk_size);
        }

        &self.queue.as_slice()[start..end]
    }

    /// Fill the queue in chunks using divide and conquer strategy.
    /// This reduces memory reallocation overhead for large ranges.
    fn fill_queue_in_chunks(&mut self, current_end: usize, target_end: usize, chunk_size: usize) {
        let mut current_pos = current_end;

        while current_pos < target_end {
            let next_end = (current_pos + chunk_size).min(target_end);

            // Fill this chunk
            for _ in current_pos..next_end {
                self.push_next_to_queue();
            }

            current_pos = next_end;
        }
    }

    /// Returns a view into the next `n` unconsumed elements of the iterator.
    ///
    /// Here, `n` represents the amount of elements as counted from the start of the unconsumed iterator.
    ///
    /// For example, if we created a (peekmore) iterator from the array `[1, 2, 3]` and consume the first
    /// element by calling the regular `Iterator::next` method, and then call `peek_amount(3)`, the iterator will
    /// return `&[Some(2), Some(3), None]`. Here `Some(2)` and `Some(3)` are queued elements which
    /// we can peek at, and are not consumed by the iterator yet. `None` is the last element returned by
    /// our view, since our original iterator is sized and doesn't contain more elements. Thus in the absence
    /// of additional elements, we return `None`. This method is a variation on [`peek_range`].
    /// You could instead have called `peek_range(0, n)` (note that `peek_range` takes indices as arguments
    /// instead of an amount).
    ///
    /// **Note:** This method does not use or modify the position of the cursor.
    ///
    /// # Example:
    ///
    /// ```
    /// use obsessive_peek::PeekMore;
    ///
    /// let iterable = [1, 2, 3];
    /// let mut iter = iterable.iter().peekmore();
    ///
    /// match iter.peek_amount(4) { // -> &[Option(&1), Option(&2), Option(&3), None]
    ///   [Some(a), Some(b), Some(c), None] => println!("Found a match ({}, {}, {}) ", a, b, c),
    ///   _ => eprintln!("Expected (just) 3 more values"),
    /// }
    /// ```
    ///
    /// [`peek_range`]: struct.PeekMoreIterator.html#method.peek_range
    #[inline]
    pub fn peek_amount(&mut self, n: usize) -> &[Option<I::Item>] {
        self.peek_range(0, n)
    }

    /// Consumes and returns the next item of this iterator if a condition is true.
    ///
    /// If `func` returns `true` for the next item of this iterator, consume and return it.
    /// Otherwise, return `None`.
    ///
    /// Note: This function always uses the next item of the iterator and it is independent of
    /// the cursor location.
    ///
    /// # Example:
    /// Consume items one-by-one.
    /// ```
    /// use obsessive_peek::PeekMore;
    ///
    /// let mut iter = (1..5).peekmore();
    ///
    /// assert_eq!(iter.next_if(|&x| x == 1), Some(1));
    ///
    /// // next_eq does not care about the cursor position
    /// let mut iter = iter.advance_cursor();
    /// assert_eq!(iter.peek(), Some(&3));
    /// assert_eq!(iter.next_if(|&x| x == 2), Some(2));
    /// ```
    /// Consume a range of items.
    /// ```
    /// use obsessive_peek::PeekMore;
    ///
    /// let mut iter = (1..15).peekmore();
    ///
    /// while iter.next_if(|&x| x <= 10).is_some() {}
    /// assert_eq!(iter.next(), Some(11));
    /// ```
    #[inline]
    pub fn next_if(&mut self, func: impl FnOnce(&I::Item) -> bool) -> Option<I::Item> {
        match self.peek_first() {
            Some(matched) if func(matched) => self.next(),
            _ => None,
        }
    }

    /// Consumes and returns the next item if it is equal to `expected`.
    ///
    /// Uses [`next_eq`] underneath.
    ///
    /// [`next_eq`]: struct.PeekMoreIterator.html#method.next_if
    #[inline]
    pub fn next_if_eq<T>(&mut self, expected: &T) -> Option<I::Item>
    where
        T: ?Sized,
        I::Item: PartialEq<T>,
    {
        self.next_if(|next| next == expected)
    }
}

impl<I: Iterator> Iterator for PeekMoreIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let res = if self.queue.is_empty() {
            self.iterator.next()
        } else {
            self.queue.remove(0)
        };

        self.decrement_cursor();

        res
    }
}

/// Uses [`ExactSizeIterator`] default implementation.
///
/// [`ExactSizeIterator`]: https://doc.rust-lang.org/core/iter/trait.ExactSizeIterator.html
impl<I: ExactSizeIterator> ExactSizeIterator for PeekMoreIterator<I> {}

/// Uses [`FusedIterator`] default implementation.
///
/// [`FusedIterator`]: https://doc.rust-lang.org/core/iter/trait.FusedIterator.html
impl<I: FusedIterator> FusedIterator for PeekMoreIterator<I> {}
