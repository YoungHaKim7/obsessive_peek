/// This enumeration provides errors which represent lack of success of the [`PeekMoreIterator`].
///
/// [`PeekMoreIterator`]: struct.PeekMoreIterator.html
#[derive(Debug, Eq, PartialEq)]
pub enum PeekMoreError {
    /// This error case will be returned if we try to move to an element, but it has already been
    /// consumed by the iterator.
    /// We can only peek at elements which haven't been consumed.
    ElementHasBeenConsumed,
}
