+++
id = "book.functional.iterators"
chapter = "book.functional"
requires = []
title = "Iterators and the Iterator Trait"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch13-02-iterators.html"
summary = "An iterator hands out items one at a time through next(); iter, iter_mut and into_iter make them from collections."
+++

# Processing a Series of Items with Iterators

An **iterator** produces a sequence of items, one at a time. You've used them in every `for` loop: `for x in &v` asks `v` for an iterator and pulls items from it until there are none left.

Iterators are **lazy**: creating one does nothing until something asks for items.

```rust,runnable title="Creating and using an iterator"
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {val}");
    }
}
```

## The `Iterator` trait and `next`

Every iterator implements the `Iterator` trait from the standard library. It has one required method, `next`, which returns the next item wrapped in `Some`, or `None` when the sequence is over:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ...and dozens of default methods built on next
}
```

`type Item` is an **associated type**: each iterator says what kind of item it produces. You can call `next` yourself to see exactly what a `for` loop does behind the scenes:

```rust,editable title="Calling next by hand"
fn main() {
    let v = vec![10, 20, 30];
    let mut it = v.iter();
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
}
```

`next` changes the iterator's internal position, so the iterator must be `mut`. (A `for` loop takes ownership of the iterator and makes it mutable for you.)

## Three ways to iterate a collection

- `iter()` gives **references** to each item (`&T`). The collection is untouched.
- `iter_mut()` gives **mutable references** (`&mut T`), to change items in place.
- `into_iter()` **takes ownership** of the collection and gives the items themselves (`T`).

## Writing your own iterator

Implement `Iterator` on a struct: say what `Item` is and write `next`. Then every default method, and `for` loops, work with it:

```rust,runnable title="A countdown iterator"
struct Countdown {
    n: u32,
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.n == 0 {
            None
        } else {
            self.n -= 1;
            Some(self.n + 1)
        }
    }
}

fn main() {
    for n in (Countdown { n: 3 }) {
        println!("{n}...");
    }
    let total: u32 = Countdown { n: 4 }.sum();
    println!("liftoff! (4+3+2+1 = {total})");
}
```

`sum` is one of the default methods. It's a **consuming adapter**: it calls `next` until the iterator runs out, using it up.
