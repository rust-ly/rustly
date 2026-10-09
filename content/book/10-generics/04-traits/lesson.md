+++
id = "book.generics.traits"
chapter = "book.generics"
requires = []
title = "Defining and Implementing Traits"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch10-02-traits.html"
summary = "A trait names a set of methods that different types can share, each with its own implementation."
+++

# Traits: Defining Shared Behaviour

A **trait** describes something a type can **do**, as a list of method signatures. Different types can implement the same trait in their own way. (If you've used other languages, traits are like *interfaces*.)

Imagine a news app that shows articles and social media posts. They're different structs, but both can be **summarised** for a preview. That shared ability is a trait:

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

The trait lists the method's signature and ends with `;` instead of a body. Every type that implements `Summary` must provide that method.

## Implementing a trait

Use `impl TraitName for TypeName`, then write the method bodies:

```rust,runnable title="Two types, one trait"
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Post {
    pub username: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup"),
        location: String::from("Pittsburgh"),
        author: String::from("Iceburgh"),
    };
    let post = Post {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };
    println!("{}", article.summarize());
    println!("{}", post.summarize());
}
```

You've been using traits all along. `#[derive(Debug)]` implements the `Debug` trait for you; `PartialEq`, `Clone` and `Copy` are traits too.

## Implementing for types you didn't write

You can implement your own trait on a standard type like `i32` or `String`:

```rust,editable title="A trait on i32"
trait Describe {
    fn describe(&self) -> String;
}

impl Describe for i32 {
    fn describe(&self) -> String {
        if *self < 0 { String::from("negative") } else { String::from("not negative") }
    }
}

fn main() {
    println!("-3 is {}", (-3).describe());
}
```

There's one limit, the **orphan rule**: either the trait or the type must be defined in your crate. You can't implement the standard `Display` trait on the standard `Vec<T>`, because neither is yours. This stops two crates from both implementing the same trait for the same type and conflicting.

## A common mistake

A type that claims a trait must implement **every** method in it:

```rust,does_not_compile title="A missing method"
trait Summary {
    fn summarize(&self) -> String;
    fn author(&self) -> String;
}

struct Post;

impl Summary for Post {
    fn summarize(&self) -> String {
        String::from("a post")
    }
}

fn main() {}
```
