+++
id = "book.concurrency.send-sync"
chapter = "book.concurrency"
requires = []
title = "Send and Sync"
track = "book"
order = 6
source = "https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html"
summary = "Two marker traits decide what can move to another thread and what can be shared between threads."
+++

# Extensible Concurrency with `Send` and `Sync`

Surprisingly little of Rust's concurrency is built into the **language**: threads, channels and mutexes are all in the standard library. What the language provides is two **marker traits**, `Send` and `Sync`, and the compiler checks them every time data crosses a thread boundary.

## `Send`: can be moved to another thread

A type is `Send` if ownership of it can be transferred to another thread. Almost every type is. `Rc<T>` is the famous exception, because its non-atomic count would break if two threads held clones. That's why the previous concept's `Rc` version failed to compile: `thread::spawn` requires its closure, and everything it captures, to be `Send`.

## `Sync`: can be shared between threads

A type is `Sync` if it's safe to **reference** from several threads at once. Formally, `T` is `Sync` if `&T` is `Send`. `Mutex<T>` is `Sync`, which is what makes `Arc<Mutex<T>>` work. `RefCell<T>` and `Cell<T>` are **not** `Sync`, because their run-time borrow tracking isn't thread-safe.

## You rarely implement them yourself

`Send` and `Sync` are **auto traits**: a struct is automatically `Send` if all its fields are `Send`, and `Sync` if all its fields are `Sync`. So your types get the right answer for free:

```rust,runnable title="A struct of Send fields is Send"
use std::thread;

#[derive(Debug)]
struct Job {
    id: u32,
    name: String,
}

fn main() {
    let job = Job { id: 1, name: String::from("render") };
    let handle = thread::spawn(move || format!("finished {job:?}"));
    println!("{}", handle.join().unwrap());
}
```

Put one non-`Send` field in a struct, and the whole struct stops being `Send`:

```rust,does_not_compile title="One Rc field makes the struct not Send"
use std::rc::Rc;
use std::thread;

struct Job {
    shared_name: Rc<String>,
}

fn main() {
    let job = Job { shared_name: Rc::new(String::from("render")) };
    thread::spawn(move || {
        println!("{}", job.shared_name);
    })
    .join()
    .unwrap();
}
```

Implementing `Send` or `Sync` by hand requires `unsafe` code (chapter 20), and is only for people building new low-level concurrency tools.

## Why this is "fearless"

Because these checks happen at compile time, a whole category of concurrency bugs (sending a non-thread-safe pointer to another thread, sharing a `RefCell` between threads) simply can't make it into a running program. When code compiles, you can trust that data shared between threads is protected.

```rust,editable title="Checking a type at compile time"
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    assert_send::<String>();
    assert_sync::<std::sync::Mutex<i32>>();
    assert_send::<std::sync::Arc<i32>>();
    // Uncomment one of these and it won't compile:
    // assert_send::<std::rc::Rc<i32>>();
    // assert_sync::<std::cell::RefCell<i32>>();
    println!("all checks passed at compile time");
}
```
