+++
id = "book.concurrency.move-threads"
chapter = "book.concurrency"
requires = []
title = "move Closures with Threads"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch16-01-threads.html#using-move-closures-with-threads"
summary = "A spawned thread might outlive the function that started it, so its closure must own its data."
+++

# Using `move` Closures with Threads

A thread's closure often needs data from the code that spawned it. Borrowing doesn't work:

```rust,does_not_compile title="A thread borrowing from main"
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });
    handle.join().unwrap();
}
```

The error, `E0373: closure may outlive the current function, but it borrows v`, is Rust preventing a real bug. The compiler can't know how long the new thread will run. `main` could drop `v` (or even return) while the thread is still reading it, leaving it with a dangling reference.

## `move` gives the thread ownership

Adding `move` makes the closure take ownership of `v`. The thread owns its data outright, so it can run as long as it likes:

```rust,editable title="Moving data into the thread"
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    handle.join().unwrap();
}
```

Once moved, `v` belongs to the thread, and `main` can't use it any more. Try adding `println!("{v:?}")` after the spawn: the compiler stops you, because `main` might drop data the thread owns.

## Giving each thread its own data

A common pattern is to split work up and **move** one piece into each thread. Here every thread gets its own chunk of words to count:

```rust,runnable title="One chunk per thread"
use std::thread;

fn main() {
    let words = vec!["apple", "banana", "cherry", "date", "elder", "fig"];
    let mut handles = Vec::new();
    for chunk in words.chunks(2) {
        let chunk: Vec<String> = chunk.iter().map(|w| w.to_string()).collect();
        handles.push(thread::spawn(move || {
            let letters: usize = chunk.iter().map(|w| w.len()).sum();
            letters
        }));
    }
    let total: usize = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("total letters: {total}");
}
```

Each chunk is turned into owned `String`s before the `move`, so each thread owns exactly its own words, and nothing is shared.
