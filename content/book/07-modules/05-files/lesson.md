+++
id = "book.modules.files"
chapter = "book.modules"
requires = []
title = "Modules in Files and pub use"
track = "book"
order = 5
source = "https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html"
summary = "How modules map to files in a real project, and re-exporting names with pub use."
+++

# Separating Modules into Files

The Playground runs a single file, so this course writes modules inline with `mod name { ... }`. In a real project, each module usually lives in its own file. The module tree stays exactly the same; only where the code is stored changes.

Replace a module's body with a semicolon, and Rust looks for the body in a file named after it:

```rust
// src/lib.rs
mod front_of_house;          // the body is in src/front_of_house.rs

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

A child module's file goes in a folder named after its parent:

```text
src/
├── lib.rs                  crate root: `mod front_of_house;`
├── front_of_house.rs       `pub mod hosting;`
└── front_of_house/
    └── hosting.rs          `pub fn add_to_waitlist() {}`
```

You only write `mod front_of_house;` **once**, in the parent. Everywhere else refers to the module by its path. `mod` isn't an "include": it declares where the module sits in the tree.

## Re-exporting with `pub use`

Normally a `use` is private to the scope it's in. `pub use` also makes the name available to code **outside**, as if the item were defined right there. This is called **re-exporting**.

It lets a library keep its internal layout while offering users a simpler one. Users of the restaurant shouldn't need to know about `front_of_house`:

```rust,runnable title="Re-exporting a module"
mod restaurant {
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("added");
            }
        }
    }

    pub use self::front_of_house::hosting;
}

fn main() {
    // `front_of_house` is private, but the re-export is public.
    restaurant::hosting::add_to_waitlist();
}
```

The standard library does this all the time. `std::collections::HashMap` is really defined in a deeper internal module, re-exported to a friendlier path.

## A common mistake

Without `pub`, a `use` inside a module is only a private shortcut for that module. Outside code can't go through it:

```rust,does_not_compile title="A private use isn't a re-export"
mod restaurant {
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    use self::front_of_house::hosting;
}

fn main() {
    restaurant::hosting::add_to_waitlist();
}
```
