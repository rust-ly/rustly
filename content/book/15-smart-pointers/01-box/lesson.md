+++
id = "book.smart-pointers.box"
chapter = "book.smart-pointers"
requires = []
title = "Box<T>: Data on the Heap"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch15-01-box.html"
summary = "A Box puts a value on the heap and owns it, which makes recursive types like a linked list possible."
+++

# Using `Box<T>` to Point to Data on the Heap

A **pointer** is a variable that holds a memory address: it "points at" some other data. References (`&`) are the simplest pointers; they only borrow. **Smart pointers** are structs that act like pointers but also **own** their data and add abilities. You've already used two: `String` and `Vec<T>` are smart pointers to heap memory.

The simplest smart pointer is `Box<T>`. It puts a value on the **heap**, and the box itself (just a pointer, a fixed size) lives on the stack:

```rust,runnable title="A boxed number"
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}
```

When `b` goes out of scope, both the box and the `5` on the heap are freed. Boxing a single number isn't useful by itself, though. Boxes matter in three situations:

1. A type whose size **can't be known at compile time**, used where a fixed size is needed. Recursive types are the classic case (below).
2. A **large** value you want to move without copying it: moving a box copies only the pointer.
3. Owning a value where you only care that it implements a trait (a **trait object**, chapter 18).

## Recursive types need a box

A **cons list** is a list made of pairs: each item holds a value and the rest of the list. `Cons(1, Cons(2, Cons(3, Nil)))`. Written naively, it doesn't compile:

```rust,does_not_compile title="A type that contains itself"
enum List {
    Cons(i32, List),
    Nil,
}

fn main() {}
```

The error, `E0072: recursive type List has infinite size`, says it all. To decide how much memory a `List` needs, Rust adds up a `Cons`: an `i32` plus a `List`, which is an `i32` plus a `List`... forever.

A `Box<List>` has a known size, one pointer, because the next `List` lives on the heap:

```rust,editable title="A cons list with Box"
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn sum(list: &List) -> i32 {
    match list {
        Cons(value, rest) => value + sum(rest),
        Nil => 0,
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{list:?}");
    println!("sum = {}", sum(&list));
}
```

`use List::{Cons, Nil};` brings the variants into scope so you don't have to write `List::` every time. In `sum`, `rest` is a `&Box<List>`, but it can be passed where a `&List` is expected. That works because of the `Deref` trait, the next concept.
