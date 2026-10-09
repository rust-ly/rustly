+++
id = "book.minigrep.ignore-case"
chapter = "book.minigrep"
requires = []
title = "Case-Insensitive Search and Environment Variables"
track = "book"
order = 6
source = "https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html"
summary = "Add a case-insensitive search, and switch it on with an IGNORE_CASE environment variable."
+++

# Working with Environment Variables

The last feature: an option to ignore upper and lower case, so `rUsT` matches "Rust" and "Trust". Users turn it on with an **environment variable**, a setting that lives in the terminal session rather than on the command line:

```sh
IGNORE_CASE=1 cargo run -- to poem.txt
```

## The case-insensitive search (test first)

Lower-case both the query and each line before comparing. `to_lowercase()` returns a new `String`, so `query` is now owned, and `contains` takes `&query`:

```rust,editable title="search_case_insensitive"
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

fn main() {
    let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
    println!("{:?}", search_case_insensitive("rUsT", contents));
}
```

The results are still the **original** lines (`line`), not the lower-cased copies, so the user sees their text unchanged.

## Reading the environment variable

`std::env::var` returns a `Result`: `Ok(value)` if the variable is set, `Err` if not. minigrep only cares **whether** it's set, so `.is_ok()` is enough:

```rust
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}
```

Then `run` picks the right search:

```rust
let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
} else {
    search(&config.query, &contents)
};
```

## Testing without the environment

Reading `env::var` inside `build` makes it depend on the outside world again: a test's result would change depending on the terminal it ran in. In this chapter's exercises, the variable's value is **passed in** as an `Option<&str>` (`Some("1")` if set, `None` if not), which keeps everything testable. In a real program, `main` would read `env::var("IGNORE_CASE").ok()` and pass that along.
