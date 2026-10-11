+++
id = "book.advanced.advanced-types"
chapter = "book.advanced"
requires = []
title = "Advanced Types"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch20-03-advanced-types.html"
summary = "Newtypes for safety, type aliases for brevity, the never type !, and dynamically sized types."
+++

# Advanced Types

## Newtypes for type safety

The newtype pattern from the last concept has a second use: making the compiler tell apart values that share a representation. Metres and millimetres are both `u32`, so passing one where the other was meant would compile, and be a bug. As separate types, it can't happen:

```rust,does_not_compile title="Meters where Millimeters are expected"
struct Meters(u32);
struct Millimeters(u32);

fn print_mm(mm: Millimeters) {
    println!("{} mm", mm.0);
}

fn main() {
    print_mm(Meters(3));
}
```

## Type aliases: a shorter name

`type Kilometers = i32;` creates an **alias**, another name for the same type. Unlike a newtype, there's no safety benefit, since `Kilometers` and `i32` are interchangeable. Aliases are for cutting down long, repeated types:

```rust,runnable title="Aliasing a long type"
type Thunk = Box<dyn Fn() + Send + 'static>;

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}

fn takes_long_type(f: Thunk) {
    f();
}

fn main() {
    let f: Thunk = Box::new(|| println!("hello"));
    takes_long_type(f);
    returns_long_type()();
}
```

The standard library does this with `std::io::Result<T>`, an alias for `Result<T, std::io::Error>`.

## The never type `!`

`!` is the type of an expression that **never produces a value**: `panic!`, `continue`, `break`, `loop` without a `break`, and `std::process::exit`. Because it never returns, `!` can stand in for **any** type. That's why this `match` works even though one arm is a number and the other is `continue`:

```rust,editable title="continue has type !"
fn main() {
    let inputs = ["3", "x", "7"];
    let mut total = 0;
    for input in inputs {
        let n: u32 = match input.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        total += n;
    }
    println!("total: {total}");
}
```

It's also why `let...else` requires the `else` block to diverge (chapter 6): the block's type must be `!`.

## Dynamically sized types and `Sized`

Some types have a size known only at run time: `str` (a string of any length) and `dyn Trait`. These **dynamically sized types** can only be used behind a pointer that also stores the size or vtable: `&str`, `Box<str>`, `&dyn Trait`, `Box<dyn Trait>`. That's why you always see `&str`, never a bare `str`.

Generic functions assume `T: Sized` by default. To accept dynamically sized types too, relax the bound with `?Sized`, and take the value behind a reference: `fn generic<T: ?Sized>(t: &T)`.
