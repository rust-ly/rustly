+++
id = "book.async.future-trait"
chapter = "book.async"
requires = []
title = "The Future Trait, and Tasks vs Threads"
track = "book"
order = 6
source = "https://doc.rust-lang.org/book/ch17-05-traits-for-async.html"
summary = "What .await does underneath: poll, Poll::Ready and Poll::Pending, Pin, and when to choose async over threads."
+++

# A Closer Look at the Traits for Async

## The `Future` trait

Every future implements this trait from the standard library:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

The runtime calls `poll` to ask "are you done yet?". The answer is a `Poll`:

- `Poll::Ready(value)`: finished, here's the output.
- `Poll::Pending`: not yet. Before returning `Pending`, the future arranges (through `cx`) to **wake** the runtime when it can make progress, for example when data arrives, so the runtime doesn't have to keep asking.

`.await` is shorthand for that loop: poll; if pending, let other tasks run until woken; poll again.

## A hand-written future

You almost never implement `Future` yourself (`async fn` writes it for you), but doing it once makes the machinery concrete. This future is ready the first time it's polled:

```rust,runnable title="Implementing Future by hand"
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Ready(u32);

impl Future for Ready {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<u32> {
        Poll::Ready(self.0)
    }
}

#[tokio::main]
async fn main() {
    let n = Ready(42).await;
    println!("got {n}");
}
```

## Why `Pin`?

An `async fn` that holds a reference across an `.await` compiles to a struct that refers to **its own** fields. Moving such a struct in memory would leave those internal references pointing at the old location. `Pin<&mut Self>` is a promise that the future won't be moved again once polling starts. Types without self-references, like `Ready`, are `Unpin`: pinning them doesn't restrict anything. In everyday code you mostly meet `Pin` in error messages, and `Box::pin(future)` is the usual fix.

## Tasks or threads?

| | Threads | Async tasks |
| --- | --- | --- |
| Best for | CPU-heavy work (number crunching, compression) | Waiting-heavy work (network, files, timers) |
| Cost | An OS thread each, with its own stack | Tiny; millions are fine |
| Switching | The OS can interrupt at any point | Only at `.await` points |

They combine well: an async program can hand a heavy computation to a thread with `tokio::task::spawn_blocking`. Just never block (with `std::thread::sleep` or a long calculation) inside an async task, because it holds up every other task on that thread.

```rust,editable title="spawn_blocking for CPU work"
#[tokio::main]
async fn main() {
    let total = tokio::task::spawn_blocking(|| (1..=1_000_000u64).sum::<u64>())
        .await
        .unwrap();
    println!("computed on a blocking thread: {total}");
}
```

That's the end of the Book's async chapter. The **Tokio track** builds on everything here: spawning, shared state, channels, select and streams, step by step.
