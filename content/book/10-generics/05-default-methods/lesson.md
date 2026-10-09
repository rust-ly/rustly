+++
id = "book.generics.default-methods"
chapter = "book.generics"
requires = []
title = "Default Implementations"
track = "book"
order = 5
source = "https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations"
summary = "Give a trait method a body that types get for free, and can override if they want."
+++

# Default Implementations

A trait method can have a **default body**. Types that implement the trait get that behaviour automatically, and can still write their own version to **override** it.

```rust,runnable title="A default summary"
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
}

// No methods written: the default is used.
impl Summary for NewsArticle {}

fn main() {
    let article = NewsArticle { headline: String::from("Rust 2.0 released") };
    println!("New article available! {}", article.summarize());
}
```

`impl Summary for NewsArticle {}` with an empty body is enough, because every method already has a default.

## Defaults that call required methods

The real power comes from mixing the two. A default method can call **other** methods of the same trait, even ones that have no default. Each type then only writes the small required part and gets the rest for free:

```rust,editable title="Write one method, get two"
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Post {
    pub username: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let post = Post {
        username: String::from("ferris"),
        content: String::from("I love Rust"),
    };
    println!("1 new post: {}", post.summarize());
}
```

`Post` only wrote `summarize_author`, but `post.summarize()` works and uses it. Many standard library traits are designed like this: `Iterator` has one required method, `next`, and dozens of default methods built on top of it, as you'll see in chapter 13.

## Overriding a default

Implement the method yourself and your version replaces the default for that type only:

```rust,runnable title="One type overrides, one doesn't"
trait Greet {
    fn greet(&self) -> String {
        String::from("Hello!")
    }
}

struct English;
struct French;

impl Greet for English {}

impl Greet for French {
    fn greet(&self) -> String {
        String::from("Bonjour !")
    }
}

fn main() {
    println!("{} {}", English.greet(), French.greet());
}
```

You can't call the default from inside your override. If you need both behaviours, put the shared part in a separate method.
