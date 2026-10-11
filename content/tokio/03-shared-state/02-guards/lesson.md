+++
id = "tokio.shared-state.guards"
chapter = "tokio.shared-state"
requires = []
title = "Guards and .await"
track = "tokio"
order = 2
source = "https://tokio.rs/tokio/tutorial/shared-state#holding-a-mutexguard-across-an-await"
summary = "A std MutexGuard held across an .await makes the task non-Send. Drop it first, or hide the locking inside non-async methods."
+++

# Guards and `.await`

The previous chapter said everything a task holds across an `.await` must be `Send`. A `std::sync::MutexGuard` is **not** `Send`: a lock must be released by the thread that took it. So this doesn't compile:

```rust,does_not_compile title="A guard held across an .await"
use std::sync::{Arc, Mutex};
use tokio::task::yield_now;

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    let mut lock = mutex.lock().unwrap();
    *lock += 1;

    yield_now().await;
} // `lock` is dropped here, after the .await

#[tokio::main]
async fn main() {
    let mutex = Arc::new(Mutex::new(0));
    tokio::spawn(async move { increment_and_do_stuff(&mutex).await }).await.unwrap();
}
```

Even setting `Send` aside, holding a blocking lock while the task is paused is a bad idea: every other task that wants the lock would block its whole worker thread until this one resumes.

## Fix 1: end the guard's scope before the `.await`

```rust,runnable title="Release the lock first"
use std::sync::{Arc, Mutex};
use tokio::task::yield_now;

async fn increment_and_do_stuff(mutex: &Mutex<i32>) {
    {
        let mut lock = mutex.lock().unwrap();
        *lock += 1;
    } // `lock` is dropped here

    yield_now().await;
}

#[tokio::main]
async fn main() {
    let mutex = Arc::new(Mutex::new(0));
    let m = Arc::clone(&mutex);
    tokio::spawn(async move { increment_and_do_stuff(&m).await }).await.unwrap();
    println!("{}", mutex.lock().unwrap());
}
```

A block makes the lock's lifetime obvious to anyone reading the code, too.

## Fix 2: put the locking in a struct with non-async methods

The tutorial's favourite fix is to wrap the mutex in a type whose methods aren't `async`. A method that isn't async can't contain an `.await`, so its guard can never be held across one:

```rust,runnable title="Locking inside sync methods"
use std::sync::Mutex;
use tokio::task::yield_now;

struct CanIncrement {
    mutex: Mutex<i32>,
}

impl CanIncrement {
    // Not async, so no .await can happen while the lock is held.
    fn increment(&self) {
        let mut lock = self.mutex.lock().unwrap();
        *lock += 1;
    }
}

async fn increment_and_do_stuff(can_incr: &CanIncrement) {
    can_incr.increment();
    yield_now().await;
}

#[tokio::main]
async fn main() {
    let c = CanIncrement { mutex: Mutex::new(0) };
    increment_and_do_stuff(&c).await;
    println!("{}", c.mutex.lock().unwrap());
}
```

## Fix 3: `tokio::sync::Mutex`

When the lock really must stay held across an `.await`, use Tokio's async mutex. Its `lock()` is itself `.await`ed, and its guard is `Send`. It costs more, so it's the last resort, not the default.
