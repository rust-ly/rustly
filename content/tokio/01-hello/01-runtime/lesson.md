+++
id = "tokio.hello.runtime"
chapter = "tokio.hello"
requires = []
title = "The Runtime and #[tokio::main]"
track = "tokio"
order = 1
source = "https://tokio.rs/tokio/tutorial/hello-tokio"
summary = "Async Rust needs a runtime to run futures. Tokio is that runtime, and #[tokio::main] starts it for you."
+++

# The Runtime and `#[tokio::main]`

Tokio is an **asynchronous runtime** for Rust. It's what most async Rust programs use to handle many network connections, timers and tasks at once, without a thread for each one.

In the Book's async chapter you met futures: an `async fn` gives back a **future**, a value that will be ready later, and `.await` waits for it. But the standard library has no way to actually **run** a future. That job belongs to a runtime: it keeps calling the futures you give it, parks them while they wait (for a timer, a socket, a channel) and wakes them up when they can make progress.

The Tokio tutorial builds a small Redis-style server. The Playground has no network, so in this track we keep the same ideas but run everything in memory.

## Starting the runtime

The usual way in is the `#[tokio::main]` attribute on an `async fn main`:

```rust,runnable title="Hello, Tokio"
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("hello");
    tokio::time::sleep(Duration::from_millis(100)).await;
    println!("tokio");
}
```

`main` can't really be async, because something has to start the runtime first. `#[tokio::main]` is a macro that rewrites your function into roughly this:

```rust,runnable title="What #[tokio::main] expands to"
use std::time::Duration;

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        println!("hello");
        tokio::time::sleep(Duration::from_millis(100)).await;
        println!("tokio");
    })
}
```

- `Builder` configures a runtime. `new_multi_thread()` spreads tasks over a pool of threads; `new_current_thread()` runs everything on the thread that calls it, which is simpler and fine for small programs and tests.
- `enable_all()` switches on Tokio's **drivers**: the timer (for `sleep` and timeouts) and I/O. Leave it out and the first `sleep` panics, because no timer is running.
- `block_on` runs one future to completion, blocking the current thread until it's done, and returns that future's output.

## Async code needs the runtime

Tokio's timers and I/O belong to a runtime. Even **creating** a Tokio timer outside one panics, because there's no timer driver to register it with:

```rust,panics title="A Tokio timer with no runtime"
use std::time::Duration;

fn main() {
    let _sleep = tokio::time::sleep(Duration::from_millis(10));
    println!("never printed");
}
```

That's why every Tokio program starts by building a runtime, with the attribute or by hand. Tests do the same with `#[tokio::test]`, which is `#[test]` plus a current-thread runtime.
