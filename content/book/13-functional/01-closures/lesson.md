+++
id = "book.functional.closures"
chapter = "book.functional"
requires = []
title = "Closures"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch13-01-closures.html"
summary = "An anonymous function you can store in a variable or pass to another function, written |args| body."
+++

# Closures: Anonymous Functions

A **closure** is a function without a name that you can store in a variable or pass to another function. You've already seen a few, like `|err| { ... }` in minigrep. Now they get a proper introduction.

```rust,runnable title="A closure in a variable"
fn main() {
    let add_one = |x: i32| x + 1;
    println!("{}", add_one(5));
}
```

The parameters go between pipes, `|x: i32|`, then comes the body. For a one-line body you don't need braces.

## Closure syntax, from long to short

These all do the same thing. The function is shown first for comparison:

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

Functions must spell out their types, because they're part of an interface others rely on. Closures are short and local, so Rust **infers** their parameter and return types from how they're used. But each closure gets one set of types: call it with an `i32` once, and it only accepts `i32`s from then on.

## Passing a closure to a function

The power of closures is that you can hand behaviour to other code. In the Book, a T-shirt company gives away shirts: you get your favourite colour if you have one, and otherwise the colour they have most of. `Option::unwrap_or_else` takes a closure that runs **only** when the option is `None`:

```rust,editable title="unwrap_or_else with a closure"
#[derive(Debug, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

fn most_stocked(red: u32, blue: u32) -> ShirtColor {
    if red > blue { ShirtColor::Red } else { ShirtColor::Blue }
}

fn main() {
    let (red, blue) = (2, 5);

    let pref = Some(ShirtColor::Red);
    let shirt = pref.unwrap_or_else(|| most_stocked(red, blue));
    println!("with a preference: {shirt:?}");

    let no_pref: Option<ShirtColor> = None;
    let shirt = no_pref.unwrap_or_else(|| most_stocked(red, blue));
    println!("without one: {shirt:?}");
}
```

`|| most_stocked(red, blue)` takes no parameters (the pipes are empty). Notice that it uses `red` and `blue` from the surrounding function: closures can **capture** variables from where they're defined, which ordinary functions can't do. That's the next concept.

## A common mistake

Because a closure's types are fixed by its first use, calling it with a different type fails:

```rust,does_not_compile title="One closure, two types"
fn main() {
    let example = |x| x;
    let s = example(String::from("hello"));
    let n = example(5);
}
```
