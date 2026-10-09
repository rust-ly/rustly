+++
id = "book.oop.states-as-types"
chapter = "book.oop"
requires = []
title = "Encoding States as Types"
track = "book"
order = 5
source = "https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html#encoding-states-and-behavior-as-types"
summary = "Make each state its own struct, so using a post in the wrong state is a compile error instead of a silent no-op."
+++

# Encoding States and Behaviour as Types

The state pattern works, but its rules are only enforced **at run time**: calling `content()` on a draft quietly returns `""`, and calling `approve()` on a draft quietly does nothing. If you forget a step, nothing tells you.

Rust's type system can do better. Instead of one `Post` type with hidden states, make **each state its own type**, and give each type only the methods that make sense for it:

```rust,editable title="States as separate types"
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost { content: self.content }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post { content: self.content }
    }
}

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    println!("{}", post.content());
}
```

## What changed

- `Post::new()` returns a `DraftPost`. A **draft has no `content` method at all**, so trying to read an unpublished post is a compile error, not an empty string.
- `request_review` and `approve` take **`self` by value** and return a **new type**. The old value is consumed, so you can't keep using the draft after submitting it.
- `main` uses **shadowing** (`let post = ...`) to keep the same name as the type changes.

```rust,does_not_compile title="Reading a draft is now a compile error"
pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

fn main() {
    let mut post = DraftPost { content: String::new() };
    post.add_text("secret draft");
    println!("{}", post.content());
}
```

## Which to choose?

The object-oriented version keeps one type and lets the workflow change without touching callers, which is good when states are loaded at run time or change often. The types version catches mistakes **at compile time** and needs no `Option` or `Box`, but callers have to handle the changing types. Rust programmers often prefer it: if a wrong sequence of calls can't compile, it can't ship.
