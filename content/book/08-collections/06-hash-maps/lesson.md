+++
id = "book.collections.hash-maps"
chapter = "book.collections"
requires = []
title = "Storing Keys and Values in Hash Maps"
track = "book"
order = 6
source = "https://doc.rust-lang.org/book/ch08-03-hash-maps.html"
summary = "HashMap<K, V> looks values up by key: create one, insert pairs, get values back and loop over them."
+++

# Storing Keys with Associated Values in Hash Maps

A vector finds things by **position**: item 0, item 1, item 2. Often you want to find things by **name** instead: a team's score, a word's definition, a user's settings. For that, Rust has `HashMap<K, V>`, which stores **keys** of type `K`, each linked to a **value** of type `V`.

Hash maps aren't brought into scope automatically, so start with a `use`:

```rust,runnable title="Team scores"
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{scores:?}");
}
```

Like vectors, hash maps keep their data on the heap, and all keys must be one type and all values one type. Here that's `HashMap<String, i32>`.

## Getting a value

`get` takes a reference to a key and returns an `Option<&V>`: `Some` if the key is there, `None` if not.

```rust,editable title="Looking up a score"
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team = String::from("Blue");
    let score = scores.get(&team).copied().unwrap_or(0);
    println!("{team}: {score}");
    println!("Red: {}", scores.get("Red").copied().unwrap_or(0));
}
```

`.copied()` turns the `Option<&i32>` into an `Option<i32>`, and `.unwrap_or(0)` gives the value, or `0` when there isn't one. You'll use this pair a lot.

## Looping over a map

A `for` loop over `&map` gives each key and value as a pair. The order is **arbitrary**: hash maps don't keep things in insertion order or sorted order, and it can change from run to run.

```rust,runnable title="Every key and value"
use std::collections::HashMap;

fn main() {
    let mut capitals = HashMap::new();
    capitals.insert("Kenya", "Nairobi");
    capitals.insert("Japan", "Tokyo");
    capitals.insert("Peru", "Lima");
    for (country, city) in &capitals {
        println!("{city} is the capital of {country}");
    }
}
```

## Hash maps take ownership

Owned values like `String` are **moved** into the map when you insert them. After `insert`, the original variables can't be used:

```rust,does_not_compile title="Using a key after inserting it"
use std::collections::HashMap;

fn main() {
    let name = String::from("Favourite colour");
    let value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(name, value);
    println!("{name}");
}
```

Copy types like `i32` are copied in instead, so they stay usable.
