

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
    let iter = vec![1, 2, 3, 4, 5].into_iter();
    let mut iter02 = vec![10, 20, 30, 40, 50].into_iter();

    // Peek at the first element.
    let first_peek = iter.peekable();

    println!("first_peek: {first_peek:?}");

    // Advance the iterator cursor to point at the first element.
    let first = iter02.next();
    println!("first {first:?}");
}

```

- result

```bash
first_peek: Peekable { iter: IntoIter([1, 2, 3, 4, 5]), peeked: None }
first Some(10)

```
