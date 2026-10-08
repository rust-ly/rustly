+++
id = "book.ownership.slices"
chapter = "book.ownership"
requires = []
title = "Slices"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch04-03-slices.html"
summary = "Borrow part of a string or array with a slice, and let the compiler keep it valid."
+++

# The Slice Type

A **slice** is a reference to a run of elements inside a collection, rather than the whole collection. Like any reference, a slice doesn't own anything.

## String slices

`&s[start..end]` borrows the bytes from `start` up to, but not including, `end`. You can leave out either end: `&s[..5]` starts at the beginning, `&s[6..]` goes to the end, and `&s[..]` is the whole string.

```rust,runnable title="Slicing a String"
fn main() {
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("[{hello}] [{world}]");
}
```

The type of a string slice is `&str`. String literals are slices too: `"hi"` is a `&str` that points into the program's binary. That's why a parameter of type `&str` accepts a literal, a whole `String` (`&s`) and a slice of one (`&s[2..]`).

The indexes count **bytes**, not characters. Slicing in the middle of a multi-byte character like `é` panics, so plain-ASCII text is the safe case for now. Iterating with `.chars()` or `.char_indices()` handles everything else.

## Why slices are better than indexes

Say you want the first word of a string. You could return the index where it ends, but that number has no connection to the string. Clear the string and the index silently becomes wrong. A slice stays tied to its string, so the borrow checker keeps it valid:

```rust,does_not_compile title="Clearing a string while a slice is alive"
fn first_word(s: &str) -> &str {
    s.split(' ').next().unwrap_or("")
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("the first word is: {word}");
}
```

`word` borrows `s` immutably, and `clear` needs a mutable borrow while `word` is still in use. That's the same rule as in the borrowing concept, and here it catches a real bug.

## Other slices

Slices work on arrays and vectors as well. A slice of `i32` values has the type `&[i32]`:

```rust,editable title="An array slice"
fn total(values: &[i32]) -> i32 {
    let mut sum = 0;
    for v in values {
        sum += v;
    }
    sum
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    println!("middle three: {}", total(&numbers[1..4]));
    println!("all: {}", total(&numbers));
}
```

To scan a string byte by byte, `s.as_bytes()` gives you a `&[u8]`, and `.iter().enumerate()` gives you each byte with its index. In the checkpoint you'll use that to write `first_word` yourself.
