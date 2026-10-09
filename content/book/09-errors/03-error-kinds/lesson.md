+++
id = "book.errors.error-kinds"
chapter = "book.errors"
requires = []
title = "Matching on Different Errors"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#matching-on-different-errors"
summary = "Look inside an error to respond differently to each kind of failure."
+++

# Matching on Different Errors

Not every failure deserves the same response. If a settings file is **missing**, you might create it; if you don't have **permission** to read it, you should tell the user. Error values usually carry a **kind** that says which failure happened, so you can tell them apart.

Integer parsing is a good example. `ParseIntError` has a `kind()` method that returns an `IntErrorKind` enum, with variants like `Empty`, `InvalidDigit`, `PosOverflow` (too big) and `NegOverflow` (too small).

```rust,runnable title="What kind of failure?"
use std::num::IntErrorKind;

fn main() {
    for input in ["", "12a", "300", "-5"] {
        match input.parse::<u8>() {
            Ok(n) => println!("{input:?} -> {n}"),
            Err(e) => match e.kind() {
                IntErrorKind::Empty => println!("{input:?} -> nothing typed"),
                IntErrorKind::InvalidDigit => println!("{input:?} -> not a number"),
                IntErrorKind::PosOverflow => println!("{input:?} -> too big for a u8"),
                _ => println!("{input:?} -> other problem: {e}"),
            },
        }
    }
}
```

Notice the nested `match`: the outer one separates success from failure, and the inner one looks at the failure's kind. The `_` arm is needed because `IntErrorKind` is marked as **non-exhaustive**: the standard library might add new kinds in the future, so you must always have a catch-all.

## The same idea with files

The Book's example uses files, which the Playground can't open, so here it is to read rather than run. `File::open` returns `Result<File, io::Error>`, and `error.kind()` gives an `io::ErrorKind`:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other => panic!("Problem opening the file: {other:?}"),
        },
    };
}
```

If the file is missing it's created; any other error is unexpected, so it panics.

## Defining your own error kinds

For your own functions, an **enum** of error kinds is often better than a `String`, because callers can `match` on it instead of comparing text:

```rust,editable title="An error enum"
#[derive(Debug)]
enum PinError {
    WrongLength,
    NotDigits,
}

fn check_pin(pin: &str) -> Result<(), PinError> {
    if pin.len() != 4 {
        return Err(PinError::WrongLength);
    }
    if pin.parse::<u32>().is_err() {
        return Err(PinError::NotDigits);
    }
    Ok(())
}

fn main() {
    println!("{:?}", check_pin("1234"));
    println!("{:?}", check_pin("12"));
    println!("{:?}", check_pin("12ab"));
}
```

`Ok(())` means "it worked, and there's no value to return". `()` is the empty **unit** value.
