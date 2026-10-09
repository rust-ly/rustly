+++
id = "book.concurrency.mutex"
chapter = "book.concurrency"
requires = []
title = "Mutex<T>: One at a Time"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch16-03-shared-state.html"
summary = "A Mutex lets only one thread at a time reach its data; lock() gives access until the guard is dropped."
+++

# Shared-State Concurrency with `Mutex<T>`

Channels give each value one owner at a time. The other approach is **shared state**: several threads access the **same** data. To keep that safe, only one thread may touch the data at any moment.

A **mutex** (short for **mut**ual **ex**clusion) enforces that. Picture a panel discussion with one microphone: you have to take the microphone before speaking, and hand it back when you're done so someone else can speak.

## Using a mutex

`Mutex<T>` wraps the data. To reach it, call `lock()`, which waits until no other thread holds the lock and then returns a **guard**:

```rust,runnable title="Locking and changing a value"
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    } // the guard is dropped here, releasing the lock
    println!("m = {m:?}");
}
```

- `lock()` returns a `Result`, which is `Err` if another thread panicked while holding the lock (the mutex is "poisoned"). `unwrap` is the usual response.
- The guard, a `MutexGuard`, acts like a `&mut` to the data (it implements `Deref` and `DerefMut`), so `*num = 6` changes the value.
- When the guard is dropped, the lock is **released automatically** (it implements `Drop`). You can't forget to unlock, the classic mutex bug in other languages.

This is the same idea as `RefCell` from chapter 15: changing data through a shared handle. `Mutex` is the thread-safe version.

## Keep locks short

While one guard exists, every other `lock()` call waits. Holding a lock longer than needed slows everything down, and taking a second lock on the same mutex while holding the first **deadlocks**: the thread waits for itself forever.

```rust,editable title="Short, scoped locks"
use std::sync::Mutex;

fn main() {
    let scores = Mutex::new(vec![10, 20]);

    scores.lock().unwrap().push(30); // locked only for this line

    let total: i32 = scores.lock().unwrap().iter().sum();
    let count = scores.lock().unwrap().len();
    println!("{count} scores, total {total}");
}
```

Each `lock()` here is a temporary guard that's dropped at the end of its statement, so the locks never overlap.

To share a mutex between **threads**, it needs more than one owner, and that's the next concept.
