+++
id = "book.generics.lifetimes-in-structs"
chapter = "book.generics"
requires = []
title = "Lifetimes in Structs, Elision and 'static"
track = "book"
order = 8
source = "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions"
summary = "Structs that hold references, the rules that let you skip annotations, and the 'static lifetime."
+++

# Lifetimes in Structs, Elision and `'static`

## Structs that hold references

So far every struct has **owned** its data (`String`, not `&str`). A struct can hold a reference, but then it must not outlive the data it borrows, so the struct needs a lifetime parameter:

```rust,runnable title="A struct that borrows text"
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt { part: first_sentence };
    println!("{excerpt:?}");
}
```

`ImportantExcerpt<'a>` means "an excerpt can't outlive the text its `part` borrows from". Here `novel` outlives `excerpt`, so all is well.

## Lifetime elision: why you rarely write them

`first_word(s: &str) -> &str` from chapter 4 had no annotations, yet compiled. Early Rust required them everywhere, but people kept writing the same obvious ones, so the compiler learned three **elision rules** that fill them in:

1. Each reference parameter gets its own lifetime.
2. If there's **exactly one** input lifetime, the output gets that lifetime.
3. If one of the inputs is `&self` or `&mut self`, the output gets `self`'s lifetime.

If the rules leave an output lifetime undecided, as with `longest(x, y)`, you must write it. Rule 3 is why methods almost never need annotations:

```rust,editable title="Methods on a struct with a lifetime"
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    let text = String::from("Brevity is wit.");
    let e = ImportantExcerpt { part: &text };
    println!("level {}", e.level());
    println!("{}", e.announce_and_return_part("hear this"));
}
```

`impl<'a>` declares the lifetime, just like `impl<T>` for generic types.

## The `'static` lifetime

`'static` means "valid for the whole run of the program". Every string literal has it, because literals are stored in the program itself: `let s: &'static str = "I live forever";`.

When an error suggests adding `'static`, be careful. It's usually a sign the code is trying to keep a reference to something that doesn't live long enough, and the real fix is to restructure or to use owned data.

## A common mistake

A struct that holds a reference without declaring a lifetime doesn't compile:

```rust,does_not_compile title="A reference field with no lifetime"
struct Excerpt {
    part: &str,
}

fn main() {}
```
