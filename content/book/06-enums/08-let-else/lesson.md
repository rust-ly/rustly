+++
id = "book.enums.let-else"
chapter = "book.enums"
requires = []
title = "let...else"
track = "book"
order = 8
source = "https://doc.rust-lang.org/book/ch06-03-if-let.html#staying-on-the-happy-path-with-letelse"
summary = "Bind a value if the pattern matches, and leave the function early if it doesn't."
+++

# Staying on the Happy Path with `let...else`

A common shape for a function is: "get a value; if it isn't there, stop; otherwise carry on with it". With `if let`, the "carry on" part ends up nested inside the block, and nesting piles up quickly:

```rust,runnable title="Nested if let"
fn describe_age(age: Option<u32>) -> String {
    if let Some(years) = age {
        if years >= 18 {
            format!("adult, {years}")
        } else {
            format!("minor, {years}")
        }
    } else {
        String::from("age unknown")
    }
}

fn main() {
    println!("{}", describe_age(Some(30)));
    println!("{}", describe_age(None));
}
```

`let...else` flips this around. It binds the pattern's variables **if** the value matches, and otherwise runs the `else` block:

```rust,editable title="The same function with let...else"
fn describe_age(age: Option<u32>) -> String {
    let Some(years) = age else {
        return String::from("age unknown");
    };
    if years >= 18 {
        format!("adult, {years}")
    } else {
        format!("minor, {years}")
    }
}

fn main() {
    println!("{}", describe_age(Some(30)));
    println!("{}", describe_age(None));
}
```

After the `let...else` line, `years` is a plain `u32` that's available for the rest of the function. The missing case is dealt with up front, and the main logic stays at the top level: the "happy path".

## The else block must leave

If the pattern doesn't match, there's no value to bind, so the code after the `let` can't run. That's why the `else` block **must** exit: `return` from the function, `break` or `continue` in a loop, or panic. If it could fall through, Rust refuses to compile it:

```rust,does_not_compile title="An else block that doesn't leave"
fn main() {
    let age: Option<u32> = None;
    let Some(years) = age else {
        println!("no age");
    };
    println!("{years}");
}
```

The error, `E0308: ... expected !, found ()`, is Rust's way of saying the `else` block must never finish normally. (`!` is the type of code that never returns.)

## In loops

Inside a loop, `continue` is a handy way to skip items that don't match:

```rust,runnable title="Skipping missing values"
fn main() {
    let scores = [Some(7), None, Some(9), None];
    let mut total = 0;
    for score in scores {
        let Some(points) = score else {
            continue;
        };
        total += points;
    }
    println!("total: {total}");
}
```
