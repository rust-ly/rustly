+++
id = "book.modules.use"
chapter = "book.modules"
requires = []
title = "Bringing Paths into Scope with use"
track = "book"
order = 4
source = "https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html"
summary = "Shorten long paths with use, rename with as, and group imports."
+++

# Bringing Paths into Scope with `use`

Writing `crate::front_of_house::hosting::add_to_waitlist()` every time gets old fast. The `use` keyword creates a shortcut: after `use crate::front_of_house::hosting;`, the name `hosting` works on its own in that scope.

```rust,runnable title="A shortcut with use"
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("added");
        }
    }
}

use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

A `use` only applies in the scope where it's written. If you move a function into a child module, it needs its own `use` (or `super::hosting`).

## Idiomatic `use`

Rust code follows two conventions:

- For **functions**, bring the **parent module** into scope and call `hosting::add_to_waitlist()`. Seeing the module name makes it clear the function isn't defined locally.
- For **structs, enums and other types**, bring in the **full path** and use the type's name directly, like `use std::collections::HashMap;` and then `HashMap::new()`.

```rust,editable title="Types by their full path"
use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::new();
    ages.insert("Ferris", 9);
    println!("{ages:?}");
}
```

## Renaming with `as`

Two items with the same name can't both be brought in directly. Either keep their parent modules, or rename one with `as`:

```rust,runnable title="Two Results, one renamed"
use std::fmt::Result;
use std::io::Result as IoResult;

fn format_ok() -> Result {
    Ok(())
}

fn io_ok() -> IoResult<()> {
    Ok(())
}

fn main() {
    println!("{:?} {:?}", format_ok(), io_ok());
}
```

## Grouping imports

Paths that share a beginning can be combined with braces, and `self` refers to the shared part itself:

```rust
use std::collections::{HashMap, HashSet};
use std::io::{self, Write}; // brings in `io` and `io::Write`
use std::collections::*;   // the glob: everything public. Use sparingly.
```

The glob operator `*` makes it hard to tell where a name came from, so it's mostly used in tests (you've seen `use super::*;` in every hidden test module) and in "prelude" modules.
