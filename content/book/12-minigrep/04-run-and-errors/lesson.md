+++
id = "book.minigrep.run-and-errors"
chapter = "book.minigrep"
requires = []
title = "A run Function, lib.rs and stderr"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#extracting-logic-from-main"
summary = "Keep main small: move the logic into a run function in lib.rs, and print errors to stderr."
+++

# Extracting Logic from `main`

The Rust community has a standard shape for command line programs. `main` should be small enough to check by reading it:

- **`main.rs`** parses the arguments, calls `run`, and handles any error.
- **`lib.rs`** holds everything else: the `Config`, `run` and the actual logic, where it can be tested.

## A `run` function that returns errors

`run` does the work. Anything that can fail returns an error with `?` instead of panicking:

```rust
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}
```

`Box<dyn Error>` means "any kind of error" (chapter 9), so `?` works on the file error. `main` then handles whatever comes back:

```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
```

`run` returns `()` on success, so there's no value to unwrap; `if let Err(e)` handles just the failure.

## Errors go to stderr

Notice `eprintln!` instead of `println!`. Programs have two output streams: **standard output** (stdout) for results, and **standard error** (stderr) for error messages. If a user saves the results to a file with `cargo run -- to poem.txt > output.txt`, only stdout goes into the file, and errors still appear on screen where they'll be seen:

```rust,runnable title="println! vs eprintln!"
fn main() {
    println!("this is a result (stdout)");
    eprintln!("this is an error message (stderr)");
}
```

The Playground shows both, with stderr mixed in with the build messages.

## Testing `run` without files

Our version of `run` takes the contents as a parameter and **returns** the lines instead of printing them. That keeps it free of I/O, so it's easy to test:

```rust,editable title="A testable run"
struct Config {
    query: String,
}

fn run(config: &Config, contents: &str) -> Result<Vec<String>, String> {
    if contents.is_empty() {
        return Err(String::from("the file is empty"));
    }
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(&config.query) {
            results.push(line.to_string());
        }
    }
    Ok(results)
}

fn main() {
    let config = Config { query: String::from("nobody") };
    let poem = "I'm nobody! Who are you?\nAre you nobody, too?\nThen there's a pair of us";
    match run(&config, poem) {
        Ok(lines) => println!("{lines:?}"),
        Err(e) => eprintln!("Application error: {e}"),
    }
}
```
