+++
id = "book.patterns.refutability"
chapter = "book.patterns"
requires = []
title = "Refutability: Whether a Pattern Might Fail"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch19-02-refutability.html"
summary = "Irrefutable patterns always match; refutable ones might not. Each place that takes a pattern accepts one kind."
+++

# Refutability: Whether a Pattern Might Fail to Match

Patterns come in two kinds:

- **Irrefutable** patterns match **every** possible value. `x` in `let x = 5;` and `(a, b)` for a tuple of two can't fail.
- **Refutable** patterns can **fail** for some values. `Some(x)` doesn't match `None`; `0` doesn't match `5`.

Each place that uses patterns accepts one kind:

| Accepts only irrefutable | Accepts refutable |
| --- | --- |
| `let`, function parameters, `for` loops | `if let`, `while let`, `let...else`, `match` arms (all but the last can be refutable) |

The rule makes sense: a plain `let` has no "what if it doesn't match?" branch, so it can't take a pattern that might not match.

## A refutable pattern in `let`

```rust,does_not_compile title="let with a pattern that might not match"
fn main() {
    let some_option_value: Option<i32> = None;
    let Some(x) = some_option_value;
    println!("{x}");
}
```

The error, `E0005: refutable pattern in local binding`, says `None` isn't covered, and suggests the fix: `let...else` (or `if let`), which **does** say what to do when it doesn't match:

```rust,editable title="Fixed with let...else"
fn main() {
    let some_option_value: Option<i32> = Some(3);
    let Some(x) = some_option_value else {
        println!("nothing to do");
        return;
    };
    println!("got {x}");
}
```

## An irrefutable pattern in `if let`

The opposite mistake isn't an error, just pointless: `if let` with a pattern that always matches. Rust warns you:

```rust,runnable title="if let that can't fail"
fn main() {
    let x = 5;
    if let y = x {
        println!("{y}");
    }
}
```

Run it and look for the `irrefutable_let_patterns` warning: the `if` adds nothing, so use a plain `let`.

You'll rarely think about refutability directly, but knowing the term makes error messages like E0005 instantly understandable.
