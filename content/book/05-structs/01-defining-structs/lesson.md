+++
id = "book.structs.defining"
chapter = "book.structs"
requires = []
title = "Defining Structs"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch05-01-defining-structs.html"
summary = "Define a struct, build instances, and use field-init shorthand and update syntax."
+++

# Defining and Instantiating Structs

A **struct** groups related values into one named type. Unlike a tuple, every piece has a name, so you write `user.email` rather than `user.1`.

You list each field with its type, then create an **instance** by giving a value for every field. The fields can be in any order:

```rust,runnable title="A User struct"
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user = User {
        email: String::from("ferris@example.com"),
        username: String::from("ferris"),
        active: true,
        sign_in_count: 1,
    };
    user.sign_in_count += 1;
    println!("{} ({}) has signed in {} times, active: {}",
        user.username, user.email, user.sign_in_count, user.active);
}
```

Mutability belongs to the whole instance. You can't mark only some fields `mut`, so to change `sign_in_count` the binding has to be `let mut user`.

## Field-init shorthand

Functions that build structs often have parameters with the same names as the fields. In that case you can write the name once: `User { email, username, ... }` means `User { email: email, username: username, ... }`.

## Struct update syntax

To make a new instance that's mostly the same as an existing one, list the fields that change and finish with `..other` to take the rest from it:

```rust,editable title="Struct update syntax"
# struct User {
#     active: bool,
#     username: String,
#     email: String,
#     sign_in_count: u64,
# }
#
fn main() {
    let user1 = User {
        active: true,
        username: String::from("ferris"),
        email: String::from("ferris@example.com"),
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("crab@example.com"),
        ..user1
    };
    println!("{} <{}>", user2.username, user2.email);
}
```

This follows the ownership rules from the last chapter. `..user1` **moves** the `username` `String` into `user2`, so after this line you can't use `user1.username`, though `user1.email` is still fine. The `bool` and `u64` fields are `Copy`, so they're simply copied.

## Tuple structs and unit structs

A **tuple struct** has a name but no field names. That's useful when a bare tuple would be ambiguous. `Color(0, 0, 0)` and `Point(0, 0, 0)` are different types even though both hold three `i32`s, so you can't mix them up:

```rust,runnable title="Tuple structs"
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("red = {}, x = {}", black.0, origin.0);
}
```

A **unit-like struct** has no fields at all, as in `struct AlwaysEqual;`. It becomes useful once you implement traits on a type, later in the course.

Why do the examples use `String` instead of `&str` for the fields? A struct that owns its data is simplest: the data lives as long as the struct does. Storing references in a struct needs **lifetimes**, which come in chapter 10.
