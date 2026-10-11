+++
id = "tokio.select.loops"
chapter = "tokio.select"
requires = []
title = "Select Loops"
track = "tokio"
order = 2
source = "https://tokio.rs/tokio/tutorial/select#loops"
summary = "select! in a loop over channels: patterns that skip closed channels, the else branch, and reusing one future across iterations with pin!."
+++

# Select Loops

`select!` really shines inside a `loop`, handling whatever arrives next from several sources: messages on two channels, a shutdown signal, a ticking timer.

## Patterns and `else`

A branch's pattern can **refuse** a value. If a future completes but its output doesn't match, that branch is **disabled** for the rest of this `select!`, and the others keep running. With `Some(v) = rx.recv()`, a closed channel (`None`) simply drops out. When every branch is disabled, the `else` branch runs:

```rust,runnable title="Reading two channels until both close"
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(8);
    let (tx2, mut rx2) = mpsc::channel(8);

    tokio::spawn(async move {
        for i in 0..3 {
            tx1.send(format!("one-{i}")).await.unwrap();
        }
    });
    tokio::spawn(async move {
        for i in 0..2 {
            tx2.send(format!("two-{i}")).await.unwrap();
        }
    });

    loop {
        tokio::select! {
            Some(v) = rx1.recv() => println!("got {v}"),
            Some(v) = rx2.recv() => println!("got {v}"),
            else => break,
        }
    }
    println!("both channels closed");
}
```

Without the `else`, a `select!` whose branches are all disabled panics.

## Reusing a future across iterations

Each time around the loop, `select!` builds its futures afresh. That's right for `rx.recv()`, but wrong for something like an overall deadline: a new `sleep(5s)` every iteration would never fire. Create the future **once**, pin it with `tokio::pin!`, and select on `&mut` it:

```rust,editable title="One deadline for the whole loop"
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(8);
    tokio::spawn(async move {
        for i in 0.. {
            if tx.send(i).await.is_err() {
                break;
            }
            sleep(Duration::from_millis(30)).await;
        }
    });

    let deadline = sleep(Duration::from_millis(100));
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            Some(n) = rx.recv() => println!("got {n}"),
            _ = &mut deadline => {
                println!("deadline reached");
                break;
            }
        }
    }
}
```

`pin!` is needed because selecting on a reference requires the future to stay put in memory; most futures (including `sleep`) aren't `Unpin`.

By default `select!` checks its branches in **random** order each time, so no branch can starve the others. Writing `biased;` as the first line makes it check them top to bottom instead, which is handy when one branch, like shutdown, should always win a tie.
