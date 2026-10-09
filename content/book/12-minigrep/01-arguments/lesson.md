+++
id = "book.minigrep.arguments"
chapter = "book.minigrep"
requires = []
title = "Accepting Command Line Arguments"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html"
summary = "Read the words typed after the program's name with std::env::args."
+++

# Accepting Command Line Arguments

This chapter builds a real tool: a simple version of `grep`, the classic command that searches files for lines containing some text. You'll run it like this:

```sh
cargo run -- searchstring example-filename.txt
```

Everything after `--` is passed to **your** program as **command line arguments**, rather than to Cargo.

## Reading the arguments

`std::env::args()` returns an iterator over the arguments, and `.collect()` turns it into a `Vec<String>`. (Iterators are chapter 13; for now, `collect` is the magic word that gathers the items into a collection.)

```rust,runnable title="Printing the arguments"
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{args:?}");
}
```

On your own machine, `cargo run -- needle haystack` prints three items. The first, `args[0]`, is always the **program's own path**, like `"target/debug/minigrep"`. The words you typed come after it. (On the Playground there are no extra arguments, so you'll only see the program path.)

So the two values minigrep needs are at index 1 and 2:

```rust
let query = &args[1];
let file_path = &args[2];
```

## Working without a terminal

The Playground can't pass arguments, read files or set environment variables. So in this chapter's exercises, the program's inputs (the argument list, the file's contents) are **passed into functions** as ordinary values.

That isn't just a workaround. Code that takes its inputs as parameters, instead of reaching out to the outside world itself, is easier to test, which is exactly the lesson of the next few concepts. Only `main` should touch `env::args()`; everything else gets plain values:

```rust,editable title="Pretend arguments"
fn describe(args: &[String]) {
    println!("program: {}", args[0]);
    println!("query:   {}", args[1]);
    println!("file:    {}", args[2]);
}

fn main() {
    let args = vec![
        String::from("minigrep"),
        String::from("frog"),
        String::from("poem.txt"),
    ];
    describe(&args);
}
```

## A common mistake

Indexing past the end of the argument list panics, which is what happens if someone runs the program without enough arguments. You'll fix that properly two concepts from now.

```rust,panics title="Not enough arguments"
fn main() {
    let args = vec![String::from("minigrep")];
    let query = &args[1];
    println!("{query}");
}
```
