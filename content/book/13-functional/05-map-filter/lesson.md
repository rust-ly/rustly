+++
id = "book.functional.map-filter"
chapter = "book.functional"
requires = []
title = "map, filter and collect"
track = "book"
order = 5
source = "https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-produce-other-iterators"
summary = "Transform every item with map, keep some with filter, and gather the results with collect."
+++

# Methods That Produce Other Iterators

**Iterator adapters** are methods that turn one iterator into another, changing the items along the way. Adapters are lazy, so a chain of them does nothing until it's **consumed**.

## `map`: transform every item

`map` takes a closure and calls it on each item, producing the results:

```rust,does_not_compile title="Forgetting to consume the iterator"
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1);
}
```

That fails because `map` returns another **iterator**, not a vector. (Write it without the `: Vec<i32>` and Rust instead warns that the unused `map` does nothing: iterators are lazy.)

## `collect`: gather the results

`collect` consumes the iterator and builds a collection. Rust needs to know **which** collection, so give the variable a type:

```rust,runnable title="map then collect"
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("{v2:?}");
}
```

`collect` can build many kinds of collection: a `Vec`, a `String` from `char`s, a `HashMap` from `(key, value)` pairs.

## `filter`: keep some items

`filter` takes a closure that returns a `bool` and keeps only the items where it's `true`. The closure can capture variables from its surroundings, which makes filters easy to customise. Here's the Book's shoe example:

```rust,editable title="Shoes in my size"
#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    println!("{:?}", shoes_in_size(shoes, 10));
}
```

`into_iter` takes ownership of the vector, so the filtered shoes are moved into the result, not copied. The closure captures `shoe_size` from the function's parameter.

## Chaining

Adapters chain naturally. Read the chain top to bottom, like a recipe:

```rust,runnable title="A chain of adapters"
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let doubled_evens: Vec<i32> = numbers
        .iter()
        .filter(|n| *n % 2 == 0)
        .map(|n| n * 2)
        .collect();
    println!("{doubled_evens:?}");
}
```

`filter` gives its closure a **reference** to each item, and `iter()` already yields references, so `n` is a `&&i32` there. `*n % 2` works anyway, because Rust's arithmetic operators look through one extra `&` for you.
