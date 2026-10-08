+++
id = "book.common-concepts.types-and-functions"
chapter = "book.common-concepts"
requires = []
title = "Data Types and Functions"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch03-02-data-types.html"
summary = "Scalar and compound types, plus how functions take parameters and return values."
+++

# Data Types and Functions

Every value in Rust has a type, and the compiler has to know every type before the program runs. Most of the time it works the types out for you. When it can't, or when you want to be explicit, you write the type after a colon: `let count: u32 = 0;`.

## Scalar types

A scalar is a single value. Rust has four kinds:

- **Integers**, signed (`i8` to `i128`, plus `isize`) and unsigned (`u8` to `u128`, plus `usize`). `i32` is the default.
- **Floating-point numbers**: `f64` (the default) and `f32`.
- **Booleans**: `bool`, either `true` or `false`.
- **Characters**: `char`, written in single quotes. A `char` is a full Unicode scalar value, so `'🦀'` works.

## Compound types

A **tuple** groups a fixed number of values that can have different types. An **array** holds a fixed number of values that all have the same type.

```rust,runnable title="Tuples and arrays"
fn main() {
    let point: (i32, f64, char) = (3, 1.5, 'x');
    let (a, b, c) = point;
    println!("a = {a}, b = {b}, c = {c}, first again = {}", point.0);

    let months = ["Jan", "Feb", "Mar"];
    println!("{} months, starting with {}", months.len(), months[0]);
}
```

Arrays are checked at runtime. If you read past the end, the program stops with a **panic** before it touches memory it doesn't own:

```rust,panics title="Indexing past the end of an array"
fn element(values: [i32; 3], index: usize) -> i32 {
    values[index]
}

fn main() {
    let values = [10, 20, 30];
    println!("{}", element(values, 5));
}
```

## Functions

Functions are declared with `fn`. Every parameter needs a type, and a return type comes after `->`. Function and variable names use `snake_case`.

A function body is a series of **statements**, optionally ending in an **expression**. Statements do something and produce no value. Expressions evaluate to a value. The function's last expression is its return value, as long as it **has no semicolon**. Adding a semicolon turns it into a statement:

```rust,does_not_compile title="A stray semicolon"
fn plus_one(x: i32) -> i32 {
    x + 1;
}

fn main() {
    println!("{}", plus_one(5));
}
```

The compiler says the function returns `()`, the empty "unit" value, instead of an `i32`, and it points at the semicolon. Remove it and the function works:

```rust,editable title="Returning the last expression"
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let six = plus_one(5);
    println!("{six}");
}
```

You can still return early with `return value;`, but idiomatic Rust leaves the final value as a bare expression.

Comments start with `//` and run to the end of the line. `///` comments document the item below them.
