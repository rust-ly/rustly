+++
id = "book.collections.string-indexing"
chapter = "book.collections"
requires = []
title = "Inside a String: Bytes and Characters"
track = "book"
order = 5
source = "https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings"
summary = "Why s[0] doesn't compile, and how to work with a String's characters or bytes instead."
+++

# Indexing into Strings

In many languages, `s[0]` gives the first character of a string. In Rust, it doesn't compile:

```rust,does_not_compile title="Strings can't be indexed"
fn main() {
    let s = String::from("hello");
    let h = s[0];
}
```

This isn't Rust being awkward. It protects you from a real bug, and to see why you need to know how text is stored.

## A String is UTF-8 bytes

A `String` stores bytes in UTF-8. English letters take **1 byte** each, but other characters take more:

```rust,runnable title="Same number of letters, different lengths"
fn main() {
    let english = String::from("Hola");
    let russian = String::from("Здра");
    println!("{english}: {} bytes", english.len());
    println!("{russian}: {} bytes", russian.len());
}
```

Both have four letters, but the Russian one is 8 bytes, because each Cyrillic letter takes 2. So "the byte at position 0" of `"Здра"` is only **half** of `З`. If `s[0]` returned a byte, it would often hand you half a letter; if it returned a character, Rust would have to scan the string from the start each time, which is slow for what looks like a quick operation. Rather than guess, Rust makes you say what you mean.

## Ask for characters or bytes

`.chars()` goes through the **characters**, decoding the UTF-8 for you. `.bytes()` goes through the raw **bytes**:

```rust,editable title="chars and bytes"
fn main() {
    let word = "Зд";
    for c in word.chars() {
        println!("char {c}");
    }
    for b in word.bytes() {
        println!("byte {b}");
    }
    println!("first char: {:?}", word.chars().next());
    println!("number of chars: {}", word.chars().count());
}
```

`chars().next()` is the safe way to get the first character: it returns an `Option<char>`, which is `None` for an empty string.

## Slicing needs character boundaries

You **can** take a slice with a byte range, like `&s[0..4]`. But the range must fall on character boundaries, or the program panics:

```rust,panics title="Slicing through the middle of a letter"
fn main() {
    let hello = "Здравствуйте";
    let s = &hello[0..1];
    println!("{s}");
}
```

Slicing ASCII text is fine. For text that might contain any language, work with `.chars()`.
