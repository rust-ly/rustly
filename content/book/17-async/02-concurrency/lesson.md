+++
id = "book.async.concurrency"
chapter = "book.async"
requires = []
title = "Concurrency with Async"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html"
summary = "Run futures at the same time with tokio::spawn and tokio::join!, instead of awaiting them one by one."
+++

# Applying Concurrency with Async

Awaiting futures one after another runs them **in sequence**: the second doesn't start until the first is done. To wait on several things **at the same time**, there are two main tools.

## `tokio::join!`: wait for several futures together

`join!` runs its futures concurrently and returns all their results once **every** one is done:

```rust,runnable title="Sequential vs concurrent"
use std::time::Duration;
use tokio::time::{sleep, Instant};

async fn boil_water() -> &'static str {
    sleep(Duration::from_millis(300)).await;
    "water"
}

async fn toast_bread() -> &'static str {
    sleep(Duration::from_millis(200)).await;
    "toast"
}

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let a = boil_water().await;
    let b = toast_bread().await;
    println!("one by one: {a}, {b} in {}ms", start.elapsed().as_millis());

    let start = Instant::now();
    let (a, b) = tokio::join!(boil_water(), toast_bread());
    println!("together:   {a}, {b} in {}ms", start.elapsed().as_millis());
}
```

One by one takes about 500 ms; together, about 300 ms, the time of the slowest. Like cooking: you toast the bread while the water boils.

`tokio::time::sleep` is the async version of `thread::sleep`. It pauses only **this** task, letting others run. Calling `std::thread::sleep` inside async code would freeze the whole thread, and every task on it.

## `tokio::spawn`: start an independent task

`tokio::spawn` hands a future to the runtime as a separate **task**, much like `thread::spawn` but far cheaper: a program can run millions of tasks. It returns a `JoinHandle` that you `.await` for the result:

```rust,editable title="Spawning tasks"
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        for i in 1..5 {
            println!("hi number {i} from the task!");
            sleep(Duration::from_millis(5)).await;
        }
        "task finished"
    });

    for i in 1..3 {
        println!("hi number {i} from main!");
        sleep(Duration::from_millis(5)).await;
    }

    let result = handle.await.unwrap();
    println!("{result}");
}
```

As with threads, the main task and the spawned task interleave, and awaiting the handle waits for the task to finish. Spawned tasks need owned data (an `async move` block), for the same reason threads do: the task may outlive the code that spawned it.
