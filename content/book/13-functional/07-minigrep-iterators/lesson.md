+++
id = "book.functional.minigrep-iterators"
chapter = "book.functional"
requires = []
title = "Improving minigrep, and Performance"
track = "book"
order = 7
source = "https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html"
summary = "Rewrite minigrep's Config::build and search with iterators, and see why iterators cost nothing at run time."
+++

# Improving Our I/O Project

With closures and iterators, two parts of minigrep from chapter 12 get shorter and clearer.

## `Config::build` without `clone`

`Config::build` took a slice `&[String]` and **cloned** the strings out of it, because it didn't own them. If it takes the **iterator** from `env::args()` instead, it owns each `String` as it comes out and can move it straight into the `Config`:

```rust,editable title="Config::build taking an iterator"
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip the program name

        let Some(query) = args.next() else {
            return Err("Didn't get a query string");
        };
        let Some(file_path) = args.next() else {
            return Err("Didn't get a file path");
        };
        Ok(Config { query, file_path })
    }
}

fn main() {
    let args = vec![String::from("minigrep"), String::from("to"), String::from("poem.txt")];
    match Config::build(args.into_iter()) {
        Ok(c) => println!("query {:?}, file {:?}", c.query, c.file_path),
        Err(e) => println!("error: {e}"),
    }
}
```

- `impl Iterator<Item = String>` accepts any iterator of `String`s: `env::args()` in the real program, or `vec.into_iter()` in a test.
- `args` is `mut` because `next` advances it.
- Each missing argument now gets its own clear error, with no indexing and no panics.

In `main`, the call becomes `Config::build(env::args())`.

## `search` as a chain

The loop that collected matching lines is exactly a `filter`:

```rust,runnable title="search with filter"
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn main() {
    let contents = "Rust:\nsafe, fast, productive.\nPick three.";
    println!("{:?}", search("duct", contents));
}
```

There's no mutable `results` vector any more, which removes a whole category of mistakes, and the code reads as a description of the result: "the lines of `contents` that contain `query`".

## Loops or iterators: which is faster?

The Book benchmarks both versions of `search` on the full text of *The Adventures of Sherlock Holmes*. The iterator version is, if anything, **slightly faster**.

Iterators are one of Rust's **zero-cost abstractions**: the compiler turns the chain of `filter`, `map` and `collect` into the same tight machine code a hand-written loop would produce, often unrolling loops and removing bounds checks along the way. So you can write the clearer version without paying for it.

Rust programmers generally prefer the iterator style. Use a plain loop when it's genuinely clearer, for example when the body has several steps with side effects.
