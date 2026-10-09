+++
id = "book.testing.assert-eq"
chapter = "book.testing"
requires = []
title = "assert_eq!, assert_ne! and Custom Messages"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch11-01-writing-tests.html#testing-equality-with-the-assert_eq-and-assert_ne-macros"
summary = "Compare a result with the expected value, and see both values when it fails."
+++

# Testing Equality with `assert_eq!` and `assert_ne!`

Most tests compare a result with the value you expected. You could write `assert!(add_two(2) == 4)`, but when it fails, all you learn is "the assertion was false". `assert_eq!` tells you **what the two values were**:

```rust,panics title="assert_eq! shows both values"
fn add_two(a: u64) -> u64 {
    a + 3 // bug!
}

fn main() {
    assert_eq!(add_two(2), 4);
}
```

The output says `left: 5` and `right: 4`. That's usually enough to see the bug immediately. `assert_ne!` is the opposite: it panics if the two values are **equal**, which is useful when you know what a value must **not** be.

To be compared and printed, the values must implement the `PartialEq` and `Debug` traits. All the standard types do. For your own structs and enums, add `#[derive(PartialEq, Debug)]`.

## Custom failure messages

Both `assert!` and `assert_eq!` accept extra arguments after the condition, used like `format!`, to explain what was being tested:

```rust,panics title="A message that says what went wrong"
fn greeting(name: &str) -> String {
    String::from("Hello!") // forgot the name
}

fn main() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain the name, value was `{result}`"
    );
}
```

Without the message, you'd only see `assertion failed: result.contains("Carol")`. With it, you see what the function actually returned.

## Testing several cases

A function usually needs more than one check. Pick inputs that cover the **normal** case, the **edges** (zero, empty, the largest value) and anything that was tricky to write:

```rust,editable title="Several cases with assert_eq!"
fn clamp_percent(n: i32) -> i32 {
    if n < 0 {
        0
    } else if n > 100 {
        100
    } else {
        n
    }
}

fn main() {
    assert_eq!(clamp_percent(50), 50);
    assert_eq!(clamp_percent(0), 0);
    assert_eq!(clamp_percent(100), 100);
    assert_eq!(clamp_percent(-5), 0, "negative values clamp to 0");
    assert_eq!(clamp_percent(250), 100, "large values clamp to 100");
    println!("all checks passed");
}
```

`assert_eq!(left, right)` doesn't care which side is "expected" and which is "actual"; the convention in Rust is result first, expected second, but either works.
