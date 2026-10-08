+++
id = "book.common-concepts.variables"
chapter = "book.common-concepts"
requires = []
title = "Variables and Mutability"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html"
summary = "Variables are immutable by default. Use `mut`, constants and shadowing when you need something else."
+++

# Variables and Mutability

In Rust, `let` creates a variable, and by default that variable is **immutable**: once it has a value, the value can't change. That surprises people coming from most other languages, but it's a deliberate choice. If a value never changes, you don't have to wonder which part of the program changed it.

Try to change one and the compiler stops you:

```rust,does_not_compile title="Assigning twice to an immutable variable"
fn main() {
    let x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is {x}");
}
```

Run it and read the error. The compiler names the line, explains that `x` can't be assigned twice, and suggests the fix.

## Opting in with `mut`

When a value really does need to change, say so with `mut`:

```rust,editable title="Opting in to mutability"
fn main() {
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is {x}");
}
```

`mut` also tells anyone reading the code that this value is going to change, so they know to watch it.

## Constants

A **constant** is declared with `const`, never changes, and needs a type annotation. Constants can live outside any function, and their value must be something the compiler can work out ahead of time. By convention their names are written in `SCREAMING_SNAKE_CASE`.

```rust,runnable title="A constant"
const SECONDS_PER_HOUR: u32 = 60 * 60;

fn main() {
    println!("A day has {} seconds", SECONDS_PER_HOUR * 24);
}
```

## Shadowing

You can declare a new variable with the same name as an earlier one. The new variable **shadows** the old one: from that point on, the name refers to the new value. This isn't the same as `mut`. Each `let` creates a fresh variable, so the new one can even have a different type:

```rust,runnable title="Shadowing"
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("That was {spaces} spaces");
}
```

With `mut`, the second line would fail. A `mut` variable can change its value, but not its type.

## Quick recap

- `let` is immutable by default. Add `mut` when the value has to change.
- `const` is for values fixed at compile time, with an explicit type.
- Shadowing reuses a name for a new value, which can be a new type.

The checkpoint below has a function that won't compile because of mutability. Fix it to unlock the next concept.
