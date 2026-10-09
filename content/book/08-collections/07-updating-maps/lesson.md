+++
id = "book.collections.updating-maps"
chapter = "book.collections"
requires = []
title = "Updating a Hash Map"
track = "book"
order = 7
source = "https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-hash-map"
summary = "Overwrite a value, insert only if missing with entry, and update a value based on the old one."
+++

# Updating a Hash Map

Each key holds one value at a time. When a value is already there, you have three choices, and Rust has a tool for each.

## 1. Overwrite it

Inserting a key that already exists **replaces** the old value:

```rust,runnable title="insert replaces"
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Blue", 25);
    println!("{scores:?}");
}
```

## 2. Insert only if the key is missing

`entry(key)` looks the key up and returns an `Entry`, which knows whether the key was there. `or_insert(value)` inserts the value only if it wasn't, and either way returns a mutable reference to the value now in the map:

```rust,editable title="entry and or_insert"
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);

    scores.entry("Yellow").or_insert(50);
    scores.entry("Blue").or_insert(50);

    println!("{scores:?}");
}
```

Yellow gets 50 because it was new. Blue keeps its 10.

## 3. Update based on the old value

Because `or_insert` returns a `&mut V`, you can change the value through it. This is the classic way to **count** things. Each word starts at 0 the first time it's seen, then goes up by one:

```rust,runnable title="Counting words"
use std::collections::HashMap;

fn main() {
    let text = "hello world wonderful world";
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{counts:?}");
}
```

`count` is a `&mut i32` pointing into the map, so `*count += 1` changes the stored number, just like changing vector elements through `&mut`.

## A common mistake

Forgetting the `*` again: `count` is a reference, and a reference can't be added to.

```rust,does_not_compile title="Adding to the reference"
use std::collections::HashMap;

fn main() {
    let mut counts = HashMap::new();
    let count = counts.entry("hi").or_insert(0);
    count += 1;
}
```
