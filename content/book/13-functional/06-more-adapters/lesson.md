+++
id = "book.functional.more-adapters"
chapter = "book.functional"
requires = []
title = "More Iterator Tools"
track = "book"
order = 6
source = "https://doc.rust-lang.org/std/iter/trait.Iterator.html"
summary = "enumerate, zip, rev, take, skip, and the consumers sum, count, any, all, find, min and max."
+++

# More Iterator Tools

The `Iterator` trait has dozens of default methods. A handful cover most real code, and knowing them lets you replace most hand-written loops with something shorter and clearer.

## Adapters that reshape the sequence

```rust,runnable title="enumerate, zip, rev, take, skip"
fn main() {
    let names = ["Ada", "Grace", "Linus"];
    let scores = [90, 85, 70];

    // enumerate: pair each item with its index
    for (i, name) in names.iter().enumerate() {
        println!("{}. {name}", i + 1);
    }

    // zip: walk two sequences side by side
    let pairs: Vec<(&&str, &i32)> = names.iter().zip(scores.iter()).collect();
    println!("{pairs:?}");

    // rev, take, skip
    let v: Vec<i32> = (1..=10).rev().skip(2).take(3).collect();
    println!("{v:?}");
}
```

`1..=10` is a **range**, and ranges are iterators too.

## Consumers that answer questions

These use up the iterator and return a single value:

```rust,editable title="Asking questions about a sequence"
fn main() {
    let temps = [18, 22, 25, 19, 30, 27];

    let total: i32 = temps.iter().sum();
    let hot_days = temps.iter().filter(|&&t| t >= 25).count();
    let any_freezing = temps.iter().any(|&t| t <= 0);
    let all_mild = temps.iter().all(|&t| t > 10);
    let first_above_24 = temps.iter().find(|&&t| t > 24);
    let max = temps.iter().max();

    println!("total {total}, hot days {hot_days}");
    println!("any freezing? {any_freezing}, all mild? {all_mild}");
    println!("first above 24: {first_above_24:?}, max: {max:?}");
}
```

`find`, `max` and `min` return an `Option`, because the sequence might be empty (or have no match). `any` and `all` stop as soon as they know the answer.

The `|&t|` and `|&&t|` patterns **destructure** the references, so `t` is a plain `i32` inside the closure. `filter` and `find` add one more `&` than `any`, which is why they need two.

## `fold`: the general accumulator

When none of the ready-made consumers fit, `fold` lets you carry a value through the whole sequence. It takes a starting value and a closure that combines the running value with each item:

```rust,runnable title="fold"
fn main() {
    let words = ["iter", "ators", " are", " neat"];
    let joined = words.iter().fold(String::new(), |mut acc, w| {
        acc.push_str(w);
        acc
    });
    let product = (1..=5).fold(1, |acc, n| acc * n);
    println!("{joined}");
    println!("5! = {product}");
}
```

Most loops that build up a result are a `fold` underneath; `sum`, `count` and `max` are just common folds with names.
