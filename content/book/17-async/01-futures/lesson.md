+++
id = "book.async.futures"
chapter = "book.async"
requires = []
title = "Futures and async/await"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html"
summary = "An async fn returns a future: a value that will be ready later. .await waits for it, and a runtime drives it."
+++

# Futures and the Async Syntax

Threads let a program do several things at once. But much of what programs wait on (a web page to download, a file to be read, a timer) isn't **work**, it's **waiting**. Giving every wait its own thread is wasteful. **Async** Rust lets one thread juggle many waiting tasks, switching to another whenever one has to wait.

## Futures

A **future** is a value that may not be ready yet but will be at some point, like an order number at a café. In Rust, futures are types that implement the `Future` trait.

An **`async fn`** doesn't run its body right away. Calling it returns a future, and the body runs when the future is **awaited**:

```rust,runnable title="async fn and .await"
async fn fetch_greeting() -> String {
    String::from("hello from the future")
}

#[tokio::main]
async fn main() {
    let future = fetch_greeting(); // nothing has run yet
    let greeting = future.await;   // now it runs, and we wait for the result
    println!("{greeting}");
}
```

`.await` goes **after** the expression (postfix), so chains read left to right: `fetch(url).await.text().await`.

## Futures are lazy

A future does nothing until it's awaited. Rust warns you if you create one and forget it:

```rust,runnable title="A future that never runs"
async fn say_hi() {
    println!("hi!");
}

#[tokio::main]
async fn main() {
    let _ = say_hi(); // creates a future and drops it: nothing printed
    say_hi().await;   // this one prints
}
```

## Runtimes

Something has to keep checking futures and push them forward: an **async runtime**. Rust's standard library defines `Future` but doesn't include a runtime, so you choose one. This course uses **Tokio**, the most widely used. `#[tokio::main]` starts the runtime and runs your `async fn main` on it.

`.await` only works inside an `async` function or block. A plain `fn main` can't await:

```rust,does_not_compile title="await outside an async function"
async fn answer() -> u32 {
    42
}

fn main() {
    let n = answer().await;
    println!("{n}");
}
```

The Book's own chapter uses a helper crate called `trpl` that isn't available on the Playground, so this chapter uses Tokio directly. Tokio has its own track in this course, which picks up where this chapter leaves off.
