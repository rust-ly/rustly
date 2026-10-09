+++
id = "book.modules.crates-and-modules"
chapter = "book.modules"
requires = []
title = "Crates and Modules"
track = "book"
order = 1
source = "https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html"
summary = "What packages and crates are, and how mod blocks group code into a tree."
+++

# Crates and Modules

Every example so far has fit in one short file. Real programs grow to thousands of lines, and they need structure: related code grouped together, implementation details hidden, and clear names for the parts other code should use. Rust gives you four tools for this, from biggest to smallest:

- A **package** is what `cargo new` creates: a folder with a `Cargo.toml` that builds one or more crates.
- A **crate** is one unit of compilation: either a **binary** (a program with a `main`) or a **library** (code for other crates to use, like the `rand` crate).
- **Modules** group code *inside* a crate and decide what's public.
- **Paths** are the names you use to reach an item, like `std::collections::HashMap`.

Every crate has a **crate root**, the file the compiler starts from: `src/main.rs` for a binary or `src/lib.rs` for a library. On the Playground, your whole program is the crate root.

## Defining a module

The `mod` keyword makes a module. Its body holds functions, structs, enums, constants and even other modules:

```rust,runnable title="A restaurant, split into modules"
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to the waitlist");
        }

        pub fn seat_at_table() {
            println!("Seated");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("Order taken");
        }
    }
}

fn main() {
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
    front_of_house::serving::take_order();
}
```

You call a function inside a module by its **path**: module names separated by `::`. The `pub` keyword makes an item visible outside its module. You'll see exactly how privacy works in the next two concepts; for now, put `pub` on anything `main` needs to reach.

## The module tree

Modules nest, so a crate's modules form a **tree**, with the crate root at the top as an implicit module called `crate`:

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         └── take_order
```

It works just like folders on your computer: modules are folders, items are files, and a path is how you find one.

## A common mistake

Items inside a module are **private by default**. Leave out `pub`, and code outside the module can't call the function:

```rust,does_not_compile title="Calling a private function"
mod kitchen {
    fn cook() {
        println!("cooking");
    }
}

fn main() {
    kitchen::cook();
}
```

The error, `E0603: function cook is private`, even points at where to add `pub`.
