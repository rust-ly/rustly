+++
id = "book.oop.state-pattern"
chapter = "book.oop"
requires = []
title = "The State Pattern"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html"
summary = "A blog post moves from draft to review to published; each state is a trait object that decides what happens next."
+++

# Implementing an Object-Oriented Design Pattern

The **state pattern** gives a value an internal **state**, represented by a set of state objects, and its behaviour changes depending on which state it's in. Each state knows its own rules and when to switch to another state. The value itself doesn't need to know them.

The Book's example is a blog post workflow:

1. A post starts as an empty **draft**.
2. When the draft is done, the author **requests a review**.
3. When the post is **approved**, it gets **published**.
4. Only published posts show their content, so a draft can't be published by accident.

## `Post` holds a `Box<dyn State>`

```rust,editable title="The blog post state machine"
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post { state: Some(Box::new(Draft {})), content: String::new() }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
struct PendingReview {}
struct Published {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> { Box::new(PendingReview {}) }
    fn approve(self: Box<Self>) -> Box<dyn State> { self }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> { Box::new(Published {}) }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> { self }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    println!("draft shows: {:?}", post.content());
    post.request_review();
    println!("in review shows: {:?}", post.content());
    post.approve();
    println!("published shows: {:?}", post.content());
}
```

## The interesting parts

- **`self: Box<Self>`** means the method takes ownership of the **boxed** state. The old state is used up and returns the new one, so the post can't keep a stale state around.
- **`Option` and `take()`**: Rust doesn't allow a struct field to be empty even briefly, so `state` is an `Option`. `take()` moves the state out and leaves `None` behind for a moment, until the new state is put back.
- **`content` has a default** returning `""`, and only `Published` overrides it. Adding a rule means changing one state, not a big `match` scattered through `Post`.
- **Unchanged states return `self`**: approving a draft does nothing.

The Book also points out the downsides: the states are coupled (each knows which state comes next), and some logic is repeated across states. The next concept shows a more Rust-like alternative.

```rust,does_not_compile title="Calling a by-value method on a borrowed state"
trait State {
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Post {
    state: Box<dyn State>,
}

impl Post {
    fn approve(&mut self) {
        self.state = self.state.approve();
    }
}

fn main() {}
```

That last snippet is why the real `Post` wraps its state in an `Option`: you can't move a value out of a field behind `&mut self`, but you can `take()` it out of an `Option`.
