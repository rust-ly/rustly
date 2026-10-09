+++
id = "book.concurrency.channels"
chapter = "book.concurrency"
requires = []
title = "Message Passing with Channels"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch16-02-message-passing.html"
summary = "Threads communicate by sending values down a channel: send moves a value in, recv takes it out."
+++

# Using Message Passing to Transfer Data Between Threads

One popular way to make threads cooperate is to have them **send each other messages** instead of sharing memory. The Go language puts it as: *"Do not communicate by sharing memory; instead, share memory by communicating."*

Rust's standard library provides **channels** for this. A channel has two halves: a **transmitter** (`tx`) that sends and a **receiver** (`rx`) that receives. Picture a river: drop a rubber duck in upstream, and it arrives downstream.

```rust,runnable title="Sending one message"
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
```

- `mpsc` stands for **multiple producer, single consumer**: many transmitters, one receiver.
- `send` returns a `Result`, which is `Err` if the receiver has been dropped.
- `recv` **blocks** until a message arrives. It returns `Err` once every transmitter has been dropped and no more messages can come. (`try_recv` checks without waiting.)

## Sending transfers ownership

`send` **moves** the value into the channel. After sending, the thread can't use it, which rules out the bug where one thread changes data another thread has just received:

```rust,does_not_compile title="Using a value after sending it"
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {val}");
    });
    println!("{}", rx.recv().unwrap());
}
```

## Several messages, several producers

The receiver can be used as an **iterator**: a `for` loop receives messages until the channel closes, which happens when every transmitter is dropped. To have several producers, `clone` the transmitter:

```rust,editable title="Two producers, one receiver"
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        for word in ["hi", "from", "the", "thread"] {
            tx.send(format!("A: {word}")).unwrap();
        }
    });
    thread::spawn(move || {
        for word in ["more", "messages", "for", "you"] {
            tx2.send(format!("B: {word}")).unwrap();
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
    println!("channel closed");
}
```

The A and B messages interleave in an unpredictable order. The loop ends after both threads finish, because that's when both transmitters are dropped.
