+++
id = "tokio.hello.async-fns"
chapter = "tokio.hello"
requires = []
title = "Async Functions and .await"
track = "tokio"
order = 2
source = "https://tokio.rs/tokio/tutorial/hello-tokio"
summary = "Futures are lazy: nothing runs until you .await. Writing async methods for an in-memory key-value client."
+++

# Async Functions and `.await`

The Tokio tutorial's first program connects to a mini-Redis server, sets `hello` to `world`, and reads it back. Each call waits for the network, so each one is an `async fn` that you `.await`:

```rust
let mut client = client::connect("127.0.0.1:6379").await?;
client.set("hello", "world".into()).await?;
let result = client.get("hello").await?;
```

We'll build the same thing in memory. A `sleep` stands in for the network round trip.

## Futures are lazy

Calling an `async fn` doesn't run its body. It returns a future, and the body only runs when that future is **awaited** (or handed to the runtime some other way):

```rust,runnable title="Nothing happens until .await"
async fn say(word: &str) {
    println!("{word}");
}

#[tokio::main]
async fn main() {
    let first = say("first?");
    say("second").await;
    first.await;
}
```

`first` is created before `say("second")`, but it prints last, because it's awaited last. This is different from threads, where `thread::spawn` starts work immediately. In Tokio, spawning a task is how you start work in the background; that's the next chapter.

## Async methods

Methods can be `async` too, and they can borrow `self` across an `.await` like any other code:

```rust,editable title="An in-memory client"
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

struct Client {
    data: HashMap<String, String>,
}

impl Client {
    async fn set(&mut self, key: &str, value: &str) {
        sleep(Duration::from_millis(10)).await; // pretend this is the network
        self.data.insert(key.to_string(), value.to_string());
    }

    async fn get(&self, key: &str) -> Option<String> {
        sleep(Duration::from_millis(10)).await;
        self.data.get(key).cloned()
    }
}

#[tokio::main]
async fn main() {
    let mut client = Client { data: HashMap::new() };
    client.set("hello", "world").await;
    println!("got {:?}", client.get("hello").await);
    println!("got {:?}", client.get("bye").await);
}
```

Each `.await` gives the runtime a chance to run something else while this future waits. Here there's nothing else, so the program simply takes about 30 ms.

## Testing time without waiting

Real sleeps make tests slow. `#[tokio::test(start_paused = true)]` starts the test with Tokio's clock **paused**: whenever every task is waiting on a timer, the clock jumps straight to the next deadline. A 10-second `sleep` finishes instantly, and `tokio::time::Instant` still reports that exactly 10 seconds passed. The graders in this track use it to check timing without making you wait.

## `.await` only works inside async code

`.await` is only allowed in an `async fn` or `async` block. A plain `fn` has to hand the future to a runtime instead:

```rust,does_not_compile title=".await in a plain function"
async fn answer() -> u32 {
    42
}

fn main() {
    let n = answer().await;
    println!("{n}");
}
```
