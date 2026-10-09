+++
id = "book.collections.creating-vectors"
chapter = "book.collections"
requires = []
title = "Creating and Updating Vectors"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch08-01-vectors.html"
summary = "Vec<T>: a growable list of values of one type, built with Vec::new or vec! and grown with push."
+++

# Creating and Updating Vectors

Arrays have a fixed length that's decided when you write the code. Most real lists aren't like that: a shopping list grows as you think of things, and a list of search results depends on the search. For those, Rust has **vectors**, written `Vec<T>`.

A vector stores any number of values, **all of the same type** `T`, next to each other in memory. Its data lives on the heap, so it can grow and shrink while the program runs.

## Making a vector

`Vec::new()` creates an empty vector. Because it starts empty, Rust can't guess what type it will hold, so you write the type:

```rust,runnable title="An empty vector"
fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{v:?} has {} items", v.len());
}
```

More often you start with some values. The `vec!` macro builds a vector from a list, and Rust works out the type from the values:

```rust,runnable title="A vector with values"
fn main() {
    let v = vec![1, 2, 3];
    println!("{v:?} has {} items", v.len());
}
```

## Adding values with `push`

`push` adds a value to the end. Changing a vector means changing the variable, so it has to be declared `mut`:

```rust,editable title="Growing a vector"
fn main() {
    let mut shopping = Vec::new();
    shopping.push("bread");
    shopping.push("milk");
    shopping.push("eggs");
    println!("{shopping:?}");

    let last = shopping.pop();
    println!("removed {last:?}, left with {shopping:?}");
}
```

Here Rust works out that `shopping` is a `Vec<&str>` from the first `push`, so no type annotation is needed. `pop` removes the last item and returns it as an `Option`, because an empty vector has nothing to remove.

When a vector goes out of scope it's dropped, and so are all the values inside it.

## A common mistake

Every element must be the same type. Mixing types doesn't compile:

```rust,does_not_compile title="A number in a vector of strings"
fn main() {
    let mut v = vec!["one", "two"];
    v.push(3);
}
```
