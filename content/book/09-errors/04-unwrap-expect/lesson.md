+++
id = "book.errors.unwrap-expect"
chapter = "book.errors"
requires = []
title = "Shortcuts: unwrap, expect and unwrap_or"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#shortcuts-for-panic-on-error-unwrap-and-expect"
summary = "Turn a Result into its value quickly, either by panicking on error or by using a fallback."
+++

# Shortcuts: `unwrap`, `expect` and `unwrap_or`

Writing a full `match` every time gets long. `Result` (and `Option`) have helper methods for the common cases.

## `unwrap`: the value, or panic

`unwrap()` returns the value inside `Ok`. If it's an `Err`, it **panics**:

```rust,panics title="unwrap on an Err"
fn main() {
    let n: i32 = "42".parse().unwrap();
    println!("got {n}");
    let m: i32 = "oops".parse().unwrap();
    println!("never printed {m}");
}
```

## `expect`: the same, with your own message

`expect("...")` behaves like `unwrap`, but the panic message is yours. Use it to say **why** you were sure the value would be there, which is far more useful when the "impossible" happens:

```rust,panics title="expect explains itself"
fn main() {
    let port_text = "eighty";
    let port: u16 = port_text
        .parse()
        .expect("the PORT setting must be a whole number");
    println!("{port}");
}
```

Experienced Rust programmers prefer `expect` over `unwrap` in real code for exactly this reason.

## `unwrap_or`: the value, or a fallback

Panicking is only right when an error means a bug. When there's a sensible default, `unwrap_or(default)` returns the value on success and the default on failure, with no panic:

```rust,editable title="Falling back to a default"
fn main() {
    for input in ["8", "lots"] {
        let copies: u32 = input.parse().unwrap_or(1);
        println!("{input:?} -> printing {copies} copies");
    }
}
```

There's also `unwrap_or_default()`, which uses the type's default value (`0` for numbers, `""` for `String`), and `is_ok()` / `is_err()` for a simple yes/no check.

## When are `unwrap` and `expect` OK?

- In **examples and prototypes**, where error handling would distract from the point.
- In **tests**, where a panic is exactly how a test should fail.
- When **you know more than the compiler**: `"127.0.0.1".parse::<IpAddr>()` can't fail because the text is hard-coded, but the compiler can't see that. `expect` with a note is fine.

Everywhere else, handle the error or pass it to the caller, which is the next concept.
