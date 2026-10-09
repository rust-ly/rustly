+++
id = "book.testing.organizing-tests"
chapter = "book.testing"
requires = []
title = "Unit Tests and Integration Tests"
track = "book"
order = 5
source = "https://doc.rust-lang.org/book/ch11-03-test-organization.html"
summary = "Unit tests sit next to the code and can test private functions; integration tests use your library from outside."
+++

# Test Organization

Rust has two kinds of test, and they answer different questions.

## Unit tests: does each piece work?

**Unit tests** live in the same file as the code, in a `tests` module marked `#[cfg(test)]`. They're small and focused, testing one function at a time.

Because the test module is a **child** of the code's module, the privacy rule from chapter 7 lets it see **private** functions. Rust doesn't force you to test only the public interface:

```rust,runnable title="A test module can call a private function"
pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(internal_adder(2, 2), 4);
    }
}

fn main() {
    println!("{}", add_two(40));
}
```

(On the Playground, Run calls `main`; the `tests` module is only compiled when running tests.)

## Integration tests: do the pieces work together?

**Integration tests** live in a separate `tests` folder next to `src`. Each file there is compiled as its own crate that uses your library from the **outside**, exactly as other people would, so it can only call public items:

```text
adder/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    ├── common/
    │   └── mod.rs          shared helpers (not a test file itself)
    └── integration_test.rs
```

```rust
// tests/integration_test.rs
use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(add_two(2), 4);
}
```

No `#[cfg(test)]` is needed: Cargo only compiles the `tests` folder when you run `cargo test`. Helpers shared by several test files go in `tests/common/mod.rs`; the `mod.rs` name tells Cargo it isn't a test file of its own. Rustly's API is tested this way: its `tests/common/mod.rs` holds a fake Playground server that several test files share.

Integration tests need a **library** crate (`src/lib.rs`). A binary-only crate can't be imported, which is one reason Rust programs usually keep their logic in `lib.rs` and leave `main.rs` tiny.

## Helper functions in tests

Tests are ordinary code, so they can use helper functions to avoid repetition. A helper isn't marked `#[test]`, so it doesn't run on its own:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn rect(w: u32, h: u32) -> Rectangle {
        Rectangle { width: w, height: h }
    }

    #[test]
    fn larger_can_hold_smaller() {
        assert!(rect(8, 7).can_hold(&rect(5, 1)));
    }
}
```
