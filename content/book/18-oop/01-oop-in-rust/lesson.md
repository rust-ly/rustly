+++
id = "book.oop.oop-in-rust"
chapter = "book.oop"
requires = []
title = "Objects, Encapsulation and Inheritance"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch18-01-what-is-oo.html"
summary = "Rust has objects and encapsulation, but no inheritance; traits and generics cover what inheritance is used for."
+++

# Characteristics of Object-Oriented Languages

There's no single definition of object-oriented programming (OOP), but most definitions include three ideas. Here's how each maps to Rust.

## 1. Objects: data plus behaviour

The famous "Gang of Four" book defines an object as something that packages **data** with the **procedures** that operate on it. By that definition, Rust has objects: structs and enums hold data, and `impl` blocks give them methods. Rust just doesn't call them objects.

## 2. Encapsulation: hiding the details

Encapsulation means code outside an object can only use its **public interface**, not its internals, so the internals can change without breaking anyone. Rust does this with `pub` (chapter 7). The Book's example is a list that always knows its average:

```rust,runnable title="An encapsulated collection"
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection { list: vec![], average: 0.0 }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut c = AveragedCollection::new();
    c.add(3);
    c.add(5);
    println!("average: {}", c.average());
}
```

The fields are private, so outside code **can't** push to `list` without updating `average`. The two can never get out of sync. And because nobody depends on the fields, `list` could become a `HashSet` tomorrow without changing any caller.

## 3. Inheritance: Rust doesn't have it

**Inheritance** lets one type reuse another's data and methods by being declared its child. Rust has no way to define a struct that inherits another's fields. People use inheritance for two reasons, and Rust has a tool for each:

- **Reusing code.** Trait **default methods** (chapter 10) let many types share one implementation, and any type can override it.
- **Polymorphism**, using different types through one interface. Rust uses **generics** with trait bounds, and **trait objects** (next concept).

Many language designers have moved away from inheritance anyway. It tends to share more code than intended, and makes programs rigid. Rust prefers composing small traits.

```rust,editable title="Sharing behaviour without inheritance"
trait Describe {
    fn name(&self) -> String;

    fn describe(&self) -> String {
        format!("This is {}.", self.name())
    }
}

struct Dog;
struct Robot;

impl Describe for Dog {
    fn name(&self) -> String {
        String::from("a dog")
    }
}

impl Describe for Robot {
    fn name(&self) -> String {
        String::from("a robot")
    }

    fn describe(&self) -> String {
        format!("BEEP. I AM {}.", self.name().to_uppercase())
    }
}

fn main() {
    println!("{}", Dog.describe());
    println!("{}", Robot.describe());
}
```
