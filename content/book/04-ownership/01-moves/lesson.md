+++
id = "book.ownership.moves"
chapter = "book.ownership"
requires = []
title = "Moves"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html"
summary = "Who owns a value, and what happens when it moves."
+++

# Ownership and Moves

Ownership is the idea that makes Rust different from other languages. It lets Rust free memory at exactly the right time without a garbage collector, and the compiler checks the rules for you.

There are three rules:

1. Every value has an **owner**, the variable that holds it.
2. A value has **one owner at a time**.
3. When the owner goes **out of scope**, the value is dropped and its memory is freed.

## Stack values are copied

Small fixed-size values, like integers, `bool`, `char` and tuples made of them, live on the stack and are cheap to copy. Assigning one to another variable just copies it, and both variables stay usable. These types implement the `Copy` trait.

## Heap values move

A `String` is different. Its text lives on the heap, and the variable holds a pointer to it. If assigning `s1` to `s2` copied only that pointer, two owners would try to free the same memory when they went out of scope. So instead, Rust **moves** the value: `s2` becomes the owner, and `s1` can't be used any more.

```rust,does_not_compile title="Using a value after a move"
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}, world!");
}
```

The error is `E0382: borrow of moved value`. The compiler points at the line where the move happened and at the line where you used the moved value.

If you want two independent copies, ask for one explicitly with `.clone()`. It copies the heap data, which can be slow for big values, and that's why Rust makes you write it out:

```rust,editable title="Fix it with a clone"
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{s1}, world! ({s2})");
}
```

## Functions move too

Passing a value to a function works the same way as assigning it. A `String` argument moves into the function, and the caller loses it. Returning a value moves ownership back out:

```rust,runnable title="Ownership in and out of functions"
fn takes_and_gives_back(s: String) -> String {
    println!("inside: {s}");
    s
}

fn main() {
    let a = String::from("moving");
    let b = takes_and_gives_back(a);
    // `a` was moved into the function; `b` owns the returned String.
    println!("outside: {b}");
}
```

Handing values back and forth like this gets tedious fast. The next concept, **borrowing**, lets a function use a value without taking ownership of it.
