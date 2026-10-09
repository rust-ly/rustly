+++
id = "book.testing.running-tests"
chapter = "book.testing"
requires = []
title = "Controlling How Tests Run"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch11-02-running-tests.html"
summary = "Run some tests instead of all of them, see printed output, and skip slow tests by default."
+++

# Controlling How Tests Are Run

`cargo test` compiles your code in test mode and runs every test. Options before `--` go to Cargo; options after it go to the test runner. These are the ones you'll use most. (They're for your own projects; the Playground runs tests for you.)

## Running tests in parallel, or not

Tests run **in parallel** on several threads by default, so they finish faster. That means tests must not depend on each other or share state, such as writing to the same file. If they do, run them one at a time:

```sh
cargo test -- --test-threads=1
```

## Seeing printed output

Output from `println!` inside a **passing** test is hidden, to keep the report readable. Failing tests show their output. To see everything:

```sh
cargo test -- --show-output
```

## Running some tests by name

Pass part of a test's name, and only matching tests run. The report tells you how many were **filtered out**:

```rust
#[test]
fn add_two_and_two() { assert_eq!(add_two(2), 4); }

#[test]
fn add_three_and_two() { assert_eq!(add_two(3), 5); }

#[test]
fn one_hundred() { assert_eq!(add_two(100), 102); }
```

```sh
cargo test one_hundred   # runs just that test
cargo test add           # runs both tests whose names contain "add"
```

Module names are part of a test's name (`tests::one_hundred`), so you can run a whole module by its name, too.

## Ignoring slow tests

Mark a test `#[ignore]` and it's skipped unless you ask for it. This is good for tests that are slow or need something special, like network access:

```rust
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

```sh
cargo test                     # skips it
cargo test -- --ignored        # runs only the ignored tests
cargo test -- --include-ignored  # runs everything
```

Rustly itself works like this: the tests that send every exercise to the real Rust Playground are `#[ignore]`d, so everyday `cargo test` runs stay fast and don't hammer a free public service.

## Choosing good test cases

With testing tools in hand, the hard part is choosing **what** to test. A useful habit is to list the **edge cases** before writing any checks:

- **Empty** input: an empty string, slice or vector.
- **One** item, and **many** items.
- **Boundaries**: 0, the largest value, the exact limit where behaviour changes.
- **Order**: already sorted, reversed, all the same.
- **Duplicates**.

Most bugs live at the edges, so that's where most tests should be.
