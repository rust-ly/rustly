+++
id = "book.functional.capturing"
chapter = "book.functional"
requires = []
title = "Capturing the Environment"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-references-or-moving-ownership"
summary = "Closures can borrow, mutably borrow or take ownership of the variables they use; move forces ownership."
+++

# Capturing References or Moving Ownership

A closure can use variables from the scope where it's defined. It **captures** them in one of three ways, the same three ways a function parameter can take a value. Rust picks the least powerful one that works.

## 1. Borrowing immutably

If the closure only reads a variable, it borrows it. Other code can still read the variable while the closure exists:

```rust,runnable title="A closure that only reads"
fn main() {
    let list = vec![1, 2, 3];
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling: {list:?}");
    only_borrows();
    println!("After calling: {list:?}");
}
```

## 2. Borrowing mutably

If the closure **changes** a variable, it borrows it mutably. The closure itself must then be declared `mut`, and while it exists, nothing else can use the variable (the usual borrowing rules):

```rust,editable title="A closure that changes a variable"
fn main() {
    let mut count = 0;
    let mut increment = || count += 1;
    increment();
    increment();
    increment();
    println!("count = {count}");
}
```

Once `increment` is no longer used, the mutable borrow ends, so the `println!` can read `count`.

## 3. Taking ownership with `move`

Put `move` before the pipes to make the closure **take ownership** of everything it uses, even if it only reads. This matters when the closure must outlive the current scope. The classic case is a new thread, which might keep running after the function that started it has returned:

```rust,runnable title="move into a thread"
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    let handle = thread::spawn(move || println!("From thread: {list:?}"));
    handle.join().unwrap();
}
```

Without `move`, the thread would borrow `list` from `main`, but `main` might finish and drop `list` while the thread is still using it. Rust refuses to compile that. (Threads are chapter 16.)

## A common mistake

After a `move` closure takes a value, the original variable can't be used:

```rust,does_not_compile title="Using a value after moving it into a closure"
fn main() {
    let name = String::from("Ferris");
    let greet = move || println!("Hi, {name}");
    greet();
    println!("{name}");
}
```
