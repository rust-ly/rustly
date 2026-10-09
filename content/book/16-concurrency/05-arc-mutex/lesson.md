+++
id = "book.concurrency.arc-mutex"
chapter = "book.concurrency"
requires = []
title = "Arc<Mutex<T>>: Sharing Across Threads"
track = "book"
order = 5
source = "https://doc.rust-lang.org/book/ch16-03-shared-state.html#sharing-a-mutext-between-multiple-threads"
summary = "Arc is the thread-safe Rc; wrap a Mutex in it to give every thread a handle to the same data."
+++

# Sharing a `Mutex<T>` Between Multiple Threads

Ten threads should each add 1 to a shared counter. Moving the mutex into the first thread's closure leaves nothing for the others, so the counter needs **several owners**. In chapter 15 that meant `Rc`. Let's try:

```rust,does_not_compile title="Rc can't cross threads"
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
```

The error says `Rc<Mutex<i32>>` cannot be sent between threads safely. `Rc` updates its count without any protection. Two threads cloning at once could both read 1 and both write 2, losing a count and freeing the value too early.

## `Arc<T>`: atomic reference counting

`Arc<T>` (**a**tomically **r**eference **c**ounted) is exactly like `Rc<T>`, but updates its count with **atomic** operations that are safe across threads. They're a little slower, which is why `Rc` exists for single-threaded code. Swap one for the other and it works:

```rust,editable title="Ten threads, one counter"
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

The pattern to remember:

1. Wrap the data: `Arc::new(Mutex::new(data))`.
2. Before each `spawn`, make a clone of the handle: `let counter = Arc::clone(&counter);`
3. `move` the clone into the closure, and `lock()` inside it.

## `Arc<Mutex<T>>` mirrors `Rc<RefCell<T>>`

| Single-threaded | Thread-safe | What it adds |
| --- | --- | --- |
| `Rc<T>` | `Arc<T>` | several owners |
| `RefCell<T>` | `Mutex<T>` | change through a shared handle |

Mutexes come with their own risk, **deadlocks**: thread A holds lock 1 and waits for lock 2 while thread B holds lock 2 and waits for lock 1. Rust can't prevent those at compile time, so keep locks short and always take several locks in the same order.
