+++
id = "tokio.shared-state.arc-mutex"
chapter = "tokio.shared-state"
requires = []
title = "Arc<Mutex<HashMap>>"
track = "tokio"
order = 1
source = "https://tokio.rs/tokio/tutorial/shared-state"
summary = "Every connection task needs the same database. Share it with an Arc, and guard it with a Mutex."
+++

# `Arc<Mutex<HashMap>>`

The mini-Redis server spawns a task per connection, and every connection reads and writes the same key-value data. So the tasks need **shared state**. Tokio offers two common approaches:

- Guard the data with a **mutex**, and share it with `Arc`.
- Give the data to one **manager task** and send it messages. That's the next chapter.

For a simple store, the mutex is the easy choice.

## The shared database

Wrap the map in a `Mutex` so only one task changes it at a time, and in an `Arc` so every task can own a handle to it. A type alias keeps the signatures short:

```rust,editable title="A shared database"
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
async fn main() {
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = Vec::new();
    for i in 0..3 {
        let db = Arc::clone(&db);
        handles.push(tokio::spawn(async move {
            let mut map = db.lock().unwrap();
            map.insert(format!("key{i}"), format!("value{i}"));
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }

    let map = db.lock().unwrap();
    let mut keys: Vec<_> = map.keys().collect();
    keys.sort();
    println!("{keys:?}");
}
```

`db.lock()` waits until no one else holds the lock, then returns a **guard** that derefs to the map. The lock is released when the guard is dropped, at the end of its scope.

## Which `Mutex`?

This is `std::sync::Mutex`, the ordinary blocking mutex, not `tokio::sync::Mutex`. That surprises people, but the tutorial recommends it: the critical sections here are tiny (an insert or a lookup), and a blocking lock that's held for nanoseconds is faster than an async one.

Reach for `tokio::sync::Mutex` only when you need to **hold the lock across an `.await`**, for example while writing to a shared connection. It's slower, because locking is itself an async operation.

If a single mutex becomes a bottleneck under heavy load, the tutorial's next idea is **sharding**: several mutexes, each guarding part of the keys, picked by hashing the key.

## `lock()` returns a `Result`

`lock()` fails only if another thread **panicked** while holding the lock, leaving the data possibly half-updated (a "poisoned" mutex). Most code calls `.unwrap()`, so a panic in one task becomes a panic in the next one to touch the data.

```rust,panics title="A poisoned mutex"
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(0));

    let db2 = Arc::clone(&db);
    let crashed = tokio::spawn(async move {
        let _guard = db2.lock().unwrap();
        panic!("crashed while holding the lock");
    })
    .await;
    println!("task panicked: {}", crashed.is_err());

    // The lock is poisoned now, so this unwrap panics.
    let value = db.lock().unwrap();
    println!("{value}");
}
```
