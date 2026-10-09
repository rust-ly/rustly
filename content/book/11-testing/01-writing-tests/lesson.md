+++
id = "book.testing.writing-tests"
chapter = "book.testing"
requires = []
title = "How to Write Tests"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch11-01-writing-tests.html"
summary = "A test is a function marked #[test]; it passes if it returns and fails if it panics."
+++

# How to Write Tests

The compiler checks that your code is **well-formed**: types match, references are valid. It can't check that your code is **correct**. If you meant `add_two` to add 2 and it adds 3, that's still valid Rust. Tests catch that kind of mistake.

You've been relying on tests this whole course: every checkpoint is graded by hidden tests. Now you'll write them.

## The anatomy of a test

A test is an ordinary function with the `#[test]` attribute above it. The rule is simple: a test **passes if it finishes** and **fails if it panics**.

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

- `#[cfg(test)]` means "only compile this module when running tests", so the test code isn't part of the real program.
- `use super::*;` brings everything from the parent module (your code) into the test module.
- `cargo test` finds every `#[test]` function, runs them, and reports `test tests::it_works ... ok`.

## `assert!`

The `assert!` macro takes a `bool`. If it's `true`, nothing happens. If it's `false`, it panics, so the test fails:

```rust,runnable title="assert! in action"
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let larger = Rectangle { width: 8, height: 7 };
    let smaller = Rectangle { width: 5, height: 1 };
    assert!(larger.can_hold(&smaller));
    assert!(!smaller.can_hold(&larger));
    println!("both assertions held");
}
```

A good test checks both directions: that the true case is true **and** that the false case is false. A broken `can_hold` that always returned `true` would pass the first assertion but not the second.

## A failing test

When an assertion fails, you see which one and where:

```rust,panics title="An assertion that fails"
fn is_even(n: u32) -> bool {
    n % 2 == 1 // bug!
}

fn main() {
    assert!(is_even(4));
}
```

## How these checkpoints work

In this chapter's checkpoints you write the **checks**. The function to test is passed in as a parameter, with a type like `fn(u32) -> bool` ("a function that takes a `u32` and returns a `bool`"). The grader calls your checker with a correct version, which must pass, and with versions that have **planted bugs**, which your assertions must catch by panicking.
