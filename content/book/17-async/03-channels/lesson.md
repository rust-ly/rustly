+++
id = "book.async.channels"
chapter = "book.async"
requires = []
title = "Async Channels and while let"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html#counting-up-on-two-tasks-using-message-passing"
summary = "Tokio's mpsc channel works like std's, but sending and receiving are awaited; while let loops until the channel closes."
+++

# Message Passing with Async Channels

Chapter 16's channels block the **thread** while waiting, which would stall every task on it. Tokio has an async version, `tokio::sync::mpsc`, where waiting is done with `.await` instead:

```rust,runnable title="An async channel"
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(8);

    tokio::spawn(async move {
        for word in ["hi", "from", "the", "task"] {
            tx.send(word.to_string()).await.unwrap();
        }
    });

    while let Some(msg) = rx.recv().await {
        println!("got: {msg}");
    }
    println!("channel closed");
}
```

Two differences from `std::sync::mpsc`:

- `mpsc::channel(8)` takes a **capacity**: at most 8 messages can wait in the channel. If it's full, `send(...).await` waits for room. This is called **backpressure**, and it stops a fast producer from using up all your memory.
- `recv().await` returns `Option<T>`: `Some(message)`, or `None` once every sender is dropped and the channel is empty.

## `while let`: loop while a pattern matches

`while let Some(msg) = rx.recv().await { ... }` is a new kind of loop. It's to `while` what `if let` is to `if`: it keeps looping as long as the value matches the pattern, binding `msg` each time, and stops the first time it doesn't (here, at `None`).

```rust,editable title="while let on a stack"
fn main() {
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}
```

## Remember to drop the sender

The receiving loop ends only when **all** senders are gone. A sender kept alive by accident means the loop waits forever. Moving each sender into its task (with `async move`) drops it when the task finishes:

```rust,runnable title="Two producers"
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(8);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        for n in 1..=3 {
            tx.send(format!("A{n}")).await.unwrap();
        }
    });
    tokio::spawn(async move {
        for n in 1..=3 {
            tx2.send(format!("B{n}")).await.unwrap();
        }
    });

    let mut all = Vec::new();
    while let Some(msg) = rx.recv().await {
        all.push(msg);
    }
    all.sort();
    println!("{all:?}");
}
```
