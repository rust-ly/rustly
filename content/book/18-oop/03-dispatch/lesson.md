+++
id = "book.oop.dispatch"
chapter = "book.oop"
requires = []
title = "Generics or Trait Objects?"
track = "book"
order = 3
source = "https://doc.rust-lang.org/book/ch18-02-trait-objects.html#trait-objects-perform-dynamic-dispatch"
summary = "Static dispatch with generics vs dynamic dispatch with dyn, when to choose each, and which traits can be used as dyn."
+++

# Static and Dynamic Dispatch

You now have two ways to write code that works with "anything that implements `Draw`":

```rust
fn draw_all_generic<T: Draw>(items: &[T]) { /* ... */ }    // generics
fn draw_all_dyn(items: &[Box<dyn Draw>]) { /* ... */ }      // trait objects
```

They look similar but behave differently.

## Static dispatch: generics

With generics, the compiler writes a **separate copy** of the function for each concrete type it's used with (monomorphization, chapter 10). Every method call is resolved at compile time, so the calls are as fast as possible and can be inlined.

The catch: one call works with **one** type. A `Vec<T>` holds only `Button`s, or only `SelectBox`es, never a mix.

## Dynamic dispatch: `dyn`

With a trait object, the compiler doesn't know the concrete type, so each call looks up the right method **at run time**, through the table of methods (the *vtable*) stored next to the pointer. That costs a tiny bit of speed and prevents some optimisations, but it allows **mixing types** in one collection, and adding new types without recompiling the code that uses them.

```rust,runnable title="One function each way"
trait Shape {
    fn area(&self) -> f64;
}

struct Square(f64);
struct Circle(f64);

impl Shape for Square {
    fn area(&self) -> f64 { self.0 * self.0 }
}
impl Shape for Circle {
    fn area(&self) -> f64 { 3.14159 * self.0 * self.0 }
}

// Static: all items must be the same type.
fn total_generic<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// Dynamic: any mix of types.
fn total_dyn(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    println!("{}", total_generic(&[Square(1.0), Square(2.0)]));
    let mixed: Vec<Box<dyn Shape>> = vec![Box::new(Square(1.0)), Box::new(Circle(1.0))];
    println!("{:.3}", total_dyn(&mixed));
}
```

**Rule of thumb:** use generics by default; use `dyn` when you need a collection of different types, or when the set of types isn't known until run time (plugins, user-chosen options, or a function that returns one of several types).

```rust,editable title="Returning one of several types"
trait Greeter {
    fn greet(&self) -> String;
}

struct English;
struct Swahili;

impl Greeter for English {
    fn greet(&self) -> String { String::from("Hello") }
}
impl Greeter for Swahili {
    fn greet(&self) -> String { String::from("Habari") }
}

fn greeter_for(lang: &str) -> Box<dyn Greeter> {
    match lang {
        "sw" => Box::new(Swahili),
        _ => Box::new(English),
    }
}

fn main() {
    println!("{}", greeter_for("sw").greet());
    println!("{}", greeter_for("en").greet());
}
```

`impl Greeter` wouldn't work as the return type here, because the function returns **different** types depending on the input. `Box<dyn Greeter>` allows that.

## Dyn compatibility

Not every trait can be used as `dyn Trait`. Roughly, the compiler must be able to call every method without knowing the concrete type, so a **dyn-compatible** trait's methods can't be generic and can't return `Self`. `Clone` isn't dyn-compatible (`clone` returns `Self`), so `Box<dyn Clone>` doesn't compile:

```rust,does_not_compile title="A trait that can't be a trait object"
fn main() {
    let items: Vec<Box<dyn Clone>> = Vec::new();
}
```
