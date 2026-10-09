+++
id = "book.enums.option"
chapter = "book.enums"
requires = []
title = "Option Instead of Null"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values"
summary = "Rust has no null. A value that might be missing has the type Option<T>."
+++

# `Option`: A Value That Might Be Missing

Lots of values can be "nothing": the first item of an empty list, a user who hasn't set a nickname, the result of dividing by zero. Many languages represent "nothing" with **null**, a special value that can sit in any variable.

Null's inventor, Tony Hoare, called it his "billion-dollar mistake". The problem is that a null looks like a normal value until you use it, and then the program crashes. Every value *might* be null, so you'd have to check everywhere, and nobody does.

Rust has **no null**. Instead, the standard library has a tiny enum:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>` is either `Some(value)` or `None`. The `<T>` means it can wrap any type: `Option<i32>`, `Option<String>` and so on (you'll learn this syntax, *generics*, in chapter 10). It's used so often that `Some` and `None` work without writing `Option::`.

```rust,runnable title="Some values and no value"
fn main() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    println!("{some_number:?} {some_char:?} {absent_number:?}");
}
```

For `None`, Rust can't guess what type is missing, so you write the type: `Option<i32>`.

## Why this is better than null

An `Option<i32>` is **not** an `i32`. They're different types, so the compiler won't let you use one where the other is expected:

```rust,does_not_compile title="An Option isn't the number inside it"
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
    println!("{sum}");
}
```

That's the whole trick. A plain `i8` is **guaranteed** to be a real number, so you never need to check it. Only values whose type is `Option` might be missing, and the compiler makes you handle the missing case before you can use the value. The "forgot to check for null" bug can't happen.

## Returning an Option

Functions return `Option` when they might not have an answer. The standard library does this a lot. For example, `.first()` on a list returns `None` when the list is empty:

```rust,editable title="first() returns an Option"
fn main() {
    let scores = vec![90, 72, 85];
    let empty: Vec<i32> = vec![];
    println!("{:?}", scores.first());
    println!("{:?}", empty.first());
}
```

Your own functions can return `Some(...)` or `None` the same way. How do you get the value *out* of a `Some`? That's what `match` is for, starting in the next concept.
