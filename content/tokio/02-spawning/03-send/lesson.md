+++
id = "tokio.spawning.send"
chapter = "tokio.spawning"
requires = []
title = "The Send Bound"
track = "tokio"
order = 3
source = "https://tokio.rs/tokio/tutorial/spawning#send-bound"
summary = "A task can move between threads at any .await, so everything it holds across an .await must be Send."
+++

# The `Send` Bound

Tasks spawned with `tokio::spawn` must also be `Send`. The multi-threaded runtime moves tasks between worker threads, and it can do that every time a task pauses at an `.await`.

When a task pauses, all the variables it will need afterwards are stored inside the task's future. So the rule is precise: **everything held across an `.await` must be `Send`**. Values that are created and dropped between two `.await`s never get moved, so they don't count.

## `Rc` across an `.await`

`Rc` isn't `Send` (its reference count isn't thread-safe), so this works, because `rc` is dropped before the `.await`:

```rust,runnable title="Rc dropped before the .await"
use std::rc::Rc;
use tokio::task::yield_now;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // The scope makes `rc` drop before the .await.
        {
            let rc = Rc::new("hello");
            println!("{rc}");
        }

        // `rc` is no longer used, so it isn't held across the .await.
        yield_now().await;
    })
    .await
    .unwrap();
}
```

But this doesn't compile, because `rc` is still needed after the `.await`:

```rust,does_not_compile title="Rc held across the .await"
use std::rc::Rc;
use tokio::task::yield_now;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let rc = Rc::new("hello");

        // `rc` is used after the .await, so it's kept in the task's state.
        yield_now().await;

        println!("{rc}");
    })
    .await
    .unwrap();
}
```

The error says ``future cannot be sent between threads safely`` and points at the `.await` where `rc` is held. `yield_now` is an `.await` that does nothing except give other tasks a turn.

## The usual fixes

- **Drop it first:** finish with the value before the `.await`, using a `{ }` block if you need to.
- **Use the thread-safe version:** `Arc` instead of `Rc`, `std::sync::Mutex` or `tokio::sync::Mutex` instead of `RefCell`. The shared-state chapter goes into which mutex to pick.

The same error often comes from holding a `std::sync::MutexGuard` across an `.await`. The guard isn't `Send`, so the fix is the same: let the guard go before you await.
