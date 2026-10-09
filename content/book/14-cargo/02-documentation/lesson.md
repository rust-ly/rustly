+++
id = "book.cargo.documentation"
chapter = "book.cargo"
requires = []
title = "Documentation Comments"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments"
summary = "/// comments become HTML docs, and the examples in them run as tests."
+++

# Making Useful Documentation Comments

Ordinary comments start with `//`. **Documentation comments** start with `///` and describe the item right below them. They use Markdown, and `cargo doc --open` turns them into a website like the standard library's docs on docs.rs:

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

## Common sections

Most well-documented functions use a few standard headings:

- **Examples**: how to call it. Almost always worth including.
- **Panics**: when the function panics, so callers can avoid it.
- **Errors**: for functions returning `Result`, what kinds of error and why.
- **Safety**: for `unsafe` functions (chapter 20), the rules the caller must follow.

## Examples are tests

The best part: `cargo test` **runs the code in your examples** as **doc tests**. If you change the function and forget the docs, the example fails, so documentation can't silently go out of date. The output has a separate section, `Doc-tests my_crate`.

## Documenting the whole crate or module

`//!` documents the item that **contains** the comment, rather than the one after it. At the top of `src/lib.rs`, it describes the whole crate:

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
```

Rustly's code uses both: every module in `crates/runner` starts with a `//!` comment explaining what it's for.

## Docs as a specification

Good docs say exactly what a function does, which is just what you need to write it. Read the docs here, then make the code match:

```rust,editable title="Code that matches its docs"
/// Returns the number of vowels (a, e, i, o, u, in either case) in `text`.
///
/// # Examples
///
/// ```
/// assert_eq!(count_vowels("Rustacean"), 4);
/// assert_eq!(count_vowels("rhythm"), 0);
/// ```
fn count_vowels(text: &str) -> usize {
    text.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}

fn main() {
    assert_eq!(count_vowels("Rustacean"), 4);
    assert_eq!(count_vowels("rhythm"), 0);
    println!("the examples hold");
}
```
