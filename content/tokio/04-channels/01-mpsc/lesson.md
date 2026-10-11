+++
id = "tokio.channels.mpsc"
chapter = "tokio.channels"
requires = []
title = "mpsc Channels"
track = "tokio"
order = 1
source = "https://tokio.rs/tokio/tutorial/channels"
summary = "Many senders, one receiver. Bounded channels give backpressure, and the receive loop ends when every sender is gone."
+++

# `mpsc` Channels

A mutex isn't the only way to share. Instead of letting every task touch the data, you can give it to **one** task and have the others send it messages. Tokio has several channel types for that:

- **`mpsc`**: multi-producer, single-consumer. Many senders, one receiver. The workhorse.
- **`oneshot`**: one sender, one receiver, one value. Perfect for a reply.
- **`broadcast`**: every receiver sees every message.
- **`watch`**: receivers only see the latest value, like a config that changes.

## Sending and receiving

`mpsc::channel(capacity)` returns a `Sender` and a `Receiver`. Clone the sender to give one to each producing task:

```rust,runnable title="Two producers, one receiver"
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await.unwrap();
    });
    tokio::spawn(async move {
        tx2.send("sending from second handle").await.unwrap();
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {message}");
    }
    println!("every sender is gone, so the loop ended");
}
```

`rx.recv()` returns `None` once **every** `Sender` has been dropped and the buffer is empty. That's how the receive loop knows to stop. Here both senders were moved into tasks and dropped when those tasks finished.

## Don't forget the original sender

The classic bug: you clone a sender for each task but keep the original alive in the receiving function. `recv()` then waits forever for a message that can never come, because a sender still exists:

```rust,editable title="Dropping the last sender"
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(8);
    for i in 0..3 {
        let tx = tx.clone();
        tokio::spawn(async move { tx.send(i).await.unwrap() });
    }
    // Without this line the loop below never ends.
    drop(tx);

    let mut total = 0;
    while let Some(n) = rx.recv().await {
        total += n;
    }
    println!("total = {total}");
}
```

## Capacity is backpressure

The number passed to `channel` is the buffer size. When the buffer is full, `send(...).await` **waits** until the receiver catches up. That's backpressure: a fast producer can't pile up unlimited messages in memory. Tokio deliberately has no unbounded default; `mpsc::unbounded_channel` exists, but you have to ask for it.
