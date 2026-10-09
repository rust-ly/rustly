+++
id = "book.enums.if-let"
chapter = "book.enums"
requires = []
title = "if let"
track = "book"
order = 7
source = "https://doc.rust-lang.org/book/ch06-03-if-let.html"
summary = "Run code when a value matches one pattern, without writing out a full match."
+++

# Concise Control Flow with `if let`

Sometimes you only care about **one** pattern. With `match`, you still have to handle every case, so you end up writing a do-nothing arm:

```rust,runnable title="A match with a do-nothing arm"
fn main() {
    let config_max: Option<u8> = Some(3);
    match config_max {
        Some(max) => println!("The maximum is {max}"),
        _ => (),
    }
}
```

That `_ => ()` is boilerplate. `if let` says the same thing more directly:

```rust,runnable title="The same thing with if let"
fn main() {
    let config_max: Option<u8> = Some(3);
    if let Some(max) = config_max {
        println!("The maximum is {max}");
    }
}
```

## Reading `if let`

`if let PATTERN = VALUE { ... }` means: if `VALUE` matches `PATTERN`, bind the pattern's variables and run the block. Otherwise, skip it. The pattern and value are separated by a single `=`, not `==`, because this isn't a comparison but a pattern match, just like one `match` arm.

## Adding an `else`

An `if let` can have an `else`, which runs for every value that didn't match. It's the same as the `_` arm of a `match`:

```rust,editable title="Counting the coins that aren't quarters"
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Quarter(UsState),
}

fn main() {
    let coins = [Coin::Penny, Coin::Quarter(UsState::Alaska), Coin::Penny];
    let mut other = 0;
    for coin in coins {
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {state:?}!");
        } else {
            other += 1;
        }
    }
    println!("{other} other coins");
}
```

## When to use which

`if let` is shorter, but you give up exhaustiveness checking: the compiler won't tell you about cases you forgot. Use `if let` when you genuinely only care about one case. When several cases each need handling, a `match` is clearer and safer.

## A common mistake

`if let` needs a pattern on the left. Writing `==` turns it into a comparison, and comparing a value with a pattern that binds a new name doesn't make sense to the compiler:

```rust,does_not_compile title="== instead of ="
fn main() {
    let x = Some(5);
    if let Some(n) == x {
        println!("{n}");
    }
}
```
