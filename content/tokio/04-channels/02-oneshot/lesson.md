+++
id = "tokio.channels.oneshot"
chapter = "tokio.channels"
requires = []
title = "The Manager Task and oneshot Replies"
track = "tokio"
order = 2
source = "https://tokio.rs/tokio/tutorial/channels#receive-responses"
summary = "One task owns the resource; others send it commands over mpsc, each carrying a oneshot sender for the reply."
+++

# The Manager Task and `oneshot` Replies

In the tutorial, many tasks want to use a single Redis connection. A connection can't be shared like a `HashMap` behind a mutex, because each request is a whole conversation with `.await`s in it. The answer is a **manager task**: it owns the connection, receives commands over an `mpsc` channel, and handles them one at a time.

## Commands as an enum

Each kind of request is an enum variant:

```rust
enum Command {
    Get { key: String, resp: oneshot::Sender<Option<String>> },
    Set { key: String, value: String, resp: oneshot::Sender<()> },
}
```

The `resp` field is how the manager answers. A **`oneshot`** channel carries exactly one value. The requester keeps the `Receiver` and awaits it; the manager uses the `Sender` once. `oneshot::Sender::send` isn't async (there's nothing to wait for, the value just goes into its slot), and it returns `Err` if the requester stopped waiting.

## The whole pattern

```rust,editable title="A manager task"
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

enum Command {
    Get { key: String, resp: oneshot::Sender<Option<String>> },
    Set { key: String, value: String, resp: oneshot::Sender<()> },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Command>(32);

    // The manager owns the data. No mutex needed: only this task touches it.
    let manager = tokio::spawn(async move {
        let mut data = HashMap::new();
        while let Some(command) = rx.recv().await {
            match command {
                Command::Get { key, resp } => {
                    let _ = resp.send(data.get(&key).cloned());
                }
                Command::Set { key, value, resp } => {
                    data.insert(key, value);
                    let _ = resp.send(());
                }
            }
        }
    });

    let (resp, answer) = oneshot::channel();
    tx.send(Command::Set { key: "hello".into(), value: "world".into(), resp }).await.unwrap();
    answer.await.unwrap();

    let (resp, answer) = oneshot::channel();
    tx.send(Command::Get { key: "hello".into(), resp }).await.unwrap();
    println!("hello = {:?}", answer.await.unwrap());

    drop(tx); // the manager's loop ends when the last sender is gone
    manager.await.unwrap();
}
```

`let _ = resp.send(...)` ignores the error on purpose: if the requester gave up waiting, there's no one to tell.

## Mutex or manager?

Both are fine. A mutex is simpler when the work inside the lock is quick and doesn't `.await`. A manager task fits when the resource needs async work per request (a connection, a file), or when you want requests handled strictly in order. The `mpsc` capacity also gives you backpressure for free.
