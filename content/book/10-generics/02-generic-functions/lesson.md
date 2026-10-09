+++
id = "book.generics.generic-functions"
chapter = "book.generics"
requires = []
title = "Generic Functions"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch10-01-syntax.html#in-function-definitions"
summary = "Write one function that works for many types, using a type parameter like <T>."
+++

# Generic Functions

Here are two functions with **identical** bodies. Only the types differ:

```rust,runnable title="Two copies, two types"
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("{}", largest_i32(&[34, 50, 25, 100, 65]));
    println!("{}", largest_char(&['y', 'm', 'a', 'q']));
}
```

A **generic** function has a **type parameter**: a placeholder for a type, written in angle brackets after the name. By convention it's a single capital letter, usually `T` (for "type"):

```rust
fn largest<T>(list: &[T]) -> &T {
```

Read it as: "`largest` works for any type `T`. It takes a slice of `T`s and returns a reference to a `T`."

## Not every type can be compared

Try the generic version:

```rust,does_not_compile title="Comparing values of an unknown type"
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("{}", largest(&[34, 50, 25, 100, 65]));
}
```

The error, `E0369: binary operation > cannot be applied to type &T`, makes sense once you think about it. `T` could be **any** type, including ones with no idea of "bigger", like a `File`. The compiler checks generic code for every possible `T`, so it won't allow `>` on an unknown type.

The fix is to promise that `T` can be compared. The compiler even suggests it: `T: PartialOrd`. This is a **trait bound**, and you'll learn exactly what that means later in this chapter:

```rust,editable title="largest for any comparable type"
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("{}", largest(&[34, 50, 25, 100, 65]));
    println!("{}", largest(&['y', 'm', 'a', 'q']));
    println!("{}", largest(&[1.5, 0.25, 9.75]));
}
```

## Generic code is just as fast

Rust doesn't check types at run time to make generics work. When you call `largest` with `i32`s and with `char`s, the compiler writes a separate copy of the function for each type, called **monomorphization**. You write the code once; the program runs as if you'd written each version by hand.
