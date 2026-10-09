+++
id = "book.generics.trait-bounds"
chapter = "book.generics"
requires = []
title = "Traits as Parameters and Bounds"
track = "book"
order = 6
source = "https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters"
summary = "Accept any type that implements a trait, with impl Trait, T: Trait, + and where."
+++

# Traits as Parameters

Traits become really useful when a function accepts **any type that implements a trait**. Write `impl TraitName` as the parameter's type:

```rust,runnable title="Notify about anything summarisable"
trait Summary {
    fn summarize(&self) -> String;
}

struct Post {
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        self.content.clone()
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    notify(&Post { content: String::from("Rust is fun") });
}
```

Inside `notify`, you can call any `Summary` method on `item`, and nothing else. Passing a type that doesn't implement `Summary` doesn't compile.

## Trait bound syntax

`impl Summary` is shorthand. The longer form uses a generic type parameter with a **trait bound**, `T: Summary`, meaning "any `T` that implements `Summary`": `fn notify<T: Summary>(item: &T)`.

The two are equivalent here. The long form matters when two parameters must be the **same** type: `fn notify<T: Summary>(a: &T, b: &T)` forces it, while `(a: &impl Summary, b: &impl Summary)` allows two different types.

## Several bounds: `+` and `where`

Require more than one trait with `+`. This is how the `largest` function from earlier works for any type that can be compared **and** copied out of the slice:

```rust,editable title="Two bounds on one type"
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("{} {}", largest(&[3, 9, 2]), largest(&['q', 'z', 'a']));
}
```

When bounds get long, move them to a `where` clause after the signature. It means exactly the same thing, but reads more easily:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

## Returning `impl Trait`

`impl Trait` also works as a **return type**: "this returns some type that implements the trait, and you don't need to know which". You'll see why that's handy with closures and iterators in chapter 13. One limit: the function must always return the **same** concrete type.

```rust,runnable title="Returning impl Display"
use std::fmt::Display;

fn make_greeting(name: &str) -> impl Display {
    format!("Hello, {name}!")
}

fn main() {
    println!("{}", make_greeting("Ferris"));
}
```
