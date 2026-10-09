+++
id = "book.concurrency.threads"
chapter = "book.concurrency"
requires = []
title = "Creating Threads"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch16-01-threads.html"
summary = "Run code at the same time with thread::spawn, and wait for it with join."
+++

# Using Threads to Run Code Simultaneously

A running program is a **process**. Inside it, code can run in several **threads** at once, for example one handling the user interface while another downloads a file. On a computer with several cores, threads really do run at the same moment.

That brings new kinds of bugs: **race conditions** (threads touching the same data in an unpredictable order) and **deadlocks** (threads waiting on each other forever). Rust's ownership and type system catch most of these at **compile time**, which the Book calls *fearless concurrency*.

## Spawning a thread

`thread::spawn` takes a closure and runs it in a new thread:

```rust,runnable title="Two threads printing at once"
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

Run it a few times. The two threads take turns in an order that can change from run to run. And the spawned thread probably **doesn't finish**: when `main` ends, the whole program stops, taking the other thread with it.

## Waiting with `join`

`spawn` returns a `JoinHandle`. Calling `.join()` on it **blocks** (waits) until that thread finishes:

```rust,editable title="Waiting for a thread to finish"
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..6 {
            println!("spawned: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..3 {
        println!("main: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    println!("both done");
}
```

Where you put `join` matters. Move it to just after `spawn` and `main` waits for the whole spawned loop before starting its own, so they no longer run at the same time.

## Getting a result back

The closure's return value comes back through `join`, wrapped in a `Result` (it's `Err` if the thread panicked):

```rust,runnable title="A thread that returns a value"
use std::thread;

fn main() {
    let handle = thread::spawn(|| (1..=100).sum::<u32>());
    let total = handle.join().unwrap();
    println!("sum from the thread: {total}");
}
```
