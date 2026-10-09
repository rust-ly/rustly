+++
id = "book.errors.panic"
chapter = "book.errors"
requires = []
title = "Unrecoverable Errors with panic!"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html"
summary = "When something is so wrong the program can't continue, it panics: it stops and prints a message."
+++

# Unrecoverable Errors with `panic!`

Rust splits errors into two kinds:

- **Recoverable** errors are things that can reasonably go wrong, like a file that doesn't exist or text that isn't a number. The program should handle them and carry on. Rust uses the `Result` type for these (next concept).
- **Unrecoverable** errors are **bugs**: a situation that should never happen, like reading past the end of an array. The safest thing is to stop. Rust does this with a **panic**.

Most languages don't separate the two, and use one mechanism (exceptions) for both. Rust makes you choose, which makes it clear from the code what can fail.

## Panicking on purpose

The `panic!` macro stops the program with a message:

```rust,panics title="A deliberate panic"
fn main() {
    println!("before");
    panic!("crash and burn");
}
```

The output shows your message and **where** the panic happened: `src/main.rs:3:5` means line 3, column 5. "before" is printed; nothing after the panic runs.

## Panics caused by a bug

More often, a panic comes from code you call. Indexing past the end of a vector is a bug, so the standard library panics instead of returning garbage:

```rust,panics title="Reading past the end"
fn main() {
    let v = vec![1, 2, 3];
    v[99];
}
```

In C, reading past the end of an array gives you whatever memory happens to be there. That's a famous source of security holes (a *buffer over-read*). Rust stops instead.

## Your own panics

Use `panic!` in your own functions when the caller broke a rule that should never be broken, and continuing would only cause worse problems later:

```rust,editable title="Rejecting an impossible value"
fn hours_to_seconds(hours: i32) -> i32 {
    if hours < 0 {
        panic!("hours can't be negative, got {hours}");
    }
    hours * 3600
}

fn main() {
    println!("{}", hours_to_seconds(2));
    // Try changing this to -1:
    println!("{}", hours_to_seconds(1));
}
```

The message accepts `{}` placeholders just like `println!`, so include the value that caused the problem. It makes the bug much easier to find.

When a panic happens, Rust **unwinds**: it walks back up the call stack, cleaning up (dropping) every value it owned. Setting the environment variable `RUST_BACKTRACE=1` prints that call stack, which shows exactly how the program got to the panic.
