+++
id = "book.cargo.release-profiles"
chapter = "book.cargo"
requires = []
title = "Release Profiles"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch14-01-release-profiles.html"
summary = "dev and release builds, opt-level, and why integer overflow behaves differently in each."
+++

# Customising Builds with Release Profiles

Cargo has two main **profiles**, sets of settings for compiling your code:

- **`dev`**, used by `cargo build` and `cargo run`: compiles quickly, with checks that help while developing.
- **`release`**, used by `cargo build --release`: takes longer to compile, but produces faster code for shipping.

You can change a profile's settings in `Cargo.toml`. The most common setting is `opt-level`, how hard the compiler optimises, from 0 to 3:

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

Rustly's own `Cargo.toml` goes further for the API: `lto = "fat"` and `codegen-units = 1` make the compiler optimise across the whole program, trading build time for speed.

## A difference you can see: integer overflow

A `u8` holds 0 to 255. What's `255 + 1`?

- In **dev** builds, Rust checks every arithmetic operation and **panics** on overflow, so you notice the bug.
- In **release** builds, the checks are off for speed, and the value **wraps around** to 0.

The Playground runs in dev (debug) mode, so here it panics:

```rust,panics title="Overflow panics in a debug build"
fn main() {
    let x: u8 = 255;
    let one: u8 = "1".parse().unwrap(); // hides the 1 from the compiler's own check
    let y = x + one;
    println!("{y}");
}
```

Relying on either behaviour is a bug. When overflow is possible, say what you want explicitly, and it then behaves the same in every profile:

```rust,editable title="Saying what should happen on overflow"
fn main() {
    let x: u8 = 250;
    println!("checked:    {:?}", x.checked_add(10));    // None on overflow
    println!("wrapping:   {}", x.wrapping_add(10));     // wraps around: 4
    println!("saturating: {}", x.saturating_add(10));   // stops at the max: 255
    println!("checked ok: {:?}", x.checked_add(5));     // Some(255)
}
```

- `checked_*` returns an `Option`: `None` if it would overflow.
- `wrapping_*` wraps around on purpose (useful for hashes and counters that cycle).
- `saturating_*` stops at the minimum or maximum value.
