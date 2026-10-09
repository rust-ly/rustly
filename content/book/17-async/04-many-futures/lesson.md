+++
id = "book.async.many-futures"
chapter = "book.async"
requires = []
title = "Racing and Timeouts"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch17-03-more-futures.html"
summary = "Wait for whichever future finishes first with tokio::select!, give up after a deadline with timeout, and join a whole set of tasks."
+++

# Working with Any Number of Futures

`join!` waits for **all** futures. Often you want something else: whichever finishes **first**, or "this, but give up if it takes too long".

## Racing with `tokio::select!`

`select!` waits on several futures and runs the branch of the **first** one to complete. The others are dropped (cancelled):

```rust,runnable title="The fastest server wins"
use std::time::Duration;
use tokio::time::sleep;

async fn server(name: &'static str, ms: u64) -> &'static str {
    sleep(Duration::from_millis(ms)).await;
    name
}

#[tokio::main]
async fn main() {
    let winner = tokio::select! {
        a = server("europe", 120) => a,
        b = server("africa", 40) => b,
        c = server("asia", 80) => c,
    };
    println!("first answer from {winner}");
}
```

Each branch is `pattern = future => expression`. Cancelling the losers is safe in Rust: a future that's dropped simply never runs again.

## Timeouts

`tokio::time::timeout(duration, future)` races a future against a timer. It returns `Ok(value)` if the future finishes in time, or `Err(Elapsed)` if the timer wins:

```rust,editable title="Giving up after a deadline"
use std::time::Duration;
use tokio::time::{sleep, timeout};

async fn slow_download() -> &'static str {
    sleep(Duration::from_millis(500)).await;
    "file contents"
}

#[tokio::main]
async fn main() {
    match timeout(Duration::from_millis(100), slow_download()).await {
        Ok(data) => println!("got {data}"),
        Err(_) => println!("gave up after 100ms"),
    }
    match timeout(Duration::from_secs(1), slow_download()).await {
        Ok(data) => println!("got {data}"),
        Err(_) => println!("gave up"),
    }
}
```

## Joining a variable number of tasks

`join!` needs the futures written out in the code. When the number is only known at run time, put tasks in a `JoinSet` and collect their results as they finish:

```rust,runnable title="A JoinSet"
use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let mut set = JoinSet::new();
    for n in [3, 1, 2] {
        set.spawn(async move {
            sleep(Duration::from_millis(n * 20)).await;
            n * 10
        });
    }
    while let Some(result) = set.join_next().await {
        println!("finished: {}", result.unwrap());
    }
}
```

Results arrive in the order the tasks **finish**: 10, 20, 30.
