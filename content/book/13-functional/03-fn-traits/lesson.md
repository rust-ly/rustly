+++
id = "book.functional.fn-traits"
chapter = "book.functional"
requires = []
title = "The Fn Traits"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch13-01-closures.html#moving-captured-values-out-of-closures-and-the-fn-traits"
summary = "FnOnce, FnMut and Fn describe what a closure does with its captures, and let functions accept closures."
+++

# The `Fn` Traits

How a closure treats the values it captures decides which of three **traits** it implements. Functions that accept closures use these traits as bounds (chapter 10), to say what kind of closure they can handle.

| Trait | The closure... | Can be called |
| --- | --- | --- |
| `FnOnce` | may **move** a captured value out | once |
| `FnMut` | may **change** captured values | many times |
| `Fn` | only **reads** captured values (or captures nothing) | many times, even at once |

Every closure implements `FnOnce`. Those that don't move values out also implement `FnMut`, and those that don't change anything also implement `Fn`. So `Fn` is the most restrictive for the closure, and the most flexible for the code calling it.

## Accepting a closure

Write a generic parameter with an `Fn` bound. Here `F: Fn(i32) -> i32` means "any closure (or function) that takes an `i32` and returns an `i32`":

```rust,runnable title="A function that takes a closure"
fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    let add_three = |n| n + 3;
    println!("{}", apply_twice(add_three, 10));
    println!("{}", apply_twice(|n| n * n, 3));
}
```

Ordinary functions work too: `apply_twice(some_fn, 10)`. A function is like a closure that captures nothing.

## `FnMut` in the standard library

`sort_by_key` sorts a slice by a key that a closure picks out of each item. It calls the closure many times, so it needs `FnMut`. Here the closure also **counts** how often it's called, which a plain `Fn` couldn't do:

```rust,editable title="sort_by_key"
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    let mut calls = 0;
    list.sort_by_key(|r| {
        calls += 1;
        r.width
    });
    println!("{list:#?}");
    println!("sorted with {calls} calls");
}
```

## Why not `FnOnce` everywhere?

`unwrap_or_else` takes an `FnOnce`, because it calls the closure at most once. `sort_by_key` can't: a closure that moved a value out would have nothing left to give on the second call, so passing one fails:

```rust,does_not_compile title="An FnOnce closure where FnMut is needed"
#[derive(Debug)]
struct Rectangle {
    width: u32,
}

fn main() {
    let mut list = [Rectangle { width: 10 }, Rectangle { width: 3 }];
    let mut sort_operations = vec![];
    let value = String::from("closure called");
    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
}
```

The closure moves `value` into the vector, so it could only run once. The error explains that the closure is `FnOnce` but `FnMut` was required.
