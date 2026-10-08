+++
id = "book.structs.methods"
chapter = "book.structs"
requires = []
title = "Methods"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch05-03-method-syntax.html"
summary = "Attach behaviour to a struct with impl blocks, self, and associated functions."
+++

# Method Syntax

A **method** is a function defined inside an `impl` block for a type. Its first parameter is always `self`, the instance the method was called on. You call it with dot syntax, `rect.area()`, instead of `area(&rect)`.

```rust,runnable title="area as a method"
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("The area of {rect:?} is {}", rect.area());
}
```

`&self` is short for `self: &Self`, and inside an `impl Rectangle` block, `Self` means `Rectangle`. The method borrows the instance, just like the `area(&Rectangle)` function in the previous concept. There are three choices:

- `&self` when the method only reads. This is the most common.
- `&mut self` when it changes the instance.
- `self` when it consumes the instance. This is rare, and is usually for turning a value into something else.

You don't have to write `(&rect).area()` or `(&mut rect).grow()`. Rust adds the `&` or `&mut` for you, based on what the method asks for.

## Methods with more parameters

Other parameters come after `self`. Here, `can_hold` compares two rectangles:

```rust,editable title="A method that takes another Rectangle"
# struct Rectangle {
#     width: u32,
#     height: u32,
# }
#
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let big = Rectangle { width: 30, height: 50 };
    let small = Rectangle { width: 10, height: 40 };
    println!("big can hold small? {}", big.can_hold(&small));
    println!("small can hold big? {}", small.can_hold(&big));
}
```

## Associated functions

A function in an `impl` block that has **no** `self` parameter is an **associated function**. You call it with `::` on the type, as in `String::from`. Associated functions are often constructors, and `new` is the conventional name, though Rust doesn't treat it specially:

```rust,runnable title="An associated function"
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    let sq = Rectangle::square(3);
    println!("{sq:?}");
}
```

A type can have several `impl` blocks. You can also give a method the same name as a field, like `fn width(&self)`: `rect.width()` calls the method and `rect.width` reads the field. That's how **getters** are written.
