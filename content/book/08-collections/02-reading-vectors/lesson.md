+++
id = "book.collections.reading-vectors"
chapter = "book.collections"
requires = []
title = "Reading Elements of Vectors"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch08-01-vectors.html#reading-elements-of-vectors"
summary = "Index with [] when the item must exist, use get when it might not, and mind the borrow rules."
+++

# Reading Elements of Vectors

There are two ways to read an element, and they behave differently when the element doesn't exist.

## Indexing with `[]`

`v[i]` gives the element at position `i`, counting from 0. `&v[i]` borrows it instead of copying it out:

```rust,runnable title="Reading by index"
fn main() {
    let v = vec![10, 20, 30, 40, 50];
    let third = &v[2];
    println!("The third element is {third}");
}
```

## `get` returns an Option

`v.get(i)` returns `Some(&element)` if the index exists and `None` if it doesn't. You handle both cases, usually with a `match` or `if let`:

```rust,editable title="Reading with get"
fn main() {
    let v = vec![10, 20, 30, 40, 50];
    for i in [2, 100] {
        match v.get(i) {
            Some(value) => println!("v[{i}] is {value}"),
            None => println!("there is no v[{i}]"),
        }
    }
}
```

## Which one to use

The difference shows when the index is out of range. `get` returns `None` and the program carries on. Indexing **panics**, crashing the program:

```rust,panics title="Indexing past the end"
fn main() {
    let v = vec![10, 20, 30];
    let does_not_exist = v[100];
    println!("{does_not_exist}");
}
```

So: use `[]` when an out-of-range index would be a bug you want to hear about loudly. Use `get` when a missing element is a normal situation, like a user typing a number that's too big.

## Borrowing and pushing don't mix

A reference to an element is a borrow of the whole vector. While you hold it, you can't change the vector:

```rust,does_not_compile title="Pushing while holding a reference"
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4);
    println!("The first element is {first}");
}
```

Why should adding to the **end** affect the first element? Because a vector keeps its items side by side. If there's no room left, `push` moves everything to a bigger block of memory, and `first` would point at the old, freed block. The borrow checker stops that from happening.
