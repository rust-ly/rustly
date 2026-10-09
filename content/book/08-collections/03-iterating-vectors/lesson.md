+++
id = "book.collections.iterating-vectors"
chapter = "book.collections"
requires = []
title = "Iterating Over Vectors"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch08-01-vectors.html#iterating-over-the-values-in-a-vector"
summary = "Loop over a vector by reference, change every element through &mut, and use an enum to mix kinds of value."
+++

# Iterating Over the Values in a Vector

To do something with every element, loop over the vector with `for`. Loop over `&v` to **borrow** each element in turn:

```rust,runnable title="Reading every element"
fn main() {
    let v = vec![100, 32, 57];
    for n in &v {
        println!("{n}");
    }
    println!("still usable: {v:?}");
}
```

Looping over `v` itself (no `&`) would **move** the vector into the loop, and you couldn't use it afterwards. Borrowing is usually what you want.

## Changing every element

Loop over `&mut v` to get a mutable reference to each element. To change the value a reference points to, **dereference** it with `*`:

```rust,editable title="Adding 50 to every element"
fn main() {
    let mut v = vec![100, 32, 57];
    for n in &mut v {
        *n += 50;
    }
    println!("{v:?}");
}
```

`n` is a `&mut i32`, a pointer to the element. `*n` means "the value it points to", so `*n += 50` changes the element itself. (You'll see much more of `*` in chapter 15.)

The borrow checker protects loops too: you can't push to or remove from a vector while looping over it, for the same reason as in the last concept.

## Storing different kinds of value with an enum

A vector holds one type. When you need a list of different kinds of thing, define an enum whose variants hold the different kinds. Each cell of a spreadsheet row might be a number, a decimal or some text:

```rust,runnable title="A spreadsheet row"
#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        Cell::Int(3),
        Cell::Text(String::from("blue")),
        Cell::Float(10.12),
    ];
    for cell in &row {
        match cell {
            Cell::Int(i) => println!("whole number {i}"),
            Cell::Float(f) => println!("decimal {f}"),
            Cell::Text(s) => println!("text {s:?}"),
        }
    }
}
```

The vector's type is `Vec<Cell>`, so it's still one type, and `match` makes sure every kind of cell is handled.

## A common mistake

Forgetting the `*` when changing through a reference. You can't add a number to a reference itself:

```rust,does_not_compile title="Missing the dereference"
fn main() {
    let mut v = vec![1, 2, 3];
    for n in &mut v {
        n += 1;
    }
}
```
