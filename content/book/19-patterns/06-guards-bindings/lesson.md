+++
id = "book.patterns.guards-bindings"
chapter = "book.patterns"
requires = []
title = "Match Guards and @ Bindings"
track = "book"
order = 6
source = "https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#extra-conditionals-with-match-guards"
summary = "Add an if condition to a match arm, and bind a value while also testing it against a pattern."
+++

# Extra Conditionals with Match Guards

A **match guard** is an `if` condition after an arm's pattern. The arm only runs if the pattern matches **and** the condition is true:

```rust,runnable title="A match guard"
fn main() {
    for num in [Some(4), Some(5), None] {
        match num {
            Some(x) if x % 2 == 0 => println!("The number {x} is even"),
            Some(x) => println!("The number {x} is odd"),
            None => (),
        }
    }
}
```

Guards can express things patterns alone can't, like "is even".

## Fixing the shadowing problem

Remember the earlier surprise where `Some(y)` made a **new** `y` instead of comparing with the outer one? A guard fixes it: bind a new name in the pattern, then compare it with the outer variable in the guard:

```rust,editable title="Comparing with an outer variable"
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }
}
```

With `|`, the guard applies to **all** the alternatives: `4 | 5 | 6 if y` means `(4 | 5 | 6) if y`, not `4 | 5 | (6 if y)`.

One thing to know: the compiler doesn't look inside guard conditions when checking exhaustiveness, so a `match` made only of guarded arms still needs a catch-all.

## `@` bindings

The `@` operator binds a value to a name **while** testing it against a pattern. Here we want to check that an id is in a range, and also use the actual number:

```rust,runnable title="Binding with @"
enum Message {
    Hello { id: i32 },
}

fn main() {
    for id in [5, 11, 50] {
        match (Message::Hello { id }) {
            Message::Hello { id: id_variable @ 3..=7 } => {
                println!("Found an id in range: {id_variable}")
            }
            Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
            Message::Hello { id } => println!("Found some other id: {id}"),
        }
    }
}
```

In the first arm, `id_variable @ 3..=7` tests the range **and** keeps the value. The second arm tests a range but can't print the number, because it didn't bind it. The last arm binds without testing. `@` gives you both at once.
