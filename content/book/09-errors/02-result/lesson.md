+++
id = "book.errors.result"
chapter = "book.errors"
requires = []
title = "Recoverable Errors with Result"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html"
summary = "Result<T, E> is either Ok with a value or Err with an error, and match handles both."
+++

# Recoverable Errors with `Result`

Most errors aren't bugs. A user types "twelve" where you wanted a number; a file has been deleted; the network is down. The program should notice and respond, not crash. For these, functions return a `Result`, defined in the standard library as `enum Result<T, E> { Ok(T), Err(E) }`.

Like `Option`, it's an ordinary enum. `T` is the type of the value on success, and `E` is the type of the error on failure. Also like `Option`, the variants `Ok` and `Err` can be used without a prefix.

`Option` says "there might be no value". `Result` says "there might be no value, **and here's why**".

## A function that returns Result

Turning text into a number can fail, so `parse` returns a `Result`. Here we ask for an `i32`:

```rust,runnable title="parse returns a Result"
fn main() {
    let good: Result<i32, _> = "42".parse();
    let bad: Result<i32, _> = "forty-two".parse();
    println!("{good:?}");
    println!("{bad:?}");
}
```

The success is `Ok(42)`. The failure is `Err(ParseIntError { kind: InvalidDigit })`: an error value that describes what went wrong.

## Handling both cases with `match`

Because `Result` is an enum, you handle it the same way as `Option`, and the compiler makes sure you deal with the error:

```rust,editable title="Reacting to bad input"
fn main() {
    for input in ["7", "seven"] {
        match input.parse::<i32>() {
            Ok(n) => println!("{input} doubled is {}", n * 2),
            Err(e) => println!("couldn't read {input:?}: {e}"),
        }
    }
}
```

`parse::<i32>()` is another way to say which type you want. The `::<>` is nicknamed the **turbofish**. Printing the error with `{e}` gives a human-readable message: "invalid digit found in string".

## Returning your own Result

Your functions can return `Result` too. A `String` is the simplest error type to start with:

```rust,runnable title="A function that can fail"
fn check_username(name: &str) -> Result<String, String> {
    if name.is_empty() {
        Err(String::from("username can't be empty"))
    } else {
        Ok(name.to_lowercase())
    }
}

fn main() {
    println!("{:?}", check_username("Ferris"));
    println!("{:?}", check_username(""));
}
```

## A common mistake

A `Result<i32, _>` isn't an `i32`, just as an `Option<i32>` isn't. You have to get the value out first:

```rust,does_not_compile title="Using a Result as a number"
fn main() {
    let n = "5".parse::<i32>();
    let doubled = n * 2;
}
```
