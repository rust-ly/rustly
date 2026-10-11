+++
id = "book.advanced.macros"
chapter = "book.advanced"
requires = []
title = "Macros"
track = "book"
order = 6
source = "https://doc.rust-lang.org/book/ch20-05-macros.html"
summary = "Write declarative macros with macro_rules!: match on syntax, capture fragments, and repeat with $(...),*. Plus a tour of procedural macros."
+++

# Macros

## Macros versus functions

You've used macros since chapter 1: `println!`, `vec!`, `assert_eq!`. A macro is **code that writes code**. The compiler expands it into ordinary Rust before type checking. That lets macros do things functions can't:

- take a **variable number of arguments** (`println!("{}", a)` and `println!("{} {}", a, b)`)
- run at compile time, so they can generate items like trait implementations

The cost is that macro definitions are harder to read and write than functions, and a macro must be defined (or imported) **before** it's used in a file.

## Declarative macros with `macro_rules!`

A declarative macro is a `match` on **syntax**. Each arm has a pattern and the code it expands to. This one has a single arm that takes no input:

```rust,runnable title="The simplest macro"
macro_rules! hello {
    () => {
        println!("Hello from a macro!");
    };
}

fn main() {
    hello!();
}
```

Patterns capture pieces of syntax with `$name:fragment`. The fragment says what kind of syntax to accept. The common ones are `expr` (an expression), `ident` (a name), `ty` (a type) and `literal`:

```rust,editable title="Capturing an expression"
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

fn main() {
    println!("{}", square!(7));
    println!("{}", square!(2 + 1));
}
```

Run it and look at the second line. `square!(2 + 1)` prints **9**, not 5, because an `expr` fragment is kept together as one expression when it's substituted. (A C-style text macro would have produced `2 + 1 * 2 + 1`.)

A macro can have several arms, and they're tried in order, just like `match`. An arm for `()`, then one for `($x:literal)`, then one for `($x:ident)` lets the same macro accept nothing, a literal or a name.

## Repetition: `$( ... ),*`

This is what makes `vec!` possible. `$( $x:expr ),*` matches **zero or more** expressions separated by commas. In the expansion, `$( ... )*` repeats its body once per match. Here is a simplified `vec!`:

```rust,runnable title="Building our own vec!"
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v: Vec<i32> = my_vec![1, 2, 3];
    println!("{v:?}");
}
```

`my_vec![1, 2, 3]` expands to three `push` calls. Use `+` instead of `*` for **one or more**, and add `$(,)?` at the end of the pattern to allow a trailing comma.

If no arm matches, it's a compile error, the same way a non-exhaustive `match` is:

```rust,does_not_compile title="No arm matches"
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

fn main() {
    println!("{}", square!(1, 2));
}
```

## Procedural macros

The other kind, **procedural macros**, are Rust functions that take a stream of tokens and return a new one. They live in their own crate with `proc-macro = true`, so they can't run in a single-file Playground. You've used them already:

- **Custom `derive`**: `#[derive(Debug, Clone, PartialEq)]` generates trait implementations for a struct or enum. Crates like `serde` add their own (`#[derive(Serialize)]`).
- **Attribute-like**: `#[route(GET, "/")]` on a function, as in web frameworks. `#[tokio::main]` from chapter 17 is one.
- **Function-like**: `sql!(SELECT * FROM posts)`, which looks like a `macro_rules!` call but can parse anything.

For most everyday needs, `macro_rules!` is enough. Reach for a procedural macro when you need to generate code from the **structure** of a type, which is exactly what `derive` does.
