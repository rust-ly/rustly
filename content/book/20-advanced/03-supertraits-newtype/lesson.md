+++
id = "book.advanced.supertraits-newtype"
chapter = "book.advanced"
requires = []
title = "Supertraits, Fully Qualified Syntax and Newtypes"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#using-supertraits"
summary = "Require one trait to build on another, call the right method when names clash, and wrap a type to implement outside traits."
+++

# Supertraits, Fully Qualified Syntax and Newtypes

## Supertraits: one trait needs another

Sometimes a trait only makes sense for types that also implement **another** trait. The Book's `OutlinePrint` draws a box of asterisks around a value, so it needs the value to be printable with `Display`. Writing `trait OutlinePrint: fmt::Display` makes `Display` a **supertrait**: inside `OutlinePrint`, you can use `to_string()`, and only `Display` types can implement it:

```rust,runnable title="A trait that requires Display"
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn main() {
    Point { x: 1, y: 3 }.outline_print();
}
```

## Fully qualified syntax: when two methods share a name

A type can implement two traits that each have a `fly` method, plus its own `fly`. Calling `person.fly()` picks the type's own method; to call a trait's version, name the trait:

```rust,editable title="Choosing which fly to call"
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) { println!("This is your captain speaking."); }
}
impl Wizard for Human {
    fn fly(&self) { println!("Up!"); }
}
impl Human {
    fn fly(&self) { println!("*waving arms furiously*"); }
}

fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

For associated functions without `self`, the full form is `<Type as Trait>::function()`, for example `<Dog as Animal>::baby_name()`.

## The newtype pattern

The orphan rule (chapter 10) stops you implementing `Display` on `Vec<String>`, because neither is yours. The **newtype pattern** gets around it: wrap the type in a tuple struct, which **is** yours, and implement the trait on the wrapper:

```rust,runnable title="Display for a wrapped Vec"
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
```

There's no run-time cost: the wrapper disappears at compile time. The downside is that `Wrapper` doesn't have `Vec`'s methods unless you add them, or implement `Deref` (chapter 15) to forward to the inner value.
