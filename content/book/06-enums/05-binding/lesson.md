+++
id = "book.enums.binding"
chapter = "book.enums"
requires = []
title = "Patterns That Bind Values"
track = "book"
order = 5
source = "https://doc.rust-lang.org/book/ch06-02-match.html#patterns-that-bind-to-values"
summary = "Pull the data out of a variant by naming it in the pattern, including the value inside Some."
+++

# Patterns That Bind Values

When a variant carries data, a `match` arm can **name** that data and use it. This is how you get values back out of an enum.

Between 1999 and 2008, the US minted quarters with a different state on the back. Let's make `Quarter` carry its state:

```rust,runnable title="Getting the state off a quarter"
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
```

In the pattern `Coin::Quarter(state)`, `state` is a new variable. When a quarter matches, `state` is set to the `UsState` inside it, and the arm can use it.

## Struct-style variants

For variants with named fields, the pattern names the fields:

```rust,editable title="Binding named fields"
enum Message {
    Move { x: i32, y: i32 },
    Write(String),
}

fn describe(msg: &Message) -> String {
    match msg {
        Message::Move { x, y } => format!("move to ({x}, {y})"),
        Message::Write(text) => format!("write {text:?}"),
    }
}

fn main() {
    println!("{}", describe(&Message::Move { x: 3, y: -1 }));
    println!("{}", describe(&Message::Write(String::from("hi"))));
}
```

## Getting the value out of an Option

This is the answer to the question from the `Option` concept. To use the number inside an `Option<i32>`, match on it. The `Some(i)` arm binds `i` to the number, and the `None` arm handles the missing case:

```rust,runnable title="plus_one"
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{six:?} {none:?}");
}
```

Notice that inside the `Some(i)` arm, `i` is a plain `i32`, so `i + 1` just works. Outside the match you only had an `Option<i32>`. Pattern matching is the safe doorway from one to the other.

## A common mistake

The variable in a pattern only exists inside its own arm. Trying to use it after the `match` fails:

```rust,does_not_compile title="Using a pattern variable outside its arm"
fn main() {
    let x = Some(3);
    match x {
        Some(n) => println!("got {n}"),
        None => println!("nothing"),
    }
    println!("{n}");
}
```
