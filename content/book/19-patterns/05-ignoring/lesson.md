+++
id = "book.patterns.ignoring"
chapter = "book.patterns"
requires = []
title = "Ignoring Values: _, _name and .."
track = "book"
order = 5
source = "https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#ignoring-values-in-a-pattern"
summary = "Skip the parts of a value you don't need with _, unused-variable names, and .. for the rest."
+++

# Ignoring Values in a Pattern

Often you only care about part of a value. There are four ways to ignore the rest.

## `_` ignores one value

`_` matches anything and **doesn't bind it**. You've used it as the catch-all arm; it also works inside a pattern, or as a function parameter you don't need:

```rust,runnable title="_ inside patterns"
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

fn main() {
    foo(3, 4);

    let numbers = (2, 4, 8, 16, 32);
    let (first, _, third, _, fifth) = numbers;
    println!("Some numbers: {first}, {third}, {fifth}");
}
```

Nested `_` is handy for "both are set, I don't care what to":

```rust,editable title="Only overwrite an unset value"
fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't overwrite an existing customised value"),
        _ => setting_value = new_setting_value,
    }
    println!("setting is {setting_value:?}");
}
```

## `_name`: bind, but don't warn

Starting a variable's name with `_` tells Rust "I know this is unused", so there's no warning. But unlike plain `_`, it **does** bind, which means it can take ownership:

```rust,does_not_compile title="_s still takes ownership"
fn main() {
    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {
        println!("found a string");
    }
    println!("{s:?}");
}
```

Change `_s` to `_` and it compiles, because `_` never binds, so `s` is never moved.

## `..` ignores the rest

`..` skips every part you don't name: the remaining fields of a struct, or the middle of a tuple:

```rust,runnable title=".. for the rest"
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point3 { x: 0, y: 0, z: 0 };
    match origin {
        Point3 { x, .. } => println!("x is {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => println!("first {first}, last {last}"),
    }
}
```

`..` must be unambiguous: `(.., second, ..)` doesn't compile, because Rust can't tell which value you mean.
