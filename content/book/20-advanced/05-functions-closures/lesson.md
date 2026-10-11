+++
id = "book.advanced.functions-closures"
chapter = "book.advanced"
requires = []
title = "Function Pointers and Returning Closures"
track = "book"
order = 5
source = "https://doc.rust-lang.org/book/ch20-04-advanced-functions-and-closures.html"
summary = "Pass named functions with the fn type, use them where closures are expected, and return closures with impl Fn or Box<dyn Fn>."
+++

# Advanced Functions and Closures

## Function pointers: the `fn` type

You've passed closures to functions. You can pass **named functions** too. The type `fn(i32) -> i32` (lowercase `fn`) is a **function pointer**:

```rust,runnable title="Passing a function to a function"
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");
}
```

`fn` is a type, not a trait, so you write it directly as the parameter type with no generics. A closure that captures nothing from its surroundings also coerces to a `fn` pointer, so `do_twice(|x| x * 10, 5)` works too.

## Functions where closures are expected

Function pointers implement all three closure traits: `Fn`, `FnMut` and `FnOnce`. So anywhere a closure is accepted, a named function works too. That includes `map`:

```rust,editable title="A closure or a named function"
fn main() {
    let numbers = vec![1, 2, 3];

    let with_closure: Vec<String> = numbers.iter().map(|i| i.to_string()).collect();
    let with_function: Vec<String> = numbers.iter().map(ToString::to_string).collect();

    println!("{with_closure:?} {with_function:?}");
}
```

Tuple struct and enum variant constructors are functions as well. With `enum Status { Value(u32) }`, `Status::Value` has type `fn(u32) -> Status`, so `(0..3).map(Status::Value).collect::<Vec<_>>()` builds a list of statuses.

## Returning closures

Every closure has its own anonymous type, so you can't write it out as a return type. When a function returns one closure, use `impl Fn`: `fn make_adder(n: i32) -> impl Fn(i32) -> i32 { move |x| x + n }`.

`impl Trait` stands for **one** concrete type. Two different closures are two different types, even if their code is identical, so they can't go in the same `Vec` as `impl Fn` values:

```rust,does_not_compile title="Two closures, two types"
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
    move |x| x + init
}

fn main() {
    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        println!("{}", handler(5));
    }
}
```

The fix is a trait object, `Box<dyn Fn(i32) -> i32>`, which works for any closure with that signature (chapter 18):

```rust,runnable title="Boxing closures of different types"
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + init)
}

fn main() {
    let handlers = vec![returns_closure(), returns_initialized_closure(123)];
    for handler in handlers {
        println!("{}", handler(5));
    }
}
```

Use `impl Fn` when you return a single closure, and `Box<dyn Fn>` when the caller needs to store or mix different ones.
