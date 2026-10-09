+++
id = "book.patterns.pattern-places"
chapter = "book.patterns"
requires = []
title = "All the Places Patterns Can Be Used"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch19-01-all-the-places-for-patterns.html"
summary = "match arms, let, if let, while let, for loops and function parameters all use patterns."
+++

# All the Places Patterns Can Be Used

A **pattern** describes the **shape** of a value and can pull pieces out of it. You've used them since chapter 6, often without noticing. Here's every place they appear.

## `match` arms

`match VALUE { PATTERN => EXPRESSION, ... }`. The patterns must be **exhaustive** together, which is why a final `_` is common.

## `let` statements

Every `let` is a pattern match! In `let x = 5;`, `x` is a pattern that matches anything and binds it. That's why this works:

```rust,runnable title="let destructures a tuple"
fn main() {
    let (x, y, z) = (1, 2, 3);
    println!("x = {x}, y = {y}, z = {z}");
}
```

The pattern `(x, y, z)` must have the same shape as the value: a tuple of three.

## `if let`, `else if` and `else if let`

You can mix them in one chain, each checking something different:

```rust,editable title="Mixing if let and if"
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favourite colour, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background colour");
        } else {
            println!("Using orange as the background colour");
        }
    } else {
        println!("Using blue as the background colour");
    }
}
```

Notice `if let Ok(age) = age`: the new `age` (a `u8`) **shadows** the old one (a `Result`) inside that block.

## `while let` and `for` loops

`while let` loops as long as a pattern matches (chapter 17). In a `for` loop, the part after `for` is a pattern too, which is why `for (index, value) in v.iter().enumerate()` can unpack each pair:

```rust,runnable title="Patterns in a for loop"
fn main() {
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
}
```

## Function parameters

Parameters are patterns, so a function can unpack a tuple right in its signature (closure parameters work the same way):

```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}
```

Called as `print_coordinates(&(3, 5))`, the pattern `&(x, y)` matches the reference to a tuple and binds `x = 3`, `y = 5`.
