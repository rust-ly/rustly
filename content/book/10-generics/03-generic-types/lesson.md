+++
id = "book.generics.generic-types"
chapter = "book.generics"
requires = []
title = "Generic Structs, Enums and Methods"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch10-01-syntax.html#in-struct-definitions"
summary = "Structs and enums with type parameters, and impl blocks that work for every type or just one."
+++

# Generic Structs, Enums and Methods

Type parameters work on structs too. Declare them after the struct's name, then use them as field types:

```rust,runnable title="A Point of any number type"
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} {float:?}");
}
```

`Point<T>` has **one** type parameter, so `x` and `y` must be the same type. `Point { x: 5, y: 4.0 }` wouldn't compile. If you want them to differ, use two: `struct Point<T, U> { x: T, y: U }`.

## You've already used generic enums

`Option` and `Result` are generic enums from the standard library:

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

That's why `Option<i32>`, `Option<String>` and `Result<u8, ParseIntError>` all exist: one definition, any types.

## Methods on generic types

To write methods for every `Point<T>`, declare `T` after `impl` too. `impl<T>` says "these methods work for any `T`":

```rust,editable title="Methods for every Point, and for one"
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Only Points of f64 get this method.
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("x = {}, distance = {}", p.x(), p.distance_from_origin());

    let q = Point { x: 'a', y: 'b' };
    println!("x = {}", q.x());
}
```

The second block has no `<T>` after `impl`: it's for the concrete type `Point<f64>` only. Calling `distance_from_origin` on a `Point<char>` won't compile, which is right, because square roots of letters make no sense.

## A common mistake

Forgetting `<T>` after `impl` makes Rust think `T` is the name of a real type:

```rust,does_not_compile title="impl without declaring T"
struct Point<T> {
    x: T,
    y: T,
}

impl Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {}
```
