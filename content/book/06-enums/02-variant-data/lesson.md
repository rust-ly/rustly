+++
id = "book.enums.variant-data"
chapter = "book.enums"
requires = []
title = "Variants That Carry Data"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#enum-values"
summary = "Attach different data to each variant, in tuple or struct form."
+++

# Variants That Carry Data

So far each variant was just a name. Variants can also **carry data**, and each variant can carry different data.

Take IP addresses. A version 4 address is four numbers like `127.0.0.1`; a version 6 address is easiest to store as text like `::1`. Without data in variants, you'd need an enum for the kind **and** a struct to pair the kind with the address. With data in the variants, one enum does it all, and each variant gets the shape it needs:

```rust,runnable title="Each variant holds its own data"
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{home:?}");
    println!("{loopback:?}");
}
```

Notice that `IpAddr::V4(...)` looks like a function call. It is one: every variant with data is also a **constructor function** that builds a value of the enum.

## Three shapes of variant

A variant can look like any of the three kinds of struct you've seen:

```rust,editable title="Unit, tuple and struct variants"
#[derive(Debug)]
enum Message {
    Quit,                       // no data, like a unit struct
    Move { x: i32, y: i32 },    // named fields, like a struct
    Write(String),              // unnamed fields, like a tuple struct
    ChangeColor(u8, u8, u8),
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: -3 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];
    for m in messages {
        println!("{m:?}");
    }
}
```

Defining four separate structs would also work, but then a function couldn't take "any message" as one type. With an enum, `fn process(msg: Message)` accepts all of them.

## Methods on enums

Just like structs, enums can have `impl` blocks. The method below doesn't look inside the variant yet (that needs `match`, coming soon), but it can still be called on any variant:

```rust,runnable title="An impl block on an enum"
#[derive(Debug)]
enum Message {
    Quit,
    Write(String),
}

impl Message {
    fn log(&self) {
        println!("[log] {self:?}");
    }
}

fn main() {
    Message::Write(String::from("saved")).log();
    Message::Quit.log();
}
```

## A common mistake

A struct-style variant needs its fields by name, in braces. Writing it like a tuple variant won't compile:

```rust,does_not_compile title="Struct variant written like a tuple"
enum Message {
    Move { x: i32, y: i32 },
}

fn main() {
    let m = Message::Move(1, 2);
}
```
