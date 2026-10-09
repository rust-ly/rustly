+++
id = "book.async.streams"
chapter = "book.async"
requires = []
title = "Streams: Futures in Sequence"
track = "book"
order = 5
source = "https://doc.rust-lang.org/book/ch17-04-streams.html"
summary = "A stream is an async iterator: a series of values that arrive over time, processed with StreamExt adapters."
+++

# Streams: Futures in Sequence

An iterator gives you items one after another, immediately. A **stream** gives you items one after another **over time**: messages arriving on a socket, rows coming back from a database, keystrokes. You could call it an async iterator.

The `futures` crate provides streams and the **`StreamExt`** trait, which adds iterator-like methods. The difference is that getting each item is awaited:

```rust,runnable title="Iterating a stream"
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    let mut s = stream::iter(vec![1, 2, 3]);
    while let Some(value) = s.next().await {
        println!("got {value}");
    }
}
```

`stream::iter` turns any iterator into a stream, which is handy for practice and tests. `next().await` returns `Some(item)` or `None` when the stream ends, so `while let` is the natural loop, exactly as with async channels.

## Adapters

`StreamExt` has the adapters you know from chapter 13, like `map`, `filter` and `take`, plus `collect`, which must be awaited:

```rust,editable title="map, filter and collect on a stream"
use futures::stream::{self, StreamExt};

#[tokio::main]
async fn main() {
    let evens_squared: Vec<i32> = stream::iter(1..=10)
        .filter(|n| futures::future::ready(n % 2 == 0))
        .map(|n| n * n)
        .collect()
        .await;
    println!("{evens_squared:?}");
}
```

One wrinkle: `filter`'s closure must return a **future** of a `bool`, not a plain `bool`, because a stream filter might need to await something. `futures::future::ready(value)` wraps a value in a future that's ready at once.

## Running async work for each item

The real power shows when each item needs async work. `then` runs an async closure per item, in order:

```rust,runnable title="Async work per item"
use futures::stream::{self, StreamExt};
use std::time::Duration;

async fn lookup(id: u32) -> String {
    tokio::time::sleep(Duration::from_millis(10)).await;
    format!("user-{id}")
}

#[tokio::main]
async fn main() {
    let users: Vec<String> = stream::iter([3, 1, 2])
        .then(|id| lookup(id))
        .collect()
        .await;
    println!("{users:?}");
}
```

The Tokio track goes further with streams. Its streams section uses the same `StreamExt` methods on data arriving over time.
