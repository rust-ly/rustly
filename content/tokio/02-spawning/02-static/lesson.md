+++
id = "tokio.spawning.static"
chapter = "tokio.spawning"
requires = []
title = "The 'static Bound"
track = "tokio"
order = 2
source = "https://tokio.rs/tokio/tutorial/spawning#static-bounds"
summary = "A spawned task may outlive the code that spawned it, so it can't borrow local data. Move owned data in, or share it with Arc."
+++

# The `'static` Bound

`tokio::spawn` requires the future to be `'static`. A spawned task can keep running after the function that spawned it has returned, so it isn't allowed to hold references to that function's local variables. They might be gone by the time the task uses them.

This is the same rule as `thread::spawn` from the Book's concurrency chapter, and the compiler error looks familiar:

```rust,does_not_compile title="Borrowing a local in a task"
#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    let handle = tokio::spawn(async {
        println!("Here's a vec: {v:?}");
    });

    handle.await.unwrap();
}
```

The async block **borrows** `v`, and the compiler can't prove the task finishes before `v` is dropped. Awaiting the handle right away doesn't help: the compiler checks the type, not the timing.

## Fix 1: move the data in

`async move` moves `v` into the task, so the task owns it and nothing is borrowed:

```rust,runnable title="async move"
#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    let handle = tokio::spawn(async move {
        println!("Here's a vec: {v:?}");
    });

    handle.await.unwrap();
}
```

The catch: once moved, `v` belongs to the task, and `main` can't use it any more.

## Fix 2: share it with `Arc`

When several tasks need the same data, give each one its own `Arc`. Cloning an `Arc` only copies a pointer and bumps a reference count:

```rust,runnable title="One Arc per task"
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let names = Arc::new(vec!["ferris".to_string(), "corro".to_string()]);

    let mut handles = Vec::new();
    for i in 0..names.len() {
        let names = Arc::clone(&names);
        handles.push(tokio::spawn(async move { names[i].to_uppercase() }));
    }
    for handle in handles {
        println!("{}", handle.await.unwrap());
    }
    println!("main still has {} names", names.len());
}
```

`'static` here means "**owns** everything it uses", not "lives forever". A `String` moved into a task is `'static`; a `&String` pointing at a local isn't.
