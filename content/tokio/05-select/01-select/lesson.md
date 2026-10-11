+++
id = "tokio.select.select"
chapter = "tokio.select"
requires = []
title = "tokio::select!"
track = "tokio"
order = 1
source = "https://tokio.rs/tokio/tutorial/select"
summary = "select! waits on several futures at once and runs the branch of whichever finishes first. The others are dropped, which cancels them."
+++

# `tokio::select!`

So far, every `.await` waits for one thing. Often you want to wait for **whichever comes first**: a reply or a timeout, a message or a shutdown signal. That's `tokio::select!`.

```rust,runnable title="First one wins"
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    tokio::select! {
        _ = sleep(Duration::from_millis(50)) => {
            println!("the 50 ms timer finished first");
        }
        _ = sleep(Duration::from_millis(100)) => {
            println!("the 100 ms timer finished first");
        }
    }
}
```

Each branch is `pattern = future => handler`. `select!` polls all the futures together. As soon as one completes, its output is matched against the pattern and **only that branch's handler** runs.

## The losers are cancelled

When one branch wins, `select!` **drops** the other futures. In async Rust, dropping a future is how you cancel it: it simply never runs again. So the code below never prints "slow done":

```rust,runnable title="Dropping a future cancels it"
use std::time::Duration;
use tokio::time::sleep;

async fn slow() -> &'static str {
    sleep(Duration::from_secs(10)).await;
    println!("slow done");
    "slow"
}

#[tokio::main]
async fn main() {
    let winner = tokio::select! {
        v = slow() => v,
        _ = sleep(Duration::from_millis(20)) => "timeout",
    };
    println!("winner: {winner}");
}
```

`select!` is an expression: each handler's value becomes the value of the whole `select!`, so all handlers must return the same type.

Cancellation stops the future at its last `.await`. Code after that point never runs, so think about what state a cancelled future might leave behind. (A spawned task is different: dropping its `JoinHandle` doesn't stop it.)

## Timeouts

The most common use is a timeout: race the real work against `sleep`. Tokio also packages this as `tokio::time::timeout(duration, future)`, which returns `Err(Elapsed)` if time runs out.
