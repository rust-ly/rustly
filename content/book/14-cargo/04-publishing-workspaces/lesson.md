+++
id = "book.cargo.publishing-workspaces"
chapter = "book.cargo"
requires = []
title = "Versions, Publishing and Workspaces"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html"
summary = "Semantic versioning, publishing to crates.io, workspaces of several crates, and cargo install."
+++

# Versions, Publishing and Workspaces

## Semantic versioning

Every crate on crates.io has a version like `1.4.2`: **MAJOR.MINOR.PATCH**. The rules, called **semantic versioning** (SemVer), tell users what an upgrade might break:

- **PATCH** (`1.4.2` → `1.4.3`): bug fixes only.
- **MINOR** (`1.4.2` → `1.5.0`): new features, nothing removed or changed incompatibly.
- **MAJOR** (`1.4.2` → `2.0.0`): breaking changes.

When you write `rand = "0.8.5"` in `Cargo.toml`, Cargo reads it as `^0.8.5`, a **caret requirement**: "any version compatible with 0.8.5". For versions 1.0 and up, compatible means the same major version and at least that version (`^1.4.2` allows `1.9.0` but not `2.0.0`). Before 1.0, the *minor* number acts as the major one, so `^0.8.5` allows `0.8.9` but not `0.9.0`.

`Cargo.lock` records the exact versions chosen, so everyone building the project gets the same ones until someone runs `cargo update`.

## Publishing a crate

1. Create an account on crates.io and run `cargo login` with your API token.
2. Fill in `Cargo.toml`: a unique `name`, `version`, `description` and `license` (e.g. `"MIT OR Apache-2.0"`).
3. Run `cargo publish`.

Publishing is **permanent**: a version can never be deleted or overwritten, so code that depends on it keeps building. If you publish a broken version, `cargo yank --vers 1.0.1` stops new projects from choosing it, without breaking projects that already use it.

## Workspaces

A **workspace** is a set of crates that share one `Cargo.lock` and one `target` folder, so they build together and agree on dependency versions:

```toml
[workspace]
members = ["adder", "add_one"]
resolver = "3"
```

```rust
// adder/src/main.rs uses the sibling crate, declared in adder/Cargo.toml
// with `add_one = { path = "../add_one" }`
fn main() {
    let num = 10;
    println!("{num} plus one is {}!", add_one::add_one(num));
}
```

`cargo test -p add_one` runs one member's tests; `cargo test` at the top runs them all. Rustly is a workspace: `crates/shared`, `crates/content`, `crates/runner` and `frontend` are members, which is why the frontend and the API can share types.

## Installing binaries

`cargo install ripgrep` downloads, builds and installs a binary crate into `~/.cargo/bin`. And any binary named `cargo-something` on your `PATH` becomes a Cargo subcommand, `cargo something`, which is how tools like `cargo-watch` extend Cargo.

```rust,runnable title="Parsing a version"
fn main() {
    let version = "1.4.2";
    let parts: Vec<u32> = version.split('.').map(|p| p.parse().unwrap()).collect();
    println!("major {}, minor {}, patch {}", parts[0], parts[1], parts[2]);
}
```
