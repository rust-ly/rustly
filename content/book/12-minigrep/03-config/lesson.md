+++
id = "book.minigrep.config"
chapter = "book.minigrep"
requires = []
title = "Refactoring: A Config with Proper Errors"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html"
summary = "Group related values into a struct, and replace a panic on bad input with a Result."
+++

# Refactoring to Improve Modularity and Error Handling

The first version of minigrep has several problems the Book points out:

1. `main` does everything, so it's hard to read and hard to test.
2. `query` and `file_path` are loose variables, even though they belong together.
3. Too few arguments causes an `index out of bounds` panic, a message that means nothing to a user.

## Group the values in a struct

`query` and `file_path` together are the program's **configuration**, so give them a struct, and a constructor that builds one from the arguments:

```rust,runnable title="A Config struct"
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}

fn main() {
    let args = vec![String::from("minigrep"), String::from("frog"), String::from("poem.txt")];
    let config = Config::new(&args);
    println!("Searching for {} in {}", config.query, config.file_path);
}
```

The `clone` calls copy the strings so `Config` can own them. That costs a little time and memory, and the Book is happy with that trade: simple, clear code first, optimisation later if it's ever needed. (Chapter 13 removes these clones with an iterator.)

## Return a Result instead of panicking

Too few arguments is a **user error**, not a bug, so as chapter 9 taught, it calls for a `Result`. By convention, a constructor named `new` shouldn't fail, so the Book renames it `build`:

```rust,editable title="Config::build returns a Result"
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

fn main() {
    let args = vec![String::from("minigrep")];
    match Config::build(&args) {
        Ok(config) => println!("query: {}", config.query),
        Err(e) => println!("Problem parsing arguments: {e}"),
    }
}
```

The error type is `&'static str`: a string literal, which lives for the whole program (chapter 10's `'static`).

## Exiting cleanly

In the real program, `main` reports the error and stops with a non-zero **exit code**, which tells the shell the program failed:

```rust
use std::process;

let config = Config::build(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {err}");
    process::exit(1);
});
```

`unwrap_or_else` takes a **closure** (the `|err| { ... }` part, an inline function you'll meet properly in chapter 13). It runs only if `build` returned an `Err`.
