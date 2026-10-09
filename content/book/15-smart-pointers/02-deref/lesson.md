+++
id = "book.smart-pointers.deref"
chapter = "book.smart-pointers"
requires = []
title = "The Deref Trait"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch15-02-deref.html"
summary = "Implementing Deref makes * work on your type, and lets Rust convert &YourType to &Target automatically."
+++

# Treating Smart Pointers Like Regular References with `Deref`

The `*` operator **dereferences** a pointer: it follows it to the value. It works on references, and on boxes too:

```rust,runnable title="* on a reference and on a Box"
fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    println!("both point at 5");
}
```

`Box` isn't built into the language like `&` is. `*` works on it because `Box` implements the **`Deref` trait**. Your own types can do the same.

## Building our own box

`MyBox` holds one value in a tuple struct. To make `*` work, implement `Deref`: say what it points to (`Target`) and return a reference to it:

```rust,editable title="Implementing Deref"
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let y = MyBox::new(5);
    assert_eq!(5, *y);
    println!("*y works: {}", *y);
}
```

Behind the scenes, `*y` becomes `*(y.deref())`: Rust calls `deref` to get a plain reference, then dereferences that. (`deref` returns a reference rather than the value so the value isn't moved out of the box.)

## Deref coercion

**Deref coercion** is a convenience that makes this pay off. When you pass a reference to a type that implements `Deref` where a different reference type is expected, Rust inserts `deref` calls until the types match:

```rust,runnable title="&MyBox<String> becomes &str"
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox(String::from("Rust"));
    hello(&m); // &MyBox<String> -> &String -> &str
}
```

`&m` is a `&MyBox<String>`. Deref turns it into `&String`, and because `String` also implements `Deref` (to `str`), into `&str`. Without coercion you'd have to write `hello(&(*m)[..])`.

This happens at compile time, so it costs nothing. It's also why `&String` works wherever `&str` is expected, and `&Vec<T>` wherever `&[T]` is. For mutable references, `DerefMut` does the same job.
