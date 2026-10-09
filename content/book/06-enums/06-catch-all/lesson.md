+++
id = "book.enums.catch-all"
chapter = "book.enums"
requires = []
title = "Exhaustive Matches and Catch-alls"
track = "book"
order = 6
source = "https://doc.rust-lang.org/book/ch06-02-match.html#matches-are-exhaustive"
summary = "A match must cover every case. Use a catch-all or _ for the cases you don't list."
+++

# Exhaustive Matches and Catch-alls

A `match` must handle **every** possible value. If you leave one out, the program doesn't compile. This rule is called **exhaustiveness**, and it's one of Rust's best safety nets.

```rust,does_not_compile title="Forgetting the None case"
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("{:?}", plus_one(Some(5)));
}
```

The error, `E0004: non-exhaustive patterns: None not covered`, tells you exactly which case is missing. Here, that's the "forgot about null" bug from the `Option` concept, caught before the program ever runs.

It also helps when code changes. Add a variant to an enum, and the compiler points at every `match` that now needs a new arm.

## Catch-all with a name

Some values have too many cases to list, like every possible `u8`. Finish the `match` with a plain variable name: it matches **anything** that the arms above didn't, and binds the value:

```rust,runnable title="A board game roll"
fn main() {
    let roll: u8 = 9;
    match roll {
        3 => println!("You get a fancy hat"),
        7 => println!("You lose your fancy hat"),
        other => println!("Move forward {other} spaces"),
    }
}
```

## `_` when you don't need the value

If the catch-all doesn't use the value, write `_`. It matches anything and **doesn't** bind it, so Rust won't warn about an unused variable:

```rust,editable title="Ignoring the rest with _"
fn main() {
    for roll in [3, 7, 9] {
        match roll {
            3 => println!("{roll}: fancy hat!"),
            7 => println!("{roll}: hat lost"),
            _ => println!("{roll}: roll again"),
        }
    }
}
```

To do **nothing** for the other cases, give `_` the empty value `()`: `_ => ()`.

## The catch-all goes last

Arms are tried in order, and a catch-all matches everything. Put it first and the arms below it can never run. Rust warns you that they're `unreachable`:

```rust,runnable title="A catch-all in the wrong place"
fn main() {
    let roll = 3;
    match roll {
        _ => println!("roll again"),
        3 => println!("fancy hat"),
    }
}
```

Run it: it prints "roll again" even though the roll is 3, and the compiler output warns about the unreachable pattern.
