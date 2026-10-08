+++
id = "book.ownership.borrowing"
chapter = "book.ownership"
requires = []
title = "Borrowing"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html"
summary = "Use a value through a reference without taking ownership, and the rules that keep references safe."
+++

# References and Borrowing

A **reference** lets you use a value without owning it. You make one with `&`, and creating a reference is called **borrowing**. When the reference goes out of scope, nothing is freed, because the reference never owned the value.

```rust,runnable title="Borrowing a String"
fn length(s: &String) -> usize {
    s.len()
}

fn main() {
    let name = String::from("Ferris");
    let n = length(&name);
    println!("{name} has {n} letters"); // `name` is still ours
}
```

Compare this with the previous concept. `length` takes `&String`, so `name` doesn't move and `main` can still use it.

## Mutable references

References are immutable by default, just like variables. To let a function change a borrowed value, the variable has to be `mut` and you pass `&mut`:

```rust,editable title="Changing a value through &mut"
fn add_excitement(s: &mut String) {
    s.push_str("!");
}

fn main() {
    let mut greeting = String::from("hello");
    add_excitement(&mut greeting);
    add_excitement(&mut greeting);
    println!("{greeting}");
}
```

## The borrowing rules

At any given time you can have **either**:

- any number of immutable references (`&T`), **or**
- exactly one mutable reference (`&mut T`).

References must also always be valid. You can't keep a reference to something that has already been dropped.

These rules prevent **data races**, where one part of the code reads a value while another part changes it. In Rust that's a compile error, not a 2 a.m. debugging session:

```rust,does_not_compile title="Two mutable borrows at once"
fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{r1}, {r2}");
}
```

A borrow lasts only until the reference's **last use**, not to the end of the block. So after you've finished with `r1`, you can take a new `&mut s`.

## Prefer `&str` for text parameters

A function that only reads text should usually take `&str` instead of `&String`. A `&String` coerces to `&str` automatically, and string literals are already `&str`, so the function accepts both. You'll see why `&str` works this way in the next concept, on slices.
