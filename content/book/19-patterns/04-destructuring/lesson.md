+++
id = "book.patterns.destructuring"
chapter = "book.patterns"
requires = []
title = "Destructuring Structs, Enums and Tuples"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#destructuring-to-break-apart-values"
summary = "Break a value into its parts with a pattern that mirrors its shape, even several levels deep."
+++

# Destructuring to Break Apart Values

A pattern can mirror the shape of a struct, enum or tuple to pull out its parts.

## Structs

```rust,runnable title="Destructuring a struct"
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    println!("a = {a}, b = {b}");

    // Shorthand: variables named after the fields.
    let Point { x, y } = p;
    println!("x = {x}, y = {y}");
}
```

Parts of a pattern can be **literals**, which test values while destructuring the rest:

```rust,editable title="Literals inside a struct pattern"
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }
}
```

## Enums, and nesting

Each variant's pattern mirrors how its data is stored: nothing, a tuple, or named fields. Patterns can **nest** as deep as the data does:

```rust,runnable title="Nested enums"
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    ChangeColor(Color),
}

fn main() {
    let msgs = [
        Message::ChangeColor(Color::Hsv(0, 160, 255)),
        Message::Move { x: 3, y: -1 },
        Message::Quit,
    ];
    for msg in msgs {
        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => println!("rgb({r}, {g}, {b})"),
            Message::ChangeColor(Color::Hsv(h, s, v)) => println!("hsv({h}, {s}, {v})"),
            Message::Move { x, y } => println!("move to ({x}, {y})"),
            Message::Quit => println!("quit"),
        }
    }
}
```

## Tuples, and mixing everything

Combine struct and tuple patterns in one go:

```rust
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

That single `let` binds four variables from a tuple inside a tuple and a struct.
