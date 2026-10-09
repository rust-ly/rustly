+++
id = "book.testing.should-panic"
chapter = "book.testing"
requires = []
title = "Testing Panics and Results"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic"
summary = "Use #[should_panic] to check that bad input is rejected, and return Result from a test to use ?."
+++

# Checking for Panics with `should_panic`

Testing that good input works is half the job. The other half is checking that **bad** input is rejected. If a function is supposed to panic, add `#[should_panic]` to its test: the test then **passes if the code panics**, and fails if it doesn't.

Remember the `Guess` type from chapter 9, which only allows 1 to 100:

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

## Make it precise with `expected`

A plain `#[should_panic]` passes for **any** panic, even one caused by a different bug. Add `expected = "..."` and the test only passes if the panic message **contains** that text:

```rust
#[test]
#[should_panic(expected = "less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}
```

Now if `new` panicked for the wrong reason (say, a typo that rejects 200 as "too small"), the test would fail and point it out.

## Tests that return `Result`

A test can also return `Result<(), E>`. It passes on `Ok(())` and fails on `Err`. This lets you use `?` inside the test instead of `unwrap` on every line:

```rust
#[test]
fn it_parses() -> Result<(), String> {
    let n: i32 = "42".parse().ok().ok_or(String::from("not a number"))?;
    if n == 42 {
        Ok(())
    } else {
        Err(format!("expected 42, got {n}"))
    }
}
```

You can't use `#[should_panic]` on a test that returns `Result`. To check that something returns an error, assert on it directly: `assert!(value.is_err())`.

## Seeing it run

The Playground runs `main`, so here's the same idea outside a test: `Guess::new` panics with the message a `should_panic(expected = ...)` test would look for.

```rust,panics title="The panic a should_panic test expects"
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {value}.");
        }
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {value}.");
        }
        Guess { value }
    }
}

fn main() {
    let g = Guess::new(200);
    println!("{}", g.value);
}
```
