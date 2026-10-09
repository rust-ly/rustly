+++
id = "book.oop.trait-objects"
chapter = "book.oop"
requires = []
title = "Trait Objects: dyn Trait"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch18-02-trait-objects.html"
summary = "Box<dyn Trait> holds a value of any type that implements the trait, so one collection can hold many different types."
+++

# Using Trait Objects to Abstract over Shared Behaviour

A vector holds values of **one** type. In chapter 8 you used an enum to store a few different kinds of value. But an enum only works when you know every kind in advance. Imagine a GUI library: it has buttons and text fields, and **users** of the library should be able to add their own components, like an image or a select box, which the library has never heard of.

## Trait objects

A **trait object** points to a value of **some** type that implements a trait, without saying which type. You write it with the `dyn` keyword, behind a pointer such as `Box`: `Box<dyn Draw>` means "a box holding something that implements `Draw`".

```rust,runnable title="A screen of different components"
pub trait Draw {
    fn draw(&self) -> String;
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            println!("{}", component.draw());
        }
    }
}

pub struct Button {
    pub width: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) -> String {
        format!("[ {} ] ({}px)", self.label, self.width)
    }
}

struct SelectBox {
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) -> String {
        format!("<select: {}>", self.options.join(" | "))
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox { options: vec![String::from("Yes"), String::from("No")] }),
            Box::new(Button { width: 50, label: String::from("OK") }),
        ],
    };
    screen.run();
}
```

`Screen` doesn't know about `SelectBox`, which "a user of the library" wrote, yet it can draw one, because all `Screen` needs is that every component implements `Draw`. This is **duck typing** ("if it walks like a duck and quacks like a duck...") with the compiler's guarantee: putting something that **doesn't** implement `Draw` in the vector won't compile.

```rust,does_not_compile title="A component that can't draw"
pub trait Draw {
    fn draw(&self) -> String;
}

fn main() {
    let components: Vec<Box<dyn Draw>> = vec![Box::new(String::from("Hi"))];
}
```

## Why a pointer?

Different types have different sizes, so Rust can't store "some `Draw` type" directly in a vector slot. A `Box<dyn Draw>` is always the same size: a pointer to the value plus a pointer to a table of its trait methods. `&dyn Draw` works too, when you're only borrowing.

```rust,editable title="&dyn Trait as a parameter"
trait Speak {
    fn speak(&self) -> String;
}

struct Cat;
struct Duck;

impl Speak for Cat {
    fn speak(&self) -> String { String::from("meow") }
}
impl Speak for Duck {
    fn speak(&self) -> String { String::from("quack") }
}

fn chorus(animals: &[&dyn Speak]) -> String {
    animals.iter().map(|a| a.speak()).collect::<Vec<_>>().join(", ")
}

fn main() {
    println!("{}", chorus(&[&Cat, &Duck, &Cat]));
}
```
