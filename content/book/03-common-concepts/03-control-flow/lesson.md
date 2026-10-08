+++
id = "book.common-concepts.control-flow"
chapter = "book.common-concepts"
requires = []
title = "Control Flow"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch03-05-control-flow.html"
summary = "Branch with `if`, and repeat with `loop`, `while` and `for`."
+++

# Control Flow

## `if` expressions

`if` runs a block only when a condition is true, and `else if` and `else` cover the other cases. The condition **must be a `bool`**. Rust won't treat a number as true or false the way JavaScript or C do:

```rust,does_not_compile title="Conditions must be bool"
fn main() {
    let number = 3;
    if number {
        println!("number was three");
    }
}
```

Write the comparison you mean, such as `if number != 0`.

Because `if` is an expression, it produces a value, so you can use it on the right of a `let`. Both branches must produce the same type:

```rust,runnable title="if is an expression"
fn main() {
    let n = 7;
    let parity = if n % 2 == 0 { "even" } else { "odd" };
    println!("{n} is {parity}");
}
```

## Loops

Rust has three loops.

**`loop`** repeats until you `break`. A `break` can carry a value out of the loop, which is handy when you're retrying something until it succeeds:

```rust,runnable title="Returning a value from loop"
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result = {result}");
}
```

**`while`** checks a condition before each pass and stops when it becomes false.

**`for`** walks through every item in a collection or range. It's the loop you'll use most, because it can't run past the end of an array and doesn't need a counter. `1..4` is the range 1, 2, 3, and `1..=4` includes the 4. `.rev()` goes backwards:

```rust,editable title="Counting down with for"
fn main() {
    for n in (1..4).rev() {
        println!("{n}...");
    }
    println!("Liftoff!");

    let total: i32 = [10, 20, 30].iter().sum();
    println!("total = {total}");
}
```

If you nest loops, `break` and `continue` apply to the innermost one. Give an outer loop a label like `'outer: loop { ... }` and use `break 'outer;` to leave it from inside.

## When to use which

- `for` when you're going over a range or collection. This is most loops.
- `while` when you stop on a condition rather than a count.
- `loop` when you stop from the inside, especially if the loop produces a value.

The checkpoint asks you to combine a `for` loop with an `if`. After that comes the chapter challenge.
