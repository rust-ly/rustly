+++
id = "book.getting-started.hello-world"
chapter = "book.getting-started"
requires = []
title = "Hello, World!"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch01-02-hello-world.html"
summary = "The smallest Rust program: a `main` function and a `println!` call."
+++

# Hello, World!

Every Rust program starts running at a function called `main`. Here's the classic first program. Press **Run** to try it:

```rust,runnable title="Hello, world"
fn main() {
    println!("Hello, world!");
}
```

There are only a few pieces here:

- `fn main() { ... }` declares a function named `main` that takes no arguments. The code between the curly braces is its **body**.
- `println!("Hello, world!");` prints a line of text. The line ends with a semicolon, like most lines of Rust.
- Code is indented with four spaces. `rustfmt`, the standard formatter, does this for you.

## `println!` is a macro

Notice the `!`. `println!` isn't an ordinary function. It's a **macro**: code that writes more code for you when the program compiles. For now, all you need to know is that a name ending in `!` is a macro. Leave out the `!` and Rust looks for a function called `println`, which doesn't exist:

```rust,does_not_compile title="Forgetting the !"
fn main() {
    println("Hello, world!");
}
```

Run it and read the error. The compiler spots that you meant the macro and tells you to add the `!`.

## Filling in values

`println!` can fill values into the text. Put a variable's name in curly braces, or use empty braces `{}` and pass the values after the text, in order:

```rust,editable title="Filling in values"
fn main() {
    let name = "Ferris";
    let year = 2015;
    println!("Hello, {name}!");
    println!("Rust 1.0 came out in {}.", year);
}
```

Change `name` to your own name and run it again.

## Building a `String` instead of printing

Sometimes you want the text as a value, not printed straight away. `format!` works exactly like `println!`, but it returns a `String`. Functions can hand values like this back: the `-> String` after the arguments says what comes back, and the last expression in the body, written without a semicolon, is the value returned:

```rust,runnable title="Returning a String"
fn shout(word: &str) -> String {
    format!("{word}!!!")
}

fn main() {
    let message = shout("Rust");
    println!("{message}");
}
```

## Quick recap

- Programs start at `fn main()`.
- A name ending in `!`, like `println!`, is a macro.
- `{}` and `{name}` fill values into text. `format!` gives you a `String` instead of printing it.

The checkpoint below asks you to write your first function. Pass it to unlock the next concept.
