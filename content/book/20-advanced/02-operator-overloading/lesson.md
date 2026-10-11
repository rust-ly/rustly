+++
id = "book.advanced.operator-overloading"
chapter = "book.advanced"
requires = []
title = "Associated Types and Operator Overloading"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch20-02-advanced-traits.html"
summary = "Traits with associated types, default type parameters, and making + and * work on your own types."
+++

# Associated Types and Operator Overloading

## Associated types

You met one in chapter 13: `Iterator` has `type Item;`. An **associated type** is a placeholder type inside a trait that each implementation fills in.

Why not a generic, `trait Iterator<T>`? With a generic, a type could implement `Iterator<u32>` **and** `Iterator<String>`, and every call to `next` would need an annotation to say which. With an associated type, each type implements the trait **once**, with **one** `Item`, so no annotations are needed.

## Operator overloading

Operators like `+` and `*` are traits in `std::ops`. Implement `Add`, and `+` works on your type:

```rust,runnable title="Adding two Points"
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let p = Point { x: 1, y: 0 } + Point { x: 2, y: 3 };
    println!("{p:?}");
}
```

`type Output` is an associated type: the result of `+`. Here it's a `Point`, but it doesn't have to be.

## Default type parameters

`Add` is defined as `trait Add<Rhs = Self>`. The `Rhs` ("right-hand side") type parameter has a **default**, `Self`, so plain `impl Add for Point` means "add a `Point` to a `Point`". To add a **different** type on the right, fill it in:

```rust,editable title="Adding metres to millimetres"
use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    println!("{:?}", Millimeters(250) + Meters(2));
}
```

Other operator traits work the same way: `Sub` (`-`), `Mul` (`*`), `Neg` (unary `-`), `AddAssign` (`+=`), `Index` (`[]`) and more. Only overload an operator when the meaning is obvious. `+` on points or money is clear; `+` on users would be confusing.

## A common mistake

Without an `Add` implementation, `+` on your type doesn't compile:

```rust,does_not_compile title="No Add for this type"
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 } + Point { x: 3, y: 4 };
}
```
