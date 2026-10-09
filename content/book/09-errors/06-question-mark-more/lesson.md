+++
id = "book.errors.question-mark-more"
chapter = "book.errors"
requires = []
title = "? with Option, and in main"
track = "book"
order = 6
source = "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#where-the--operator-can-be-used"
summary = "Use ? on Option values, and let main return a Result so it can use ? too."
+++

# Where the `?` Operator Can Be Used

## `?` on an Option

`?` also works on `Option`, in a function that returns `Option`. On `Some(value)` it gives the value; on `None` it returns `None` from the function straight away.

This function finds the last character of the first line. Two things might be missing (there might be no lines, and the line might be empty), and each `?` handles one:

```rust,runnable title="? on Option"
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    println!("{:?}", last_char_of_first_line("Hello, world\nHow are you?"));
    println!("{:?}", last_char_of_first_line(""));
    println!("{:?}", last_char_of_first_line("\nhi"));
}
```

`text.lines().next()` is an `Option<&str>`; if there are no lines, the `?` returns `None`. Otherwise the line's `chars().last()` is already an `Option<char>`, which becomes the function's result.

You can't mix the two: `?` on a `Result` won't work in a function that returns `Option`, and vice versa. Convert first, for example with `.ok()` (turns a `Result` into an `Option`) or `.ok_or(error)` (turns an `Option` into a `Result`).

```rust,editable title="Converting between Option and Result"
fn first_number(text: &str) -> Option<i32> {
    let first = text.split_whitespace().next()?;
    first.parse().ok()
}

fn require_name(name: Option<&str>) -> Result<&str, String> {
    name.ok_or(String::from("a name is required"))
}

fn main() {
    println!("{:?} {:?}", first_number("12 apples"), first_number("apples"));
    println!("{:?} {:?}", require_name(Some("Ada")), require_name(None));
}
```

## `main` can return a Result

`main` usually returns nothing, so `?` can't be used in it. But `main` may also return `Result<(), E>`. Then `?` works, and if `main` returns an `Err`, the program prints the error and exits with a failure code:

```rust,runnable title="? in main"
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let n: i32 = "42".parse()?;
    println!("parsed {n}");
    Ok(())
}
```

`Box<dyn Error>` means "any kind of error". You'll learn what `Box` and `dyn` mean in chapters 15 and 18; for now, read it as a catch-all error type that lets `?` work on any error in `main`.
