+++
id = "book.errors.question-mark"
chapter = "book.errors"
requires = []
title = "Propagating Errors with ?"
track = "book"
order = 5
source = "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors"
summary = "Pass an error up to the caller instead of handling it yourself, and let ? write the match for you."
+++

# Propagating Errors with `?`

Often the function where an error happens isn't the right place to decide what to do about it. A function that reads a setting doesn't know whether the caller wants to retry, use a default, or show a message. So it **propagates** the error: it returns it to its caller and lets them decide.

## Propagating by hand

Here's a function that parses a width and a height and returns the area. If either parse fails, it returns that error to the caller:

```rust,runnable title="Returning errors with match"
use std::num::ParseIntError;

fn area(w: &str, h: &str) -> Result<u32, ParseIntError> {
    let width = match w.parse::<u32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    let height = match h.parse::<u32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    Ok(width * height)
}

fn main() {
    println!("{:?}", area("3", "4"));
    println!("{:?}", area("3", "four"));
}
```

That pattern ("if it's `Ok`, take the value; if it's `Err`, return it") is so common that Rust has an operator for it.

## The `?` operator

Put `?` after a `Result`. If it's `Ok(value)`, the expression becomes `value`. If it's `Err(e)`, the function **returns `Err(e)` immediately**:

```rust,editable title="The same function with ?"
use std::num::ParseIntError;

fn area(w: &str, h: &str) -> Result<u32, ParseIntError> {
    let width = w.parse::<u32>()?;
    let height = h.parse::<u32>()?;
    Ok(width * height)
}

fn main() {
    println!("{:?}", area("3", "4"));
    println!("{:?}", area("3", "four"));
}
```

Same behaviour, a fraction of the code. You can even chain it: `Ok(w.parse::<u32>()? * h.parse::<u32>()?)`.

One extra thing `?` does: if your function's error type is different, `?` converts the error using the `From` trait, as long as a conversion exists. That's how one function can collect errors of several kinds into one error type.

## `?` only works in functions that return Result

`?` might `return` an error, so the function it's in must return a `Result` (or `Option`, next concept). Using it in a function that returns something else doesn't compile:

```rust,does_not_compile title="? in a function that returns i32"
fn double(s: &str) -> i32 {
    let n: i32 = s.parse()?;
    n * 2
}

fn main() {
    println!("{}", double("4"));
}
```

The error, `E0277: the ? operator can only be used in a function that returns Result or Option`, says exactly that. Either change the return type to a `Result`, or handle the error with `match` or `unwrap_or`.
