+++
id = "tokio.spawning.tasks"
chapter = "tokio.spawning"
requires = []
title = "Tasks and JoinHandles"
track = "tokio"
order = 1
source = "https://tokio.rs/tokio/tutorial/spawning"
summary = "tokio::spawn starts a task that runs concurrently with the rest of the program. Its JoinHandle gives you the result."
+++

# Tasks and `JoinHandle`s

In the tutorial, the mini-Redis server accepts connections in a loop. If it handled each connection right there in the loop, a slow client would block everyone else. Instead it **spawns a task** per connection.

A Tokio **task** is an asynchronous green thread: a future that the runtime schedules on its own. Tasks are cheap. Spawning one is a small allocation, and a program can have hundreds of thousands of them.

## `tokio::spawn`

`tokio::spawn` takes a future, starts running it in the background, and immediately returns a `JoinHandle`. Awaiting the handle waits for the task and gives back its output:

```rust,runnable title="Spawning a task"
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Some work that runs in the background.
        "return value"
    });

    // Do some other work here...

    let out = handle.await.unwrap();
    println!("got {out}");
}
```

Awaiting a `JoinHandle` gives a `Result`. It's `Err` if the task **panicked** (or was cancelled), so one crashing task doesn't take down the rest of the program.

## Tasks run concurrently

Unlike awaiting futures one after another, spawned tasks all make progress at the same time. While one waits on a timer or a socket, the others run:

```rust,editable title="Three tasks at once"
use std::time::Duration;
use tokio::time::{Instant, sleep};

async fn slow_double(n: u64) -> u64 {
    sleep(Duration::from_millis(100)).await;
    n * 2
}

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let handles: Vec<_> = (1..=3).map(|n| tokio::spawn(slow_double(n))).collect();
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await.unwrap());
    }

    println!("{results:?} in about {} ms", start.elapsed().as_millis() / 100 * 100);
}
```

Three 100 ms jobs finish in about 100 ms, not 300. Spawning all the tasks **first** and then awaiting the handles is what makes that happen. Try awaiting each `tokio::spawn(...)` straight away inside the `map` and the jobs go back to running one at a time.

Awaiting the handles in order still returns the results in order, even if a later task finished first: each handle holds its task's result until you ask for it.
