+++
id = "book.structs.deriving-traits"
chapter = "book.structs"
requires = []
title = "Printing Structs with Debug"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch05-02-example-structs.html"
summary = "Refactor loose values into a struct, and derive Debug to print it."
+++

# An Example Program Using Structs

Here's a function that computes the area of a rectangle from two loose numbers:

```rust
fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

It works, but nothing in the signature says that the two numbers belong together, and it would be easy to swap them at the call site. A tuple `(u32, u32)` groups them, but then `dimensions.0` hides which one is the width. A struct gives the group a name and gives each part a name:

```rust,runnable title="Area of a Rectangle"
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("The area is {} square pixels.", area(&rect));
}
```

`area` borrows the rectangle (`&Rectangle`) because it only needs to read it. `main` keeps ownership and can still use `rect` afterwards.

## Printing a struct

`println!("{}", rect)` doesn't compile. `{}` uses the `Display` trait, and Rust can't guess how you want your own type shown to users. Using `{:?}`, which asks for the **`Debug`** format, doesn't work either until you opt in:

```rust,does_not_compile title="Rectangle doesn't implement Debug"
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("rect is {rect:?}");
}
```

The fix is one line above the struct, `#[derive(Debug)]`. A **derive** asks the compiler to write a standard implementation of a trait for you. Use `{:?}` for a one-line view and `{:#?}` for a pretty-printed one:

```rust,editable title="Deriving Debug"
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("rect is {rect:?}");
    println!("rect is {rect:#?}");
    dbg!(&rect);
}
```

## `dbg!`

`dbg!(expr)` prints the file, the line, the expression and its value to **stderr**, then returns the value. You can wrap it around part of a bigger expression, like `width: dbg!(30 * scale)`, without changing what the code does. Pass a reference, `dbg!(&rect)`, when you don't want it to take ownership.

Next, you'll move `area` inside `Rectangle` itself as a **method**.
