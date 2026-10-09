+++
id = "book.patterns.literals-ranges"
chapter = "book.patterns"
requires = []
title = "Literals, Variables, | and Ranges"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html"
summary = "Match exact values, several values with |, and whole ranges with ..=, and watch out for variables that shadow."
+++

# Pattern Syntax: Literals, Multiple Patterns and Ranges

## Literals

A pattern can be an exact value:

```rust,runnable title="Matching literals"
fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
```

## Several patterns with `|`

`|` means **or**: the arm runs if any of its patterns match.

## Ranges with `..=`

`start..=end` matches every value in the range, ends included. Ranges work for numbers and `char`s:

```rust,editable title="| and ranges"
fn describe(n: u32) -> &'static str {
    match n {
        0 => "none",
        1 | 2 => "a couple",
        3..=9 => "a few",
        _ => "lots",
    }
}

fn kind(c: char) -> &'static str {
    match c {
        'a'..='z' => "lower case letter",
        'A'..='Z' => "upper case letter",
        '0'..='9' => "digit",
        _ => "something else",
    }
}

fn main() {
    for n in [0, 2, 5, 40] {
        println!("{n}: {}", describe(n));
    }
    for c in ['q', 'Q', '7', '!'] {
        println!("{c}: {}", kind(c));
    }
}
```

The compiler checks ranges at compile time, which is why only number and `char` ranges are allowed, and an empty range like `9..=3` is an error.

## Watch out: a name is a new variable

A **name** in a pattern doesn't compare against an existing variable with that name. It creates a **new** variable that matches anything and shadows the old one inside the arm:

```rust,runnable title="Shadowing in a match arm"
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}
```

It prints `Matched, y = 5`, not "Default case". The `y` in `Some(y)` is a fresh variable bound to `5`; it has nothing to do with the outer `y = 10`. To compare against the outer `y`, you need a **match guard**, coming two concepts from now.
