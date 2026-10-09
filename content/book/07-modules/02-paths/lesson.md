+++
id = "book.modules.paths"
chapter = "book.modules"
requires = []
title = "Paths and Privacy"
track = "book"
order = 2
source = "https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html"
summary = "Absolute and relative paths, super, and the rule for what code can see what."
+++

# Paths and Privacy

To use an item in the module tree, you name it with a **path**. There are two kinds:

- An **absolute path** starts from the crate root with the word `crate`: `crate::front_of_house::hosting::add_to_waitlist()`.
- A **relative path** starts from the current module: `front_of_house::hosting::add_to_waitlist()`.

```rust,runnable title="Two ways to name the same function"
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("added");
        }
    }
}

fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

fn main() {
    eat_at_restaurant();
}
```

Which to choose? Absolute paths keep working if you move the *calling* code somewhere else; relative paths keep working if you move the caller *and* the item together. Many Rust programmers default to absolute paths.

## The privacy rule

Everything is **private** unless marked `pub`, and the rule for who can see a private item is simple:

> Code can see private items in **its own module and every module above it** (its ancestors), but not inside the modules below it (its children).

Think of a restaurant. The kitchen staff can see the dining room, but customers can't see into the kitchen. A child module is the kitchen: it can use everything in its parent, but the parent can only use what the child marks `pub`.

```rust,editable title="A child can see its parent's private items"
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // `super` means the parent module, here the crate root.
        super::deliver_order();
    }

    fn cook_order() {
        println!("cooking");
    }

    pub fn handle_complaint() {
        fix_incorrect_order();
    }
}

fn deliver_order() {
    println!("delivered");
}

fn main() {
    back_of_house::handle_complaint();
}
```

`deliver_order` is private, but `back_of_house` is its child, so it can call it. To reach up one level, start the path with `super`, like `..` in a file path.

## `pub mod` isn't enough on its own

Making a module public only lets other code **refer to the module**. Its contents stay private until each item is marked `pub` too:

```rust,does_not_compile title="A public module with a private function"
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

fn main() {
    front_of_house::hosting::add_to_waitlist();
}
```

The error says `add_to_waitlist` is private. Every step of the path has to be visible to the caller.
