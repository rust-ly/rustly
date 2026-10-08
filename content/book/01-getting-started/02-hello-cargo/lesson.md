+++
id = "book.getting-started.hello-cargo"
chapter = "book.getting-started"
requires = []
title = "Rust on Your Machine: rustup and Cargo"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch01-03-hello-cargo.html"
summary = "Install Rust with rustup, then let Cargo create, build and run your projects."
+++

# Rust on Your Machine: rustup and Cargo

Rustly runs your code for you, so you don't need anything installed to follow the course. When you're ready to build things on your own computer, here's how it works.

## Installing Rust

Rust is installed with **rustup**, which manages Rust versions for you. On macOS or Linux, run this in a terminal:

```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

On Windows, download the installer from [rustup.rs](https://rustup.rs). Then check that it worked:

```sh
rustc --version
cargo --version
```

Later on, `rustup update` brings you to the newest stable release, and `rustup doc` opens the docs offline, including this Book. The full steps are on the [installation page](https://doc.rust-lang.org/book/ch01-01-installation.html).

## Cargo, Rust's build tool

You *can* compile a single file with `rustc main.rs`, but almost every real project uses **Cargo**. It creates projects, builds them, downloads the libraries they depend on and runs their tests:

```sh
cargo new hello_cargo
cd hello_cargo
cargo run
```

`cargo new` makes a folder with two things in it:

- `src/main.rs`, which already contains the Hello, world program from the last concept.
- `Cargo.toml`, the project's settings file. Its `[package]` section holds the project's `name`, `version` and Rust `edition`, and `[dependencies]` lists the libraries it uses.

## The commands you'll use every day

| Command | What it does |
| --- | --- |
| `cargo build` | Compiles the project into `target/debug/` |
| `cargo run` | Builds the project if anything changed, then runs it |
| `cargo check` | Checks that the code compiles without producing a program. Much faster, so use it while writing |
| `cargo test` | Runs the project's tests |
| `cargo build --release` | Builds an optimized program into `target/release/` |

## Testing your code

Rustly checks your exercises the same way `cargo test` does. Under the hood, each exercise runs hidden tests that look like this. Run the snippet to see one pass:

```rust,runnable title="A test, like the ones checking your exercises"
fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    assert_eq!(double(21), 42);
    println!("double(21) is 42, so the check passed");
}
```

`assert_eq!` compares two values. If they're different, the program stops with a **panic** that shows both values:

```rust,panics title="A failing check"
fn double(n: i32) -> i32 {
    n + 2
}

fn main() {
    assert_eq!(double(21), 42);
}
```

## Quick recap

- rustup installs and updates Rust. Cargo creates, builds, runs and tests projects.
- `Cargo.toml` holds a project's name, version and dependencies.
- Use `cargo check` while writing, `cargo run` to try the program, and `cargo test` to run its tests.

The checkpoint below has you build the line Cargo prints while it compiles a package.
