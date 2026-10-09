+++
id = "book.collections.strings"
chapter = "book.collections"
requires = []
title = "Creating and Updating Strings"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch08-02-strings.html"
summary = "String is a growable collection of UTF-8 text: build it, append to it, and join strings together."
+++

# Storing UTF-8 Text with Strings

You've used strings since chapter 1, but they're more complicated than they look, so they get a proper look here. A `String` is a **collection**: a growable, heap-allocated sequence of bytes that always holds valid **UTF-8** text.

Rust has two main string types:

- `String`: owned and growable. You can change it.
- `&str`, a **string slice**: a borrowed view into some text. String literals like `"hello"` are `&str`.

Many operations from `Vec<T>` work on `String` too, because a `String` is really a wrapper around a vector of bytes.

## Creating a String

```rust,runnable title="Three ways to make a String"
fn main() {
    let a = String::new();              // empty
    let b = "initial contents".to_string();
    let c = String::from("initial contents");
    println!("{a:?} {b:?} {c:?}");
}
```

`to_string` and `String::from` do the same thing; pick whichever reads better. Because strings are UTF-8, any language works: `String::from("こんにちは")`, `String::from("Здравствуйте")` and `String::from("👋")` are all valid.

## Growing a String

`push_str` appends a string slice, and `push` appends a single `char`:

```rust,editable title="push_str and push"
fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("{s}");
}
```

`push_str` takes a `&str`, so it doesn't take ownership of what you pass in. You can keep using it afterwards.

## Joining strings with `+` and `format!`

`+` joins two strings, but it has a quirk: the left side must be a `String`, which is **moved**, and the right side must be a `&str`:

```rust,runnable title="+ moves the left-hand String"
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved here; s2 is only borrowed
    println!("{s3}");
    println!("{s2} is still usable");
}
```

For more than two pieces, `+` gets hard to read. `format!` works like `println!` but returns the `String` instead of printing it, and it doesn't take ownership of anything: `let s = format!("{s1}-{s2}-{s3}");` leaves all three strings usable.

## A common mistake

Using a `String` after it has been moved by `+`:

```rust,does_not_compile title="Using s1 after +"
fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s = s1 + "-" + &s2;
    println!("{s1}");
}
```
